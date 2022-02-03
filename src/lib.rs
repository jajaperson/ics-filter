use std::str::Lines;

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
