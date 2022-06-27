use std::fmt;

#[derive(Debug)]
pub struct Mech {}

impl fmt::Display for Mech {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(hi)")
    }
}
