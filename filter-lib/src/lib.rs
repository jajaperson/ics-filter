use std::{collections::HashMap, str::Lines};

use querystring::querify;
use urlencoding::decode;

pub fn filter_from_query(query: &str) -> String {
    let params: HashMap<String, String> = querify(query)
        .into_iter()
        .map(|(k, v)| (k.to_owned(), decode(v).unwrap().into_owned())) // bad unwrap
        .collect();

    let ics_url = params.get("ics_url").unwrap().trim();
    let summary_match = params.get("summary_match").unwrap();

    let source = {
        let mut response = chttp::get(ics_url).expect("valid url");
        let body = response.body_mut().text().unwrap();

        body
    };

    let result = filter_events_by_name(&source, &summary_match);

    result
}

pub fn filter_events_by_name(ical: &str, substr: &str) -> String {
    let mut result_lines: Vec<String> = Vec::new();

    let mut source_lines = ical.lines();

    loop {
        match source_lines.next() {
            Some(l) => {
                if l.starts_with("BEGIN:VEVENT") {
                    result_lines.append(&mut filter_event(
                        &mut source_lines,
                        vec![l.into()],
                        substr,
                    ))
                } else {
                    result_lines.push(l.into())
                }
            }
            None => break,
        }
    }

    result_lines.join("\n")
}

fn filter_event(feed: &mut Lines, mut buffer: Vec<String>, substr: &str) -> Vec<String> {
    let mut is_match = false;

    loop {
        match feed.next() {
            Some(l) => {
                if l.starts_with("END:VEVENT") {
                    buffer.push(l.into());
                    break;
                } else if l.starts_with("SUMMARY:") {
                    is_match = l.contains(substr);
                    buffer.push(l.into());
                } else {
                    buffer.push(l.into());
                }
            }
            None => break,
        }
    }

    if is_match {
        buffer
    } else {
        vec![]
    }
}
