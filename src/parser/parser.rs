use crate::parser::model::error::ParseError;
use crate::parser::model::message::{Message, Prefix};

use crate::parser::model::command_enum::Command;

fn parse_prefix(input: &str) -> Result<Prefix, ParseError> {
    if input.contains('!') || input.contains('#') {
        let user_parts: Vec<&str> = input.split('!').collect();
        if user_parts.len() != 2 {
            return Err(ParseError::InvalidPrefix);
        }
        let nick = user_parts[0];
        let host_parts: Vec<&str> = user_parts[1].split('@').collect();
        let (user, host) = match host_parts.len() {
            1 => (Some(host_parts[0]), None),
            2 => (Some(host_parts[0]), Some(host_parts[1])),
            _ => return Err(ParseError::InvalidPrefix),
        };
        Ok(Prefix::User { nick, user, host })
    } else {
        Ok(Prefix::ServerName(input))
    }
}

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
        prefix: Some(Prefix::ServerName(prefix.unwrap_or("unknown"))),
        command: Command::PASS,
        params: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_servername() {
        let inpub = ":server.com PASS secret\r\n";
        let msg = parse_message(inpub).unwrap();
        println!("{:#?}", msg);
    }

    #[test]
    fn test_prefix_user() {
        let inpub = ":nick!user@host JOIN #channel\r\n";
        let msg = parse_message(inpub).unwrap();
        println!("{:#?}", msg);
    }
}
