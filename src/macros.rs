#![macro_escape]

macro_rules! unwrap_or {
    ($expr:expr, $or:expr) => (
        match $expr {
            Some(x) => x,
            None => { $or; }
        }
    )
}
