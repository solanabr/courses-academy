// Pulse Station, epoch clock (m08-l1), reference solution

function epochClock(slot: number, slotMs: number): string {
  const EPOCH_SLOTS = 432000;
  const epoch = Math.floor(slot / EPOCH_SLOTS);
  const slotsLeft = EPOCH_SLOTS - (slot % EPOCH_SLOTS);
  const hours = (slotsLeft * slotMs) / 3600000;
  return `epoch ${epoch}: ${slotsLeft} slots / ${hours.toFixed(1)}h remaining`;
}
