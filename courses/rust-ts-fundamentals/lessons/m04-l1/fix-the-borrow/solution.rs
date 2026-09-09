// Solution: the helpers BORROW a slice; the caller lends `&latencies` twice.
// Ownership never moves, so `latency_report` can keep using its Vec after
// both calls, no clone, no fight with the borrow checker.

fn max_latency(latencies: &[u64]) -> u64 {
    let mut max = 0;
    for &l in latencies {
        if l > max {
            max = l;
        }
    }
    max
}

fn count_over(latencies: &[u64], threshold_ms: u64) -> u64 {
    let mut n = 0;
    for &l in latencies {
        if l > threshold_ms {
            n += 1;
        }
    }
    n
}

fn latency_report(latencies: Vec<u64>, threshold_ms: u64) -> String {
    let max = max_latency(&latencies);
    let over = count_over(&latencies, threshold_ms);
    format!("max={},over={}", max, over)
}
