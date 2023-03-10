use std::fmt;

pub enum CameraBrand {
    SONY,
    NIKON,
    CANON,
    UNKNOWN,
}

impl std::fmt::Display for CameraBrand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value: String;
        match *self {
            CameraBrand::UNKNOWN => value = String::from("Unknown"),
            CameraBrand::SONY => value = String::from("Sony"),
            CameraBrand::CANON => value = String::from("Canon"),
            CameraBrand::NIKON => value = String::from("Nikon"),
        };
        write!(f, "{}", value)

    }
}
