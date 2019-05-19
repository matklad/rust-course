use std::borrow::Cow;

fn to_lowercase<'a>(s: &'a str) -> Cow<'a, str> {
    if s.chars().all(char::is_lowercase) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.to_lowercase())
    }
}
