// Solution: states are an enum, transitions are one exhaustive match over
// (state, probe_ok). There is no arm that produces an illegal transition, 
// the type plus the match ARE the rulebook, and a future new variant breaks
// the build at exactly the arms that must learn about it.

#[derive(Debug, Clone, Copy, PartialEq)]
enum ProbeState {
    Pending,
    Up,
    Degraded,
    Down,
}

fn parse_state(s: &str) -> Option<ProbeState> {
    match s {
        "Pending" => Some(ProbeState::Pending),
        "Up" => Some(ProbeState::Up),
        "Degraded" => Some(ProbeState::Degraded),
        "Down" => Some(ProbeState::Down),
        _ => None,
    }
}

fn next_state(current: &str, probe_ok: bool, consecutive_failures: u32) -> String {
    let state = match parse_state(current) {
        Some(state) => state,
        None => return "Invalid".to_string(),
    };
    let next = match (state, probe_ok) {
        (ProbeState::Pending, true) => ProbeState::Up,
        (ProbeState::Pending, false) => ProbeState::Down,
        (ProbeState::Up, true) => ProbeState::Up,
        (ProbeState::Up, false) => ProbeState::Degraded,
        (ProbeState::Degraded, true) => ProbeState::Up,
        (ProbeState::Degraded, false) if consecutive_failures >= 3 => ProbeState::Down,
        (ProbeState::Degraded, false) => ProbeState::Degraded,
        (ProbeState::Down, true) => ProbeState::Up,
        (ProbeState::Down, false) => ProbeState::Down,
    };
    format!("{:?}", next)
}
