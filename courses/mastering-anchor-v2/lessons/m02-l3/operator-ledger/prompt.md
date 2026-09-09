# The operator ledger — the challenge prompt

One account per operator, touched on **every payout** — a hot account. It holds:

- `payout_bps` — the operator's cut, in basis points,
- `machines` — the addresses of the cabinets this operator runs, with no ceiling
  written into the spec,
- `settings` — a `HashMap` of per-machine configuration keys to values,
- `support_note` — a free-form note the operator types for the floor staff.

Label each field **Pod** or **BorshAccount**, give exactly **one reason** per
choice, then say which of the borsh fields, if any, needs the wire-hole check
and why. Four labels, four reasons, one wire-hole sentence, no more.

Calibration (a field that is *not* on the list): "the cabinet's `machine_key`
is Pod because `[u8; 32]` has a length fixed at compile time." One clause of
fact, one clause of reason. If a reason runs longer than that, you are probably
justifying the wrong tier.

Two of the four are the point. `machines` is the field the flowchart makes you
interrogate rather than answer. And because this ledger is hot, whichever
fields land in tier 2 you should also say whether they belong in *this*
account at all, the way the lab split the description out.

Write your four labels down **before** you open `reference.md`.
