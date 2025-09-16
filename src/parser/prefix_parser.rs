use crate::parser::model::error::ParseError;
use crate::parser::model::message::Prefix;

use regex::{Regex, RegexBuilder};
use std::sync::LazyLock;

pub fn parse_prefix(input: &str) -> Result<Prefix<'_>, ParseError> {
    static PREFIX_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        RegexBuilder::new(
            r"^:(?P<server_or_nick>[A-Za-z0-9\-\[\]\\`^{}:./]*)(!(?P<user>[^@ ]+))?(@(?P<host>[^ ]+))?$")
            .build()
            .unwrap()
    });
    if let Some(caps) = PREFIX_REGEX.captures(input) {
        let server_or_nick = caps.name("server_or_nick").unwrap().as_str();
        if server_or_nick.is_empty() {
            return Err(ParseError::InvalidPrefix(input.to_string()));
        }
        let user = caps.name("user").map(|m| m.as_str());
        let host = caps.name("host").map(|m| m.as_str());

        return if user.is_none() && host.is_none() {
            // It's a server name
            Ok(Prefix::ServerName(server_or_nick))
        } else {
            // It's a user prefix
            Ok(Prefix::User {
                nick: server_or_nick,
                user,
                host,
            })
        };
    }
    Err(ParseError::InvalidPrefix(input.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_servername_or_nick() {
        let input = ":server.com";
        let prefix = parse_prefix(input).unwrap();
        assert!(matches!(prefix, Prefix::ServerName(name) if name == "server.com"));

        let input = ":192.168.0.35";
        let prefix = parse_prefix(input).unwrap();
        assert!(matches!(prefix, Prefix::ServerName(name) if name == "192.168.0.35"));

        let input = ":nick!user@host";
        let prefix = parse_prefix(input).unwrap();
        assert!(
            matches!(prefix, Prefix::User { nick, user, host } if nick == "nick" && user == Some("user") && host == Some("host"))
        );

        // Test with just nick
        // Should be treated as ServerName before it determines as a nick later
        let input = ":nick";
        let prefix = parse_prefix(input).unwrap();
        assert!(matches!(prefix, Prefix::ServerName(name) if name == "nick"));
    }

    #[test]
    fn test_prefix_nick() {
        let input = ":nick!user@host";
        let prefix = parse_prefix(input).unwrap();
        assert!(
            matches!(prefix, Prefix::User { nick, user, host } if nick == "nick" && user == Some("user") && host == Some("host"))
        );

        let input = ":nick!user";
        let prefix = parse_prefix(input).unwrap();
        assert!(
            matches!(prefix, Prefix::User { nick, user, host } if nick == "nick" && user == Some("user") && host == None)
        );

        let input = ":nick@host";
        let prefix = parse_prefix(input).unwrap();
        assert!(
            matches!(prefix, Prefix::User { nick, user, host } if nick == "nick" && user == None && host == Some("host"))
        );

        // Test with just nick
        // Should be treated as ServerName before it determines as a nick later
        let input = ":nick";
        let prefix = parse_prefix(input).unwrap();
        assert!(matches!(prefix, Prefix::ServerName(name) if name == "nick"));
    }

    #[test]
    fn test_prefix_invalid() {
        let mut input = ":!user@host";
        let err = parse_prefix(input);
        assert!(matches!(err, Err(ParseError::InvalidPrefix(_))));

        input = ":nick!@host";
        let err = parse_prefix(input);
        assert!(matches!(err, Err(ParseError::InvalidPrefix(_))));

        input = ":nick!user@";
        let err = parse_prefix(input);
        assert!(matches!(err, Err(ParseError::InvalidPrefix(_))));
    }
}
