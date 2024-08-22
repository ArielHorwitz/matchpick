use anyhow::{bail, Result};

pub fn process(
    utf8_data: &str,
    match_against: Option<String>,
    enter_pattern: &str,
    exit_pattern: &str,
    ignore_pattern: Option<String>,
) -> Result<String> {
    let mut matcher = MultilineMatch::new(
        match_against,
        enter_pattern.to_owned(),
        exit_pattern.to_owned(),
        ignore_pattern,
    );
    let mut outputs: Vec<String> = Vec::new();
    for (i, line) in utf8_data.lines().enumerate() {
        if let Some(line) = matcher
            .check_line(line)
            .map_err(|e| anyhow::anyhow!("parse failed at line {}: {e}", i + 1))?
        {
            outputs.push(line);
        };
    }
    Ok(outputs.join("\n"))
}

#[derive(Debug, Clone)]
struct MultilineMatch {
    enter_pattern: String,
    exit_pattern: String,
    ignore_pattern: Option<String>,
    match_against: Option<String>,
    default_case_buffer: Vec<String>,
    state: State,
}

impl MultilineMatch {
    fn new(
        match_against: Option<String>,
        enter_pattern: String,
        exit_pattern: String,
        ignore_pattern: Option<String>,
    ) -> Self {
        Self {
            enter_pattern,
            exit_pattern,
            ignore_pattern,
            match_against,
            default_case_buffer: Vec::new(),
            state: State::Normal,
        }
    }

    fn check_line(&mut self, line: &str) -> Result<Option<String>> {
        let output = match self.check_new_state(line) {
            Some(new_state) => self.handle_new_state(new_state)?,
            None => self.handle_normal_line(line),
        };
        Ok(output)
    }

    fn handle_normal_line(&mut self, line: &str) -> Option<String> {
        match &self.state {
            State::Normal | State::Matched => Some(line.to_owned()),
            State::Default => {
                self.default_case_buffer.push(line.to_owned());
                None
            }
            State::Other | State::Done => None,
        }
    }

    fn check_new_state(&self, line: &str) -> Option<NewState> {
        if let Some(ignore_pattern) = &self.ignore_pattern {
            if line.contains(ignore_pattern) {
                return None;
            }
        }
        if let Some((_pat, names)) = line.split_once(&self.enter_pattern) {
            let names = names.trim();
            if names.is_empty() {
                Some(NewState::Enter)
            } else {
                let names = names
                    .split_whitespace()
                    .map(std::borrow::ToOwned::to_owned)
                    .collect();
                Some(NewState::Switch(names))
            }
        } else if line.contains(&self.exit_pattern) {
            Some(NewState::Exit)
        } else {
            None
        }
    }

    // Allow same arms for the sake of comments explaining each state change in the state machine
    #[allow(clippy::match_same_arms)]
    fn handle_new_state(&mut self, new_state: NewState) -> Result<Option<String>> {
        let mut result_value = None;
        self.state = match (&self.state, new_state) {
            // Enter matching
            (State::Normal, NewState::Enter) => State::Default,
            // Entering new switch case (check if matched)
            (State::Default | State::Other, NewState::Switch(names)) => {
                if let Some(match_against) = &self.match_against {
                    if names.contains(match_against) {
                        // Found case
                        State::Matched
                    } else {
                        // Switch case
                        State::Other
                    }
                } else {
                    // Wanted default
                    result_value = Some(self.default_case_buffer.join("\n"));
                    State::Done
                }
            }
            // Leaving matched case for another (no further action needed)
            (State::Matched, NewState::Switch(_)) => State::Done,
            // Leaving switch case for another (already done)
            (State::Done, NewState::Switch(_)) => State::Done,
            // Exiting normally
            (State::Matched | State::Done, NewState::Exit) => {
                self.default_case_buffer.clear();
                State::Normal
            }
            // Exiting without match (use default buffer)
            (State::Other, NewState::Exit) => {
                result_value = Some(self.default_case_buffer.join("\n"));
                self.default_case_buffer.clear();
                State::Normal
            }
            // Invalid state changes
            (State::Normal, NewState::Switch(_)) => {
                bail!("cannot start new case: need default first")
            }
            (State::Normal, NewState::Exit) => bail!("cannot end match: not in match"),
            (State::Default, NewState::Enter) => {
                bail!("cannot start new match: in default of previous match")
            }
            (State::Default, NewState::Exit) => bail!("ended match without alternatives"),
            (State::Other, NewState::Enter) => {
                bail!("cannot start new match: switching previous match")
            }
            (State::Matched, NewState::Enter) => {
                bail!("cannot start new match: in matched case of previous match")
            }
            (State::Done, NewState::Enter) => {
                bail!("cannot start new match: no exit of previous match")
            }
        };
        Ok(result_value)
    }
}

#[derive(Debug, Clone, Copy)]
enum State {
    // Not matching
    Normal,
    // Buffering the default case
    Default,
    // In the matched case
    Matched,
    // In another case
    Other,
    // Post-match, no further action required
    Done,
}

#[derive(Debug)]
enum NewState {
    // Start new match
    Enter,
    // New switch case
    Switch(Vec<String>),
    // Finish match
    Exit,
}
