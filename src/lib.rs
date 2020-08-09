mod matcher;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matcher_test() {
        let matched = matcher::match_with("this is test");
        assert_eq!(matched, "this is test");
    }
}
