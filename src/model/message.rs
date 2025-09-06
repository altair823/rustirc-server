pub struct Message {
    pub prefix: Option<Prefix>, // optional
    pub command: Command,
    pub params: Vec<String>,
}

pub enum Prefix {
    // either server_name or (nick, user, host)
    ServerName(String), // see RFC 952
    User {
        nick: String, // any string starting with a letter and no spaces.  - [ ] \ ^ { } are allowed after the first character
        user: Option<String>, // optional, any string with no spaces
        host: Option<String>, // optional, see RFC 952
    },
}

pub enum Command {
    Letter(String), // at least 1 letter (A-Z, a-z) with no spaces
    Number(u16),    // exactly 3 digits
}
