// String helpers ported from the regexes in currency.js `parse()` and `format()`.

/// Insert `separator` between digit groups: 3s, or Indian 2s after the last 3 with `use_vedic`.
/// `dollar` is ASCII digits only.
#[allow(clippy::manual_is_multiple_of)] // `is_multiple_of` needs Rust 1.87
pub fn format_dollar(
    use_vedic: bool,
    dollar: &str,
    separator: &str,
) -> String {
    let len = dollar.len();
    let mut out = String::with_capacity(len + len / 2 * separator.len());

    for (i, ch) in dollar.char_indices() {
        let remaining = len - i;
        let boundary = if use_vedic {
            remaining >= 3 && remaining % 2 == 1
        } else {
            remaining % 3 == 0
        };
        if i > 0 && boundary {
            out.push_str(separator);
        }
        out.push(ch);
    }

    out
}

/// JS: `.replace(/[^-\d<decimal>]/g, '')`. `\d` is ASCII-only in JS.
pub fn replace_any_non_numeric_values(
    decimal: &str,
    value: &str,
) -> String {
    value
        .chars()
        .filter(|&c| c.is_ascii_digit() || c == '-' || decimal.contains(c))
        .collect()
}

/// JS: `.replace(/\((.*)\)/, '-$1')`, i.e. `(1.99)` → `-1.99`.
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

pub fn convert_any_decimal_values(
    decimal: &str,
    numeric_values: &str,
) -> String {
    numeric_values.replace(decimal, ".")
}

/// JS `Number.prototype.toFixed`: exact ties round away from zero and `-0` prints as `0`.
/// Rust's `{:.N}` rounds ties to even and keeps the sign of `-0`.
pub fn js_to_fixed(
    x: f64,
    dp: usize,
) -> String {
    let x = x + 0.; // -0 → 0
    if !is_tie(x, dp) {
        return format!("{x:.dp$}");
    }

    // An exact tie is exactly representable with dp + 1 digits ending in 5:
    // drop the 5 and add one unit in the last place.
    let mut d = format!("{:.*}", dp + 1, x.abs()).into_bytes();
    d.pop();
    if d.last() == Some(&b'.') {
        d.pop();
    }
    let mut i = d.len();
    loop {
        if i == 0 {
            d.insert(0, b'1');
            break;
        }
        i -= 1;
        match d[i] {
            b'.' => {}
            b'9' => d[i] = b'0',
            _ => {
                d[i] += 1;
                break;
            }
        }
    }

    let s = String::from_utf8(d).unwrap();
    if x < 0. {
        format!("-{s}")
    } else {
        s
    }
}

/// True when `x * 10^dp` ends in exactly `.5`. With `x = m * 2^k`, that holds iff
/// `2 * m * 5^dp * 2^(dp + k)` is an odd integer, i.e. `trailing_zeros(m) + dp + 1 + k == 0`.
fn is_tie(
    x: f64,
    dp: usize,
) -> bool {
    if x == 0. || !x.is_finite() {
        return false;
    }
    let bits = x.abs().to_bits();
    let exp = (bits >> 52) as i64;
    let frac = bits & ((1 << 52) - 1);
    let (m, k) = if exp == 0 {
        (frac, -1074)
    } else {
        (frac | 1 << 52, exp - 1075)
    };
    m.trailing_zeros() as i64 + dp as i64 + 1 + k == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_fixed_matches_js() {
        // Expected values taken from node.
        assert_eq!(js_to_fixed(0.25, 1), "0.3");
        assert_eq!(js_to_fixed(-0.25, 1), "-0.3");
        assert_eq!(js_to_fixed(2.5, 0), "3");
        assert_eq!(js_to_fixed(-2.5, 0), "-3");
        assert_eq!(js_to_fixed(9.5, 0), "10");
        assert_eq!(js_to_fixed(0.15, 1), "0.1"); // 0.1499999… is not a tie
        assert_eq!(js_to_fixed(1.005, 2), "1.00");
        assert_eq!(js_to_fixed(90071992547409.9, 4), "90071992547409.9063"); // exactly …409.90625
        assert_eq!(js_to_fixed(-0., 2), "0.00");
        assert_eq!(js_to_fixed(-0.00001, 4), "-0.0000");
    }

    #[test]
    fn groups_with_multi_char_separator() {
        assert_eq!(format_dollar(false, "1234567", ", "), "1, 234, 567");
        assert_eq!(
            format_dollar(true, "1234567", "&nbsp;"),
            "12&nbsp;34&nbsp;567"
        );
    }
}
