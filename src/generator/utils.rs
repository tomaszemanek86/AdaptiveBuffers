pub fn to_snake_case(s: &str) -> String {
    let mut new_s = String::new();
    let mut last_was_uppercase = false;
    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            new_s.push(c.to_ascii_lowercase());
            last_was_uppercase = c.is_ascii_uppercase();
        } else {
            if c.is_ascii_uppercase() {
                if last_was_uppercase {
                    new_s.push(c.to_ascii_lowercase());
                } else {
                    new_s.push('_');
                    new_s.push(c.to_ascii_lowercase());
                }
                last_was_uppercase = true;
            } else {
                new_s.push(c);
                last_was_uppercase = false;
            }
        }
    }
    new_s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_snake_case_test() {
        assert_eq!(to_snake_case("AbCdEfgHI",), "ab_cd_efg_hi".to_string())
    }
}
