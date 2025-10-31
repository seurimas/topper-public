use topper_aetolia::explainer::{PROMPT_REGEX, VS_REGEX, WHO_REGEX};
use wasm_bindgen::prelude::*;

#[derive(Debug)]
#[wasm_bindgen]
pub struct ExplainerPageBuilder {
    last_color: String,
    lines: Vec<String>,
    line_remaining: String,
    time: String,
    me: String,
    you: String,
}

#[wasm_bindgen]
impl ExplainerPageBuilder {
    pub fn new() -> Self {
        Self {
            last_color: String::new(),
            lines: Vec::new(),
            line_remaining: String::new(),
            time: String::new(),
            me: String::new(),
            you: String::new(),
        }
    }

    pub fn parse_matchup(&mut self, text: &str) {
        if self.time.is_empty() {
            if let Some(captures) = WHO_REGEX.captures(&text) {
                if let Some(who) = captures.name("who") {
                    self.me = who.as_str().to_string();
                }
            } else if let Some(captures) = VS_REGEX.captures(&text) {
                if let Some(vs) = captures.name("vs") {
                    self.you = vs.as_str().to_string();
                }
            } else if let Some(matches) = PROMPT_REGEX.find(&text) {
                self.time = matches.as_str().to_string();
            }
        }
    }

    pub fn get_title(&self) -> String {
        format!("{} vs {} ({})", self.me, self.you, self.time)
    }

    pub fn get_lines(&self) -> Vec<String> {
        self.lines.clone()
    }

    pub fn append_colored_text(&mut self, mut text: String, color: String) {
        if !self.last_color.eq(&color) {
            self.line_remaining = format!("{}<{}>", self.line_remaining, color);
            self.last_color = color.clone();
        }
        while let Some((end_old, start_new)) = text.split_once("\n") {
            self.lines
                .push(format!("{}{}", self.line_remaining, end_old));
            self.line_remaining = format!("<{}>", color);
            text = start_new.to_string();
        }
        self.line_remaining = format!("{}{}", self.line_remaining, text);
    }
}
