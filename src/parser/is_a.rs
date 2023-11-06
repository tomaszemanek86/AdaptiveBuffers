pub fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

pub fn is_letter(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

pub fn is_underscore(c: char) -> bool {
    c == '_'
}

pub fn is_word_beg(c: char) -> bool {
    is_letter(c) || is_underscore(c)
}

pub fn is_word_mid(c: char) -> bool {
    is_word_beg(c) || is_digit(c)
}

pub fn is_white_space(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n'
}
