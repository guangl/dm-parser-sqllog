// ...existing code...

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedRecord<'a> {
    pub ts: &'a str,
    pub meta_raw: &'a str,
    pub ep: Option<&'a str>,
    pub sess: Option<&'a str>,
    pub thrd: Option<&'a str>,
    pub user: Option<&'a str>,
    pub trxid: Option<&'a str>,
    pub stmt: Option<&'a str>,
    pub appname: Option<&'a str>,
    pub ip: Option<&'a str>,
    pub body: &'a str,
    pub execute_time_ms: Option<u64>,
    pub row_count: Option<u64>,
    pub execute_id: Option<u64>,
}

/// Iterator that yields record slices (&str) from an input log text without allocating.
pub struct RecordSplitter<'a> {
    text: &'a str,
    bytes: &'a [u8],
    n: usize,
    // scanning position: always non-decreasing
    scan_pos: usize,
    // the start index of the next record to yield
    next_start: Option<usize>,
    // whether we've yielded the final record already
    finished: bool,
    // cached prefix (leading errors) end index
    first_start: Option<usize>,
}

impl<'a> RecordSplitter<'a> {
    pub fn new(text: &'a str) -> Self {
        let bytes = text.as_bytes();
        let n = text.len();
        let mut first_start = None;
        if n >= 23 {
            let limit = n.saturating_sub(23);
            let mut pos = 0usize;
            while pos <= limit {
                if (pos == 0 || bytes[pos - 1] == b'\n')
                    && crate::tools::is_ts_millis_bytes(&bytes[pos..pos + 23])
                {
                    first_start = Some(pos);
                    break;
                }
                pos += 1;
            }
        }
        let scan_pos = first_start.unwrap_or(0).saturating_add(1);
        RecordSplitter {
            text,
            bytes,
            n,
            scan_pos,
            next_start: first_start,
            finished: false,
            first_start,
        }
    }

    /// Return the full slice of leading error text (everything before the first record)
    pub fn leading_errors_slice(&self) -> Option<&'a str> {
        self.first_start.map(|s| &self.text[..s])
    }
}

impl<'a> Iterator for RecordSplitter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let start = match self.next_start {
            Some(s) => s,
            None => {
                self.finished = true;
                return None;
            }
        };

        // scan for the next record's start
        if self.scan_pos > self.n {
            // no room for another ts, yield remainder
            self.finished = true;
            return Some(&self.text[start..self.n]);
        }
        let limit = self.n.saturating_sub(23);
        let mut pos = self.scan_pos;
        while pos <= limit {
            if (pos == 0 || self.bytes[pos - 1] == b'\n')
                && crate::tools::is_ts_millis_bytes(&self.bytes[pos..pos + 23])
            {
                // found next start
                let end = pos;
                // prepare for next call
                self.next_start = Some(pos);
                self.scan_pos = pos + 1;
                return Some(&self.text[start..end]);
            }
            pos += 1;
        }

        // no next start => yield final record
        self.finished = true;
        Some(&self.text[start..self.n])
    }
}

/// Split a full log text into records using timestamp-detection.
/// Returns (records, leading_errors). Each record is a borrowed slice from `text`.
pub fn split_by_ts_records_with_errors<'a>(text: &'a str) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut records: Vec<&'a str> = Vec::new();
    let mut errors: Vec<&'a str> = Vec::new();

    let splitter = RecordSplitter::new(text);
    if let Some(prefix) = splitter.leading_errors_slice() {
        for line in prefix.lines() {
            errors.push(line);
        }
    }
    for rec in splitter {
        records.push(rec);
    }
    (records, errors)
}

/// Split into caller-provided containers to avoid per-call allocations.
///
/// This function clears and fills `records` and `errors`. If callers reuse these
/// vectors across repeated calls (e.g. in a loop), they can avoid allocating a
/// fresh `Vec` on each call.
pub fn split_into<'a>(text: &'a str, records: &mut Vec<&'a str>, errors: &mut Vec<&'a str>) {
    records.clear();
    errors.clear();

    let splitter = RecordSplitter::new(text);
    if let Some(prefix) = splitter.leading_errors_slice() {
        for line in prefix.lines() {
            errors.push(line);
        }
    }
    for rec in splitter {
        records.push(rec);
    }
}

/// Stream over records and invoke the callback for each record without
/// allocating a Vec. This is the lowest-allocation way to process a log text.
pub fn for_each_record<F>(text: &str, mut f: F)
where
    F: FnMut(&str),
{
    let splitter = RecordSplitter::new(text);
    // ignore leading errors for the streaming API; callers can inspect them via
    // RecordSplitter::leading_errors_slice if they need to.
    if let Some(_prefix) = splitter.leading_errors_slice() {
        // drop the prefix borrow before iterating
    }
    for rec in splitter {
        f(rec);
    }
}

/// Parse each record and invoke callback with ParsedRecord; zero-allocation
/// when used together with streaming Splitter.
pub fn parse_records_with<F>(text: &str, mut f: F)
where
    F: for<'r> FnMut(ParsedRecord<'r>),
{
    for_each_record(text, |rec| {
        let parsed = parse_record(rec);
        f(parsed);
    });
}

/// Parse into a caller-provided Vec to avoid allocating a new Vec on each call.
pub fn parse_into<'a>(text: &'a str, out: &mut Vec<ParsedRecord<'a>>) {
    out.clear();
    let splitter = RecordSplitter::new(text);
    for rec in splitter {
        out.push(parse_record(rec));
    }
}

/// Parse all records sequentially and return a Vec of ParsedRecord.
pub fn parse_all(text: &str) -> Vec<ParsedRecord<'_>> {
    let splitter = RecordSplitter::new(text);
    splitter.map(|r| parse_record(r)).collect()
}

fn parse_digits_forward(s: &str, mut i: usize) -> Option<(u64, usize)> {
    let bytes = s.as_bytes();
    let n = bytes.len();
    // skip non-digits
    while i < n && !bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i >= n || !bytes[i].is_ascii_digit() {
        return None;
    }
    let mut val: u64 = 0;
    while i < n && bytes[i].is_ascii_digit() {
        val = val
            .saturating_mul(10)
            .saturating_add((bytes[i] - b'0') as u64);
        i += 1;
    }
    Some((val, i))
}

/// Parse a single record (as produced by split_by_ts_records_with_errors)
/// Returns a ParsedRecord borrowing from the input `rec`.
pub fn parse_record<'a>(rec: &'a str) -> ParsedRecord<'a> {
    let ts: &'a str = if rec.len() >= 23 { &rec[..23] } else { "" };

    // find first '(' after timestamp, then the matching ')'
    let after_ts: &'a str = if rec.len() > 23 { &rec[23..] } else { "" };
    let mut meta_raw: &'a str = "";
    let mut body: &'a str = "";

    if let Some(open_idx) = after_ts.find('(') {
        if let Some(close_rel) = after_ts[open_idx..].find(')') {
            meta_raw = &after_ts[open_idx + 1..open_idx + close_rel];
            // body starts after the closing ')' character
            let body_start = 23 + open_idx + close_rel + 1;
            if body_start < rec.len() {
                body = rec[body_start..].trim_start();
            }
        } else {
            // no closing paren: treat rest as body
            body = after_ts;
        }
    } else {
        // no meta parentheses: everything after ts is body
        body = after_ts;
    }

    // parse meta tokens (borrowed slices)
    let mut ep: Option<&'a str> = None;
    let mut sess: Option<&'a str> = None;
    let mut thrd: Option<&'a str> = None;
    let mut user: Option<&'a str> = None;
    let mut trxid: Option<&'a str> = None;
    let mut stmt: Option<&'a str> = None;
    let mut appname: Option<&'a str> = None;
    let mut ip: Option<&'a str> = None;

    let mut iter = meta_raw.split_whitespace().peekable();
    while let Some(tok) = iter.next() {
        if tok.starts_with("EP[") {
            ep = Some(tok);
        } else if tok.starts_with("sess:") {
            sess = Some(&tok[5..]);
        } else if tok.starts_with("thrd:") {
            thrd = Some(&tok[5..]);
        } else if tok.starts_with("user:") {
            user = Some(&tok[5..]);
        } else if tok.starts_with("trxid:") {
            trxid = Some(&tok[6..]);
        } else if tok.starts_with("stmt:") {
            stmt = Some(&tok[5..]);
        } else if tok == "appname:" {
            // next token might be ip:::... or the appname value
            if let Some(next) = iter.peek() {
                if (*next).starts_with("ip:::") {
                    // consume next and extract ip
                    let nexttok = iter.next().unwrap();
                    let ippart = nexttok.trim_start_matches("ip:::");
                    let ipclean = ippart.trim_start_matches("ffff:");
                    ip = Some(ipclean);
                    appname = Some("");
                } else {
                    // take next as appname value
                    let val = iter.next().unwrap();
                    appname = Some(val);
                }
            } else {
                appname = Some("");
            }
        } else if tok.starts_with("appname:") {
            let val = &tok[8..];
            if val.starts_with("ip:::") {
                let ippart = val.trim_start_matches("ip:::");
                let ipclean = ippart.trim_start_matches("ffff:");
                ip = Some(ipclean);
                appname = Some("");
            } else {
                appname = Some(val);
            }
        }
    }

    // parse numeric metrics from body from tail -> head: EXEC_ID -> ROWCOUNT -> EXECTIME
    let mut execute_id: Option<u64> = None;
    let mut row_count: Option<u64> = None;
    let mut execute_time_ms: Option<u64> = None;

    let body_str = body;
    let mut search_end = body_str.len();

    if let Some(pos) = body_str[..search_end].rfind("EXEC_ID:") {
        let start = pos + "EXEC_ID:".len();
        if let Some((v, _end)) = parse_digits_forward(body_str, start) {
            execute_id = Some(v);
        }
        search_end = pos;
    }

    if let Some(pos) = body_str[..search_end].rfind("ROWCOUNT:") {
        let start = pos + "ROWCOUNT:".len();
        if let Some((v, _end)) = parse_digits_forward(body_str, start) {
            row_count = Some(v);
        }
        search_end = pos;
    }

    if let Some(pos) = body_str[..search_end].rfind("EXECTIME:") {
        let start = pos + "EXECTIME:".len();
        if let Some((v, _end)) = parse_digits_forward(body_str, start) {
            execute_time_ms = Some(v);
        }
    }

    ParsedRecord {
        ts,
        meta_raw,
        ep,
        sess,
        thrd,
        user,
        trxid,
        stmt,
        appname,
        ip,
        body,
        execute_time_ms,
        row_count,
        execute_id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_by_ts_records() {
        let log_text = "2023-10-05 14:23:45.123 (EP[12345] sess:1 thrd:1 user:admin trxid:0 stmt:1 appname:MyApp)\nSELECT * FROM users
2023-10-05 14:24:00.456 (EP[12346] sess:2 thrd:2 user:guest trxid:0 stmt:2 appname:MyApp)\nINSERT INTO orders VALUES (1, 'item');\n";
        let (records, errors) = split_by_ts_records_with_errors(log_text);

        assert_eq!(records.len(), 2);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_split_with_leading_errors() {
        let log_text = "garbage line 1\ngarbage line 2\n2023-10-05 14:23:45.123 (EP[12345] sess:1 thrd:1 user:admin trxid:0 stmt:1 appname:MyApp)\nSELECT 1\n";
        let (records, errors) = split_by_ts_records_with_errors(log_text);

        assert_eq!(records.len(), 1);
        assert_eq!(errors.len(), 2);
        assert!(records[0].contains("SELECT 1"));
    }

    #[test]
    fn test_record_splitter_iterator() {
        let log_text =
            "garbage\n2023-10-05 14:23:45.123 (EP[1]) foo\n2023-10-05 14:23:46.456 (EP[2]) bar\n";
        let it = RecordSplitter::new(log_text);
        assert_eq!(it.leading_errors_slice().unwrap().trim(), "garbage");
        let v: Vec<&str> = it.collect();
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn test_parse_simple_log_sample() {
        let log_text = "2025-08-12 10:57:09.562 (EP[0] sess:0x7fb24f392a30 thrd:757794 user:HBTCOMS_V3_PROD trxid:688489653 stmt:0x7fb236077b70 appname: ip:::ffff:10.3.100.68) EXECTIME: 0ms ROWCOUNT: 1 EXEC_ID: 289655185\n2025-08-12 10:57:09.562 (EP[0] sess:0x7fb24f392a30 thrd:757794 user:HBTCOMS_V3_PROD trxid:0 stmt:NULL appname:) TRX: START\n";

        let (records, errors) = split_by_ts_records_with_errors(log_text);
        assert_eq!(errors.len(), 0);
        assert_eq!(records.len(), 2);

        let r0 = parse_record(records[0]);
        assert_eq!(r0.execute_time_ms, Some(0));
        assert_eq!(r0.row_count, Some(1));
        assert_eq!(r0.execute_id, Some(289655185));
        assert_eq!(r0.ip, Some("10.3.100.68"));
        assert_eq!(r0.appname, Some(""));

        let r1 = parse_record(records[1]);
        assert!(r1.body.contains("TRX: START"));
    }
}
