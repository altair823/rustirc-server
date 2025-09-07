use crate::parser::model::command_enum::Command;

#[derive(Debug)]
pub struct Message<'a> {
    pub prefix: Option<Prefix<'a>>, // optional
    pub command: Command,
    pub params: Vec<&'a str>,
}

#[derive(Debug)]
pub enum Prefix<'a> {
    // either server_name or (nick, user, host)
    ServerName(&'a str), // see RFC 952
    User {
        nick: &'a str, // any string starting with a letter and no spaces.  - [ ] \ ^ { } are allowed after the first character
        user: Option<&'a str>, // optional, any string with no spaces
        host: Option<&'a str>, // optional, see RFC 952
    },
}
