#!/usr/bin/env bash
# Redeploys the m01-l1 twins to devnet and lands one reference increment per
# twin. Run this after a devnet reset (or when the reference signatures prune),
# then copy the four printed values into the lesson (sites listed at the end).
#
# Everything stateful happens in a scratch copy: keypairs and build artifacts
# NEVER land in this repo. Prerequisites: solana-cli (Agave 3.x), cargo-build-sbf,
# and a funded devnet wallet (the script makes one and asks you to fund it —
# ~1.2 SOL covers both deploys; airdrops or a transfer both work).
set -euo pipefail

RPC=https://api.devnet.solana.com
SRC="$(cd "$(dirname "$0")" && pwd)"
WORK="$(mktemp -d /tmp/anchor-v2-twins.XXXXXX)"
echo "workdir: $WORK (keys + builds live here, not in the repo)"
cp -R "$SRC/counter-v1-twin" "$SRC/counter-v2-twin" "$WORK/"

# 1. Deployer wallet (fresh). Fund it from another shell while this loop waits.
DEPLOYER="$WORK/deployer.json"
solana-keygen new --no-bip39-passphrase -s -o "$DEPLOYER" >/dev/null
DEP_PK=$(solana-keygen pubkey "$DEPLOYER")
echo "deployer: $DEP_PK"
echo "fund it now: solana airdrop 1 $DEP_PK --url devnet (repeat), or transfer ~1.2 SOL"
until [ "$(solana balance "$DEP_PK" --url devnet | awk '{print ($1 >= 1.1)}')" = "1" ]; do
  sleep 20
  echo "  waiting for >= 1.1 SOL... (current: $(solana balance "$DEP_PK" --url devnet))"
done

blockhash() {
  curl -s -X POST "$RPC" -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","id":1,"method":"getLatestBlockhash","params":[{"commitment":"finalized"}]}' \
    | python3 -c "import json,sys;print(json.load(sys.stdin)['result']['value']['blockhash'])"
}

send_b64() { # $1 = base64 tx; prints the signature, or hard-exits on an RPC error
  curl -s -X POST "$RPC" -H "Content-Type: application/json" \
    -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"sendTransaction\",\"params\":[\"$1\",{\"encoding\":\"base64\",\"preflightCommitment\":\"confirmed\"}]}" \
    | python3 -c '
import json, sys
r = json.load(sys.stdin)
sig = r.get("result")
if not sig:
    print("sendTransaction failed:", r, file=sys.stderr)
    sys.exit(1)
print(sig)'
}

deploy_twin() { # $1 = crate dir name, $2 = lib name
  local dir="$WORK/$1" lib="$2"
  cd "$dir"
  cargo build-sbf >/dev/null
  local pk
  pk=$(solana-keygen pubkey "target/deploy/${lib}-keypair.json")
  # Sync declare_id! to the fresh program keypair and rebuild so the binary
  # carries the id it will live at (the m01-l2 rule, done by hand).
  python3 - "$pk" <<'PY'
import re, sys
pk = sys.argv[1]
f = "src/lib.rs"
s = open(f).read()
s = re.sub(r'declare_id!\("[^"]+"\)', f'declare_id!("{pk}")', s)
open(f, "w").write(s)
PY
  cargo build-sbf >/dev/null
  solana program deploy "target/deploy/${lib}.so" \
    --program-id "target/deploy/${lib}-keypair.json" \
    -k "$DEPLOYER" --url devnet >/dev/null
  echo "$pk"
}

land() { # $1 = crate dir, $2 = mode, $3 = counter keypair; prints signature
  local dir="$WORK/$1"
  cd "$dir"
  local tx
  tx=$(cargo run -q --example land -- "$2" "$DEPLOYER" "$3" "$(blockhash)" 2>/dev/null)
  send_b64 "$tx"
}

echo "== v1 twin (anchor-lang 1.1.2) =="
V1_ID=$(deploy_twin counter-v1-twin counter_v1_twin)
solana-keygen new --no-bip39-passphrase -s -o "$WORK/counter-v1.json" >/dev/null
land counter-v1-twin init "$WORK/counter-v1.json" >/dev/null
sleep 20
V1_SIG=$(land counter-v1-twin increment "$WORK/counter-v1.json")

echo "== v2 twin (anchor-lang 2.0.0-rc.1) =="
V2_ID=$(deploy_twin counter-v2-twin counter_v2_twin)
solana-keygen new --no-bip39-passphrase -s -o "$WORK/counter-v2.json" >/dev/null
land counter-v2-twin init "$WORK/counter-v2.json" >/dev/null
sleep 20
V2_SIG=$(land counter-v2-twin increment "$WORK/counter-v2.json")

sleep 20
echo
echo "==== the four pins (paste into lessons/m01-l1/intro.md) ===="
echo "V1_TWIN_ID=$V1_ID"
echo "V2_TWIN_ID=$V2_ID"
echo "V1_TWIN_SIG=$V1_SIG"
echo "V2_TWIN_SIG=$V2_SIG"
echo
echo "==== the expected output blocks (Lab steps 2 and 4 print BOTH lines) ===="
solana confirm -v "$V1_SIG" --url devnet | grep -i "compute units"
solana confirm -v "$V2_SIG" --url devnet | grep -i "compute units"
echo
echo "Update sites: the opener export, Lab step 1's four exports, the expected"
echo "log lines in steps 2 and 4, the 'verified' dates on all three, and the"
echo "sample CU number in visual-src/m01-l1/v06-flowchart.html (re-render:"
echo "weasyprint v06-flowchart.html out.pdf && pdftoppm -png -r 144 -singlefile)."
echo "Then delete $WORK (it holds throwaway keys)."
