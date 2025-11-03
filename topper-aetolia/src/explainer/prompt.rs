use regex::Regex;

lazy_static! {
    pub static ref PROMPT_REGEX: Regex =
        Regex::new(r"\[(?P<hour>\d\d):(?P<minute>\d\d):(?P<second>\d\d):(?P<centi>\d\d)\]")
            .unwrap();
}

pub fn parse_prompt_time(line: &String, last_time: i32) -> Option<i32> {
    if let Some(captures) = PROMPT_REGEX.captures(line.as_ref()) {
        if let (Some(hour), Some(minute), Some(second), Some(centi)) = (
            captures.name("hour"),
            captures.name("minute"),
            captures.name("second"),
            captures.name("centi"),
        ) {
            let hour: i32 = hour.as_str().parse().unwrap();
            let minute: i32 = minute.as_str().parse().unwrap();
            let second: i32 = second.as_str().parse().unwrap();
            let centi: i32 = centi.as_str().parse().unwrap();
            let mut time = centi + (((((hour * 60) + minute) * 60) + second) * 100);
            if time < last_time {
                // It's a braaand neww day, and the sun is hiiigh.
                time = time + (24 * 360000);
            }
            return Some(time);
        }
    }
    None
}

pub fn format_prompt_time(time: i32) -> String {
    let centi = time % 100;
    let total_seconds = time / 100;
    let seconds = total_seconds % 60;
    let total_minutes = total_seconds / 60;
    let minutes = total_minutes % 60;
    let hours = total_minutes / 60;
    format!("[{:02}:{:02}:{:02}:{:02}]", hours, minutes, seconds, centi)
}

pub fn replace_prompt_time(line: &String, new_time: i32) -> String {
    PROMPT_REGEX
        .replace(line, &format_prompt_time(new_time))
        .to_string()
}

pub fn is_prompt(line: &String) -> bool {
    PROMPT_REGEX.is_match(line.as_ref())
}
