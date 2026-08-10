#[cfg(feature = "regex")]
use fancy_regex::Regex;
#[cfg(feature = "regex")]
use lazy_static::lazy_static;

#[cfg(feature = "regex")]
fn format_dollar_group(
    dollar: &str,
    separator: &str,
) -> String {
    lazy_static! {
        static ref GROUP_REGEX: Regex = Regex::new(r"(\d)(?=(\d{3})+\b)").unwrap();
    }

    let replace = format!("$1{separator}");

    GROUP_REGEX.replace_all(dollar, replace).to_string()
}

#[cfg(feature = "regex")]
fn format_dollar_vedic(
    dollar: &str,
    separator: &str,
) -> String {
    lazy_static! {
        static ref VEDIC_REGEX: Regex = Regex::new(r"(\d)(?=(\d\d)+\d\b)").unwrap();
    }

    let replace = format!("$1{separator}");

    VEDIC_REGEX.replace_all(dollar, replace).to_string()
}

#[cfg(feature = "regex")]
pub fn format_dollar(
    use_vedic: bool,
    dollar: &str,
    separator: &str,
) -> String {
    if use_vedic {
        format_dollar_vedic(dollar, separator)
    } else {
        format_dollar_group(dollar, separator)
    }
}

#[cfg(feature = "regex")]
pub fn replace_any_non_numeric_values(
    decimal: &str,
    value_allow_negative: &str,
) -> String {
    lazy_static! {
        static ref REGEX: fn(&str) -> Regex =
            |decimal: &str| Regex::new(&format!(r"[^-\d{decimal}]")).unwrap();
    }

    REGEX(decimal)
        .replace_all(value_allow_negative, "")
        .to_string()
}

#[cfg(feature = "regex")]
pub fn allow_negative_values(value: &str) -> String {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"\((.*)\)").unwrap();
    }

    REGEX.replace(value, "-$1").to_string()
}

#[cfg(feature = "regex")]
pub fn convert_any_decimal_values(
    decimal: &str,
    numeric_values: &str,
) -> String {
    lazy_static! {
        static ref REGEX: fn(&str) -> Regex =
            |decimal: &str| Regex::new(&("\\".to_string() + decimal)).unwrap();
    }

    REGEX(decimal).replace_all(numeric_values, ".").to_string()
}

#[cfg(not(feature = "regex"))]
fn format_dollar_group(
    dollar: &str,
    separator: &str,
) -> String {
    let mut formatted = String::with_capacity(dollar.len() + dollar.len() / 3);

    for (i, ch) in dollar.chars().rev().enumerate() {
        if i != 0 && i % 3 == 0 {
            formatted.push_str(separator);
        }
        formatted.push(ch);
    }

    formatted.chars().rev().collect()
}

#[cfg(not(feature = "regex"))]
fn format_dollar_vedic(
    dollar: &str,
    separator: &str,
) -> String {
    let chars: Vec<char> = dollar.chars().collect();
    let mut formatted = String::with_capacity(dollar.len() + dollar.len() / 2);
    let len = chars.len();

    if len > 3 {
        let offset = len - 3;
        let mut prefix_len = offset % 2;
        if prefix_len == 0 && offset > 0 {
            prefix_len = 2;
        }

        formatted.extend(&chars[..prefix_len]);
        if prefix_len > 0 {
            formatted.push_str(separator);
        }

        for i in (prefix_len..offset).step_by(2) {
            formatted.push(chars[i]);
            formatted.push(chars[i + 1]);
            formatted.push_str(separator);
        }

        formatted.extend(&chars[offset..]);
    } else {
        formatted.extend(chars);
    }

    formatted
}

#[cfg(not(feature = "regex"))]
pub fn format_dollar(
    use_vedic: bool,
    dollar: &str,
    separator: &str,
) -> String {
    if use_vedic {
        format_dollar_vedic(dollar, separator)
    } else {
        format_dollar_group(dollar, separator)
    }
}

#[cfg(not(feature = "regex"))]
pub fn replace_any_non_numeric_values(
    decimal: &str,
    value: &str,
) -> String {
    let allowed = |c: char| c.is_ascii_digit() || c == '-' || c.to_string() == decimal;
    value.chars().filter(|&c| allowed(c)).collect()
}

#[cfg(not(feature = "regex"))]
pub fn allow_negative_values(value: &str) -> String {
    if let (Some(open), Some(close)) = (value.find('('), value.rfind(')')) {
        if open < close {
            return format!(
                "{}-{}{}",
                &value[..open],
                &value[open + 1..close],
                &value[close + 1..]
            );
        }
    }
    value.to_string()
}

#[cfg(not(feature = "regex"))]
pub fn convert_any_decimal_values(
    decimal: &str,
    numeric_values: &str,
) -> String {
    numeric_values.replace(decimal, ".")
}
