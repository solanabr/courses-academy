# The operator ledger — my reference labeling

Your four labels go on paper before you read mine. That was the deal.

## The four labels

- **`payout_bps` — Pod**, because a basis-point cut is one fixed-width integer
  (`PodU16`) whose size never moves.
- **`machines` — Pod**, as a capped `PodVec<Address, MAX_MACHINES>`, because "no
  ceiling written into the spec" is a spec that forgot its ceiling, not a field
  without one: an operator runs a physical floor, and a floor holds a countable
  number of cabinets.
- **`settings` — BorshAccount**, because a `HashMap`'s keys and entry count have
  no compile-time bound, so there are no fixed bytes to cast.
- **`support_note` — BorshAccount**, because a free-form note is the genuinely
  unbounded case: arbitrary length is the feature, not an oversight.

## The wire-hole sentence

`settings` needs the check: it is a `HashMap`, whose iteration order is the
first wincode-vs-borsh hole, so a borsh-decoding client can read the same
logical state in a different entry order — never hash or sign over that
account's raw bytes. (`support_note` is a single `String`: no map, no set, no
float, so the compatibility holds and no check is owed.)

## The hot-account split

This ledger is touched on every payout, so the two borsh fields do not belong
in it at all. Keep the hot account Pod — `payout_bps` and `machines`, read on
every payout with a cast — and move `settings` and `support_note` into a
separate `OperatorNotes`-style `BorshAccount` that only its own edit handlers
ever deserialize. That is the lab's description split, replayed: the deserialize
tax is proportional to the whole account's size and paid on every access, so
you scope it to the account nobody hot-paths.

## Where you might disagree with me

The honest disagreement is `machines`, and it is a Q2 disagreement, not a
Q1 one. We both answered Q1 the same way — the field is not fixed-size. Q2 is
"genuinely unbounded, or merely unspecified?", and I read the missing ceiling
as an oversight to push back on: cap it, stay zero-copy on the hot path, and
make the spec say the number out loud. If you read "no ceiling" as a real
requirement — operators can run unbounded fleets and the spec means it — then
your `machines` goes `BorshAccount`, and the hot-account rule immediately
demands you split it out of the payout path too. Neither label is wrong on its
own; what would be wrong is picking borsh without noticing you also just made
the hot account pay for it. Which question you answered differently is the
skill this challenge grades.
