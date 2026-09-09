// kv-config-parser, reference solution
//
// Hand-roll the parser serde is about to delete. One probe-target line looks like:
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

fn parse_kv(line: &str) -> Result<(String, String, u64), String> {
    let mut name: Option<String> = None;
    let mut url: Option<String> = None;
    let mut timeout_ms: Option<u64> = None;

    for pair in line.split(';') {
        if pair.is_empty() {
            continue;
        }
        let (key, value) = match pair.split_once('=') {
            Some((k, v)) => (k, v),
            None => return Err(format!("bad pair: {pair}")),
        };
        match key {
            "name" => name = Some(value.to_string()),
            "url" => {
                if !(value.starts_with("http://") || value.starts_with("https://")) {
                    return Err(format!("invalid url: {value}"));
                }
                url = Some(value.to_string());
            }
            "timeout_ms" => {
                let parsed = value
                    .parse::<u64>()
                    .map_err(|_| format!("invalid timeout_ms: {value}"))?;
                timeout_ms = Some(parsed);
            }
            other => return Err(format!("unknown key: {other}")),
        }
    }

    let name = name.ok_or_else(|| "missing key: name".to_string())?;
    let url = url.ok_or_else(|| "missing key: url".to_string())?;
    let timeout_ms = timeout_ms.ok_or_else(|| "missing key: timeout_ms".to_string())?;
    Ok((name, url, timeout_ms))
}

fn parse_target(line: &str) -> String {
    match parse_kv(line) {
        Ok((name, url, timeout_ms)) => format!("OK {name} {url} {timeout_ms}"),
        Err(e) => format!("ERR {e}"),
    }
}
