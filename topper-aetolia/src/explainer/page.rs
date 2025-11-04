use serde::{Deserialize, Serialize};
use topper_core::{colored_lines::get_content_of_raw_colored_text, timeline::TimeSlice};

use crate::{
    explainer::{parse_me_and_you, parse_prompt_time, replace_prompt_time},
    timeline::{AetObservation, AetPrompt, AetTimeSlice},
};

use super::{Comment, Mutation};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ExplainerPage {
    pub id: String,
    pub body: Vec<String>,
    #[serde(default)]
    pub comments: Vec<Comment>,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub mutations: Vec<Mutation>,
}

impl PartialEq for ExplainerPage {
    fn eq(&self, other: &Self) -> bool {
        true
        // if !self.id.eq(&other.id) {
        //     false
        // } else if self.body.len() != other.body.len() {
        //     false
        // } else {
        //     self.comments.eq(&other.comments)
        // }
    }
}

impl ExplainerPage {
    pub fn new(id: String, body: Vec<String>) -> Self {
        Self {
            id,
            body,
            comments: Vec::new(),
            locked: false,
            mutations: Vec::new(),
        }
    }

    pub fn get_body(&self) -> &Vec<String> {
        &self.body
    }

    pub fn hide_real_times(&mut self) {
        let mut first_time = self
            .get_body()
            .iter()
            .find_map(|line| {
                let line = get_content_of_raw_colored_text(line);
                parse_prompt_time(&line, 0)
            })
            .unwrap_or(0);
        let mut last_time = first_time;
        self.body.iter_mut().for_each(|line| {
            let line_content = get_content_of_raw_colored_text(line);
            if let Some(line_time) = parse_prompt_time(&line_content, last_time) {
                let hidden_time = line_time - first_time;
                *line = replace_prompt_time(line, hidden_time);
                last_time = line_time;
            }
        });
    }

    pub fn get_duration(&self) -> Option<i32> {
        let start_time = self.get_body().iter().find_map(|line| {
            let line = get_content_of_raw_colored_text(line);
            parse_prompt_time(&line, 0)
        })?;
        let end_time = self.get_body().iter().rev().find_map(|line| {
            let line = get_content_of_raw_colored_text(line);
            parse_prompt_time(&line, start_time)
        })?;
        Some(end_time - start_time)
    }

    pub fn filter_out_from_body(&mut self, filter: &str) {
        self.body.retain(|line| !line.contains(filter))
    }

    pub fn filter_out_command(&mut self, command: &str) {
        let has_command =
            |line: &String| line.contains(&format!("<#ffff00>>>> <white>{}", command));
        let mut in_command = false;
        self.body.retain(|line| {
            if has_command(line) {
                in_command = true;
                false
            } else if in_command {
                if line.contains("<#ffff00>>>> <white>") {
                    in_command = false;
                    true
                } else {
                    false
                }
            } else {
                true
            }
        });
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }

    pub fn get_comment(&self, line: usize) -> Option<Comment> {
        self.comments
            .iter()
            .filter(|comment| comment.is_for_line(line))
            .cloned()
            .next()
    }

    pub fn get_comment_lines(&self) -> Vec<usize> {
        self.comments
            .iter()
            .map(|comment| comment.get_line())
            .collect()
    }

    pub fn update_comment(&mut self, line: usize, new_val: String) {
        self.comments
            .iter_mut()
            .filter(|comment| comment.is_for_line(line))
            .next()
            .map(move |comment| comment.update_body(new_val));
    }

    pub fn delete_comment(&mut self, line: usize) {
        self.comments.retain(|comment| !comment.is_for_line(line));
    }

    pub fn get_start_and_end_time(&self) -> Option<(i32, i32)> {
        let start_time = self.get_body().iter().find_map(|line| {
            let line = get_content_of_raw_colored_text(line);
            parse_prompt_time(&line, 0)
        })?;
        let end_time = self.get_body().iter().rev().find_map(|line| {
            let line = get_content_of_raw_colored_text(line);
            parse_prompt_time(&line, start_time)
        })?;
        Some((start_time, end_time))
    }

    pub fn build_time_slices(
        &self,
        observer: &impl Fn(&AetTimeSlice) -> Vec<AetObservation>,
    ) -> Vec<AetTimeSlice> {
        let (me, _you) = parse_me_and_you(self);
        let mut slices = Vec::new();
        let mut slice_lines = Vec::new();
        let mut last_time = 0;
        for (line_idx, line_text) in self.get_body().iter().enumerate() {
            let line_text = get_content_of_raw_colored_text(line_text);
            if let Some(time) = parse_prompt_time(&line_text, last_time) {
                last_time = time;
                let mut slice = AetTimeSlice {
                    observations: None,
                    gmcp: Vec::new(),
                    lines: slice_lines,
                    prompt: AetPrompt::Promptless,
                    time,
                    me: me.clone(),
                };
                slice.observations = Some(observer(&slice));
                slices.push(slice);
                slice_lines = Vec::new();
            } else {
                slice_lines.push((line_text, line_idx as u32));
            }
        }
        slices
    }

    pub fn build_line_times(&self) -> Vec<(usize, i32)> {
        let mut times = Vec::new();
        let mut last_time = 0;
        for (line_idx, line_text) in self.get_body().iter().enumerate() {
            let line_text = get_content_of_raw_colored_text(line_text);
            if let Some(time) = parse_prompt_time(&line_text, last_time) {
                times.push((line_idx, time));
                last_time = time;
            }
        }
        times.push((self.get_body().len(), last_time));
        times
    }
}
