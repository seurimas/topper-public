use regex::Regex;
use topper_core::colored_lines::get_content_of_raw_colored_text;

use crate::explainer::{ExplainerPage, PROMPT_REGEX};

lazy_static! {
    pub static ref WHO_REGEX: Regex = Regex::new(r"^Who:\s+(?P<who>\w+)$").unwrap();
    pub static ref VS_REGEX: Regex = Regex::new(r"^Vs:\s+(?P<vs>\w+)$").unwrap();
    pub static ref CLASS_REGEX: Regex = Regex::new(r"^Class:\s+(?P<class>\w+)$").unwrap();
}

pub fn parse_me_and_you(page: &ExplainerPage) -> (String, String) {
    let mut me = "Unknown".to_string();
    let mut you = "Assailant".to_string();
    for line_text in page.get_body().iter() {
        let text = get_content_of_raw_colored_text(line_text);
        if let Some(captures) = WHO_REGEX.captures(&text) {
            if let Some(who) = captures.name("who") {
                me = who.as_str().to_string();
            }
        } else if let Some(captures) = VS_REGEX.captures(&text) {
            if let Some(vs) = captures.name("vs") {
                you = vs.as_str().to_string();
            }
        } else if let Some(matches) = PROMPT_REGEX.find(&text) {
            break;
        }
    }
    (me, you)
}
