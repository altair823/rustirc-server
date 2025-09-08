use crate::parser::model::command_enum::Command;
use crate::parser::model::error::ParseError;
use crate::parser::model::message::{Message, Prefix};
use crate::parser::prefix_parser::parse_prefix;

pub fn parse_message(input: &str) -> Result<Message, ParseError> {
    let input = input.trim_end_matches("\r\n");
    let parts: Vec<&str> = input.splitn(3, ' ').collect();
    if parts.len() < 2 {
        return Err(ParseError::InvalidFormat);
    }
    let prefix: Option<Prefix> = if parts[0].starts_with(':') {
        Some(parse_prefix(parts[0])?)
    } else {
        None
    };
    let params: Vec<&str> = if parts.len() > 2 {
        parts[2].split(' ').collect()
    } else {
        Vec::new()
    };

    Ok(Message {
        prefix: Some(Prefix::ServerName("server.com")),
        command: Command::PASS,
        params: Vec::new(),
    })
}
