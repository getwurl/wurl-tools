use std::str::FromStr;
use std::time::Duration;
use regex::Regex;

lazy_static! {
    static ref MESSAGE_RE: Regex = Regex::new(r"^(send (?P<message>.+) )?(?P<command>every|after) (?P<interval>\d+)(?P<unit>\w)$").expect("Failed to compile instruction regex");
}

#[derive(Debug, PartialEq)]
pub enum Command {
    INTERVAL,
    DELAY
}

#[derive(Debug, PartialEq)]
pub struct InstructionParseError {
    reason: String,

}

impl InstructionParseError {
    pub fn new<T: Into<String>>(reason: T) -> InstructionParseError {
        InstructionParseError {
            reason: reason.into()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    message: Option<String>,
    command: Command,
    duration: Duration,
}

impl Instruction {
    pub fn message(self: &Instruction) -> &Option<String> {
        &self.message
    }

    pub fn command(self: &Instruction) -> &Command {
        &self.command
    }

    pub fn duration(self: &Instruction) -> &Duration {
        &self.duration
    }
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let matches_opt = MESSAGE_RE.captures(input);
        if matches_opt.is_none() {
            return Err(InstructionParseError::new(format!("Invalid instruction: {}", input)));
        }
        let matches = matches_opt.unwrap();

        Ok(Instruction {
            message: matches.name("message").map_or(None, |m| Some(String::from(m.as_str()))),
            command: match matches["command"].to_lowercase().as_ref() {
                "every" => Command::INTERVAL,
                "after" => Command::DELAY,
                _ => unimplemented!()
            },
            duration: Duration::from_secs(matches["interval"].parse().unwrap()),
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use std::time::Duration;
    use super::{Command, Instruction, InstructionParseError};

    #[test]
    fn parse_interval() {
        let result = Instruction::from_str("every 2s");
        let expected = Instruction {
            message: None,
            command: Command::INTERVAL,
            duration: Duration::from_secs(2),
        };
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn parse_delay() {
        let result = Instruction::from_str("after 2s");
        let expected = Instruction {
            message: None,
            command: Command::DELAY,
            duration: Duration::from_secs(2),
        };
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn parse_interval_with_message() {
        let result = Instruction::from_str("send {\"type\": \"PING\"} every 2s");
        let expected = Instruction {
            message: Some(String::from("{\"type\": \"PING\"}")),
            command: Command::INTERVAL,
            duration: Duration::from_secs(2),
        };
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn parse_delay_with_message() {
        let result = Instruction::from_str("send {\"type\": \"PING\"} after 2s");
        let expected = Instruction {
            message: Some(String::from("{\"type\": \"PING\"}")),
            command: Command::DELAY,
            duration: Duration::from_secs(2),
        };
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn parse_invalid_messages() {
        let result = Instruction::from_str("Hello");
        let expected = InstructionParseError::new(String::from("Invalid instruction: Hello"));
        assert_eq!(expected, result.unwrap_err());
    }
}
