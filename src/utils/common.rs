pub fn replace_first_and_last(
    s: &str,
    first: &str,
    last: &str,
    first_replace: &str,
    last_replace: &str,
) -> String {
    let mut s = s.to_string();
    s.replace_range(0..first.len(), first_replace);
    s.replace_range(s.len() - last.len()..s.len(), last_replace);
    s
}
