use aho_corasick::AhoCorasick;

fn format_dollar_group(
    dollar: String,
    separator: String,
) -> String {
    let mut formatted_integer = String::new();

    for (i, c) in dollar.chars().rev().enumerate() {
        formatted_integer.push(c);

        if (i + 1) % 3 == 0 && i != dollar.len() - 1 {
            formatted_integer.push_str(separator.as_str());
        }
    }

    formatted_integer.chars().rev().collect()
}

fn format_dollar_vedic(
    dollar: String,
    separator: String,
) -> String {
    let mut formatted_integer = String::new();

    let mut index: usize;

    let mut rem: usize;

    for (i, c) in dollar.chars().rev().enumerate() {
        formatted_integer.push(c);

        rem = if formatted_integer.len() < 3 { 3 } else { 2 };

        index = i + if formatted_integer.len() < 3 { 1 } else { 0 };

        if index % rem == 0 && i != dollar.len() - 1 {
            formatted_integer.push_str(separator.as_str());
        }
    }

    formatted_integer.chars().rev().collect()
}

pub fn format_dollar(
    use_vedic: bool,
    dollar: String,
    separator: String,
) -> String {
    if use_vedic {
        return format_dollar_vedic(dollar, separator);
    }

    format_dollar_group(dollar, separator)
}

pub fn replace_any_non_numeric_values(
    decimal: String,
    value: String,
) -> String {
    let patterns = &[
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "-",
        decimal.as_str(),
    ];

    let ac = AhoCorasick::new(patterns).unwrap();

    let mut new_value: String = String::from("");

    for mat in ac.find_iter(&value) {
        new_value.push(value.as_bytes()[mat.start()] as char);
    }

    new_value
}

pub fn allow_negative_values(value: &str) -> String {
    let patterns = &["(", ")"];

    let ac = AhoCorasick::new(patterns).unwrap();

    let mut new_value: String = value.to_string();

    let mut start: usize;

    let mut end: usize;

    for (i, mat) in ac.find_iter(value).enumerate() {
        start = mat.start() - i;

        end = mat.end() - i;

        new_value.replace_range(start..end, "");
    }

    new_value
}

pub fn convert_any_decimal_values(
    decimal: String,
    value: String,
) -> String {
    let patterns = &[decimal.as_str()];

    let ac = AhoCorasick::new(patterns).unwrap();

    let mut new_value: String = value.clone();

    for mat in ac.find_iter(&value) {
        new_value.replace_range(mat.span().range(), ".");
    }

    new_value
}
