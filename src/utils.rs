#[macro_export]
macro_rules! ref_or_return {
    ($option_name:expr) => {
        match $option_name {
            Some(ref t) => t,
            None => return,
        }
    }
}