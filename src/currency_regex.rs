use fancy_regex::Regex;
use lazy_static::lazy_static;

fn format_dollar_group(
    dollar: String,
    separator: String,
) -> String {
    lazy_static! {
        static ref GROUP_REGEX: Regex = Regex::new(r"(\d)(?=(\d{3})+\b)").unwrap();
    }

    let replace = format!("$1{separator}");

    GROUP_REGEX.replace_all(&dollar, replace).to_string()
}

fn format_dollar_vedic(
    dollar: String,
    separator: String,
) -> String {
    lazy_static! {
        static ref VEDIC_REGEX: Regex = Regex::new(r"(\d)(?=(\d\d)+\d\b)").unwrap();
    }

    let replace = format!("$1{separator}");

    VEDIC_REGEX.replace_all(&dollar, replace).to_string()
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
    value_allow_negative: String,
) -> String {
    lazy_static! {
        static ref REGEX: fn(String) -> Regex =
            |decimal: String| Regex::new(&format!(r"[^-\d{decimal}]")).unwrap();
    }

    REGEX(decimal)
        .replace_all(&value_allow_negative, "")
        .to_string()
}

pub fn allow_negative_values(value: &str) -> String {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"\((.*)\)").unwrap();
    }

    REGEX.replace(value, "-$1").to_string()
}

pub fn convert_any_decimal_values(
    decimal: String,
    numeric_values: String,
) -> String {
    lazy_static! {
        static ref REGEX: fn(String) -> Regex =
            |decimal: String| Regex::new(&("\\".to_string() + &decimal)).unwrap();
    }

    REGEX(decimal).replace_all(&numeric_values, ".").to_string()
}
