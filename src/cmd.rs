pub enum Command {
    Get,
    Set,
    Invalid,
  }

impl Command {
    pub fn from_str(str: &String) -> Command {
        match str.as_bytes() {
            b"set" => Command::Set,
            b"get" => Command::Get,
            _ => Command::Invalid,
        }
    }
}