// Fix the borrow.
//
// This file does NOT compile. `latency_report` hands ownership of `latencies`
// to `max_latency`, then tries to use the moved value again on the next line.
// The borrow checker rejects it: two owners would make cleanup and mutation
// ambiguous.
//
// Your job: make BOTH helpers borrow a slice (`&[u64]`) instead of taking
// ownership of the `Vec`, and pass `&latencies` at the call sites.
// Do NOT clone the vector, that compiles, but it dodges the lesson.
//
// Contract (do not change the signature of `latency_report`):
//   latency_report(vec![120, 340, 930], 500) -> "max=930,over=1"
//   max  = the largest latency in the list (0 for an empty list)
//   over = how many latencies are STRICTLY greater than threshold_ms

fn max_latency(latencies: Vec<u64>) -> u64 {
    let mut max = 0;
    for l in latencies {
        if l > max {
            max = l;
        }
    }
    max
}

fn count_over(latencies: Vec<u64>, threshold_ms: u64) -> u64 {
    let mut n = 0;
    for l in latencies {
        if l > threshold_ms {
            n += 1;
        }
    }
    n
}

fn latency_report(latencies: Vec<u64>, threshold_ms: u64) -> String {
    let max = max_latency(latencies);
    // ERROR: `latencies` was moved into `max_latency` on the line above.
    let over = count_over(latencies, threshold_ms);
    format!("max={},over={}", max, over)
}
