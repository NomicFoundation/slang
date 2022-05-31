pub fn to_unit<T>(_: T) {}

pub fn to_char<T>(_: T) -> char {
    'a'
}

pub fn to_str<T>(_: T) -> &'static str {
    "a"
}
