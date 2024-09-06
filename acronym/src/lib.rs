pub fn abbreviate(phrase: &str) -> String {
    phrase.split(|x| x == ' ' || x == '-')
    .filter(|x| !x.is_empty())
    .flat_map(|x| {
        if x.chars().all(|x| x.is_ascii_uppercase()) || x.chars().all(|x| x.is_ascii_lowercase()) {
            [x.chars().next().unwrap().to_ascii_uppercase()].to_vec()
        } else {
            x.chars().filter(|c| c.is_uppercase()).collect::<Vec<_>>()
        }
    })
    .collect()
}
