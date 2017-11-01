use tg_botapi::types;

#[macro_export]
macro_rules! ref_or_return {
    ($option_name:expr) => {
        match $option_name {
            Some(ref t) => t,
            None => return,
        }
    }
}

macro_rules! ref_or_return_oknone {
    ($option_name:expr) => {
        match $option_name {
            Some(ref t) => t,
            None => return Ok(None),
        }
    }
}

pub fn escape_html(s: &str) -> String {
    s.replace("&", "&amp;")
     .replace("<", "&lt;")
     .replace(">", "&gt;")
}

pub fn escape_md(s: &str) -> String {
    s.replace("_", "\\_")
     .replace("[", "\\[")
     .replace("]", "\\]")
     .replace("*", "\\*")
     .replace("`", "\\`")
}

pub fn make_name(user: &types::User) -> String {
    match user.last_name {
        Some(ref last_name) => {
            format!("{} {}", &user.first_name, last_name)
        }

        None => {
            user.first_name.to_string()
        }
    }
}
