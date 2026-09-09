// Finish the match: the ProbeState machine.
//
// This starter COMPILES but the machine is wrong, `Degraded` is missing
// from it entirely, and unknown state strings leak through as "Pending".
//
// The legal machine (the lesson's canon):
//   Pending  + ok            -> Up
//   Pending  + fail          -> Down
//   Up       + ok            -> Up
//   Up       + fail          -> Degraded      (one failure degrades, it does not kill)
//   Degraded + ok            -> Up
//   Degraded + fail, consecutive_failures >= 3 -> Down
//   Degraded + fail, consecutive_failures <  3 -> Degraded
//   Down     + ok            -> Up            (recovery)
//   Down     + fail          -> Down
//   any unknown state string -> "Invalid"
//
// Your job: model the states as a proper `enum ProbeState`, parse the incoming
// string into it (unknown -> return "Invalid"), and write next_state as a
// match over (state, probe_ok), let the COMPILER tell you which arms you
// forgot. Return the next state's name exactly: "Pending", "Up", "Degraded",
// "Down", or "Invalid".

fn next_state(current: &str, probe_ok: bool, consecutive_failures: u32) -> String {
    let _ = consecutive_failures; // TODO: the Degraded ladder needs this
    let next = match current {
        "Pending" => {
            if probe_ok {
                "Up"
            } else {
                "Down"
            }
        }
        "Up" => {
            if probe_ok {
                "Up"
            } else {
                "Down" // TODO: one failure should degrade, not kill
            }
        }
        "Down" => {
            if probe_ok {
                "Up"
            } else {
                "Down"
            }
        }
        _ => "Pending", // TODO: unknown states must not leak through
    };
    next.to_string()
}
