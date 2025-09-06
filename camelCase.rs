fn to_camel_case(s: &str) -> String {
    // Split by one or more spaces, dashes, or underscores
    let words: Vec<&str> = s
        .split(|c: char| c == ' ' || c == '-' || c == '_')
        .filter(|w| !w.is_empty())
        .collect();

    if words.is_empty() {
        return String::new();
    }

    let first = words[0].to_lowercase();
    let rest: String = words[1..]
        .iter()
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(first_char) => first_char.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                None => String::new(),
            }
        })
        .collect();

    first + &rest
}


