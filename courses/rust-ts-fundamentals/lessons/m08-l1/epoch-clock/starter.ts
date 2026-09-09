// Pulse Station, epoch clock (m08-l1)
//
// The chain gauge you just built measures slot time; this helper turns a raw
// slot number into something a human reads on the dashboard: which epoch we
// are in, and how long until it rolls over.
//
// Contract:
//   - An epoch is EXACTLY 432,000 slots. Always. Slot time changes; epoch
//     length in slots does not, which is why faster slots shrink epochs in
//     wall-clock time.
//   - epoch        = the zero-based epoch this slot sits in
//   - slotsLeft    = slots remaining until the NEXT epoch boundary
//                    (at an exact boundary, a full 432,000 remain)
//   - hours        = slotsLeft * slotMs, expressed in hours, 1 decimal place
//   - Return exactly:  `epoch ${epoch}: ${slotsLeft} slots / ${hours}h remaining`
//     e.g. "epoch 1023: 432000 slots / 36.0h remaining"
//   - number, not bigint: unlike lamports, slot and slot-ms magnitudes here stay far under 2^53.
//
// The version below is the classic first draft: it ROUNDS the epoch instead
// of flooring it, reports slots ELAPSED instead of remaining, and forgets
// that slotMs is milliseconds when converting to hours. Fix all three.
// Keep epochClock the FIRST function in the file, the grader calls it.

function epochClock(slot: number, slotMs: number): string {
  const EPOCH_SLOTS = 432000;
  // BUG 1: round() drags a mid-epoch slot into the wrong epoch.
  const epoch = Math.round(slot / EPOCH_SLOTS);
  // BUG 2: this is how far we ARE, not how far we have LEFT.
  const slotsLeft = slot % EPOCH_SLOTS;
  // BUG 3: slotMs is milliseconds, dividing by 3600 leaves ms in the number.
  const hours = (slotsLeft * slotMs) / 3600;
  return `epoch ${epoch}: ${slotsLeft} slots / ${hours.toFixed(1)}h remaining`;
}
