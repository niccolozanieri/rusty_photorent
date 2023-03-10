use std::fmt;

pub enum LensBrand {
    SONY,
    NIKON,
    CANON,
    LEICA,
    UNKNOWN,
}

impl std::fmt::Display for LensBrand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value: String;
        match *self {
            LensBrand::LEICA => value = String::from("Leica"),
            LensBrand::UNKNOWN => value = String::from("Unknown"),
            LensBrand::SONY => value = String::from("Sony"),
            LensBrand::CANON => value = String::from("Canon"),
            LensBrand::NIKON => value = String::from("Nikon"),
        };
        write!(f, "{}", value)

    }
}