// kv-config-parser, hand-roll the parser serde is about to delete.
//
// One probe-target line looks like:
//
//   name=api;url=https://example.com/health;timeout_ms=500
//
// Rules (the spec your tests grade):
//   - pairs are separated by ';'; empty segments (a trailing ';') are ignored
//   - each pair splits on the FIRST '=' only, URLs may contain '=' in a query string
//   - allowed keys: name, url, timeout_ms; any other key  -> Err("unknown key: <k>")
//   - a segment with no '='                               -> Err("bad pair: <segment>")
//   - url must start with http:// or https://             -> Err("invalid url: <v>")
//   - timeout_ms must parse as u64                        -> Err("invalid timeout_ms: <v>")
//   - all three keys are required; check in the fixed order name, url, timeout_ms
//                                                         -> Err("missing key: <k>")
//
// parse_kv returns Result, errors are values, exactly like m04-l2 taught.
// parse_target (the graded entry point, last in the file) renders the Result
// as "OK <name> <url> <timeout_ms>" or "ERR <message>".
//
// This starter COMPILES but the parser is a liar: it splits on EVERY '=',
// swallows unknown keys, invents defaults for missing/broken values, and
// never returns an Err. Fix it so every rule above comes back as a value.

fn parse_kv(line: &str) -> Result<(String, String, u64), String> {
    let mut name = String::new();
    let mut url = String::new();
    let mut timeout_ms: u64 = 0;

    for pair in line.split(';') {
        // TODO: this splits on EVERY '=', a URL with a query string gets truncated.
        // You want the FIRST '=' only (split_once).
        let parts: Vec<&str> = pair.split('=').collect();
        let key = parts[0];
        let value = parts.get(1).copied().unwrap_or("");
        match key {
            "name" => name = value.to_string(),
            // TODO: no url scheme check, ftp:// sails straight through.
            "url" => url = value.to_string(),
            // TODO: unwrap_or(0) turns a broken timeout into a silent default.
            "timeout_ms" => timeout_ms = value.parse::<u64>().unwrap_or(0),
            // TODO: unknown keys and '='-less segments should be Errs, not shrugs.
            _ => {}
        }
    }

    // TODO: nothing checks that all three keys actually appeared.
    Ok((name, url, timeout_ms))
}

fn parse_target(line: &str) -> String {
    match parse_kv(line) {
        Ok((name, url, timeout_ms)) => format!("OK {name} {url} {timeout_ms}"),
        Err(e) => format!("ERR {e}"),
    }
}
