#![allow(
    clippy::excessive_precision,          // JS literals, kept verbatim
    clippy::inconsistent_digit_grouping,  // vedic groupings
    clippy::useless_conversion            // currency(currency(x)) port
)]

// 1:1 port of currency.js test/test.js (same names and order), then Rust-only tests.

use crate::{Currency, CurrencyErr, CurrencyOpts};

fn c(v: f64) -> Currency {
    Currency::from(v)
}

fn p(s: &str) -> Currency {
    s.parse().unwrap()
}

fn values(items: &[Currency]) -> Vec<f64> {
    items.iter().map(Currency::value).collect()
}

fn total(items: &[Currency]) -> f64 {
    items.iter().fold(0., |acc, x| acc + x.value())
}

#[test]
fn should_be_immutable() {
    let value = c(1.);
    let _new_value = &value + 0.25;

    assert_eq!(value, 1., "original value not modified");
}

#[test]
fn should_be_instance_of_currency() {
    fn is_currency(_: &Currency) {}
    is_currency(&c(1.));
}

#[test]
fn should_allow_numbers() {
    assert_eq!(c(1.23).value(), 1.23);
}

#[test]
fn should_allow_strings() {
    assert_eq!(p("1.23").value(), 1.23);
}

#[test]
fn should_allow_currency() {
    assert_eq!(Currency::from(p("1.23")).value(), 1.23);
}

#[test]
fn should_accept_strings_with_symbols() {
    assert_eq!(p("$1234.56").value(), 1234.56);
    assert_eq!(p("£78.90").value(), 78.90);
}

#[test]
fn should_allow_comma_delimited_strings() {
    assert_eq!(p("1,234,567.89").value(), 1234567.89);
}

#[test]
fn should_strip_invalid_characters() {
    assert_eq!(p("a1b2c3").value(), 123.);
}

#[test]
fn should_allow_negative_strings() {
    assert_eq!(p("(5.24)").value(), -5.24);
    assert_eq!(p("-12.34").value(), -12.34);
}

#[test]
fn should_default_to_0_with_invalid_strings() {
    assert_eq!(p("abc").value(), 0.);
}

#[test]
fn should_return_value() {
    assert_eq!(c(1.23).value(), 1.23);
}

#[test]
fn should_return_int_value() {
    assert_eq!(c(1.23).int_value(), 123);
}

#[test]
fn should_add_floating_point_value() {
    let value = c(2.51) + 0.01;

    assert_eq!(value.value(), 2.52);
    assert_ne!(value.value(), 2.51 + 0.01);
}

#[test]
fn should_subtract_floating_point() {
    let value = c(2.52) - 0.01;

    assert_eq!(value.value(), 2.51);
    assert_ne!(value.value(), 2.52 - 0.01);
}

#[test]
fn should_round_half_up() {
    assert_eq!(c(17.955).value(), 17.96);
    assert_eq!(c(17.855).value(), 17.86);
    assert_eq!(c(17.455).value(), 17.46);
    assert_eq!(p("17.955").value(), 17.96);
    assert_eq!(p("17.855").value(), 17.86);
    assert_eq!(p("17.455").value(), 17.46);
}

#[test]
fn should_round_negative_values_half_up() {
    assert_eq!(c(-17.955).value(), -17.95);
    assert_eq!(c(-17.855).value(), -17.85);
    assert_eq!(c(-17.455).value(), -17.45);
    assert_eq!(p("-17.955").value(), -17.95);
    assert_eq!(p("-17.855").value(), -17.85);
    assert_eq!(p("-17.455").value(), -17.45);
}

#[test]
fn currency_multiplication() {
    let value = c(1.23) * 2.;
    let floating_value = c(0.1) * 0.2;

    assert_eq!(value.value(), 2.46);
    assert_eq!(floating_value.value(), 0.02);
    assert_ne!(floating_value.value(), 0.1 * 0.2);
}

#[test]
fn currency_multiplication_with_precision() {
    let value = CurrencyOpts::new().precision(3).from(1.369) * 3.;
    assert_eq!(value.value(), 4.107);
}

#[test]
fn currency_division() {
    assert_eq!((c(9.87) / 2.).value(), 4.94);
}

#[test]
fn currency_division_with_precision() {
    let value = CurrencyOpts::new().precision(3).from(4.107) / 3.;
    assert_eq!(value.value(), 1.369);
}

#[test]
fn should_parse_negative_values() {
    let pos = c(1.23);
    let neg = c(-1.23);
    let distribute = neg.distribute(4);

    assert_eq!(neg.value(), -1.23);
    assert_eq!((pos - 2.01).value(), -0.78);
    assert_eq!(distribute[0].value(), -0.31);
    assert_eq!(total(&distribute), -1.23);
}

#[test]
fn should_create_equal_distribution() {
    let values_ = c(1.00).distribute(4);

    assert_eq!(values_.len(), 4);
    assert_eq!(values(&values_), [0.25, 0.25, 0.25, 0.25]);
}

#[test]
fn should_create_non_equal_distribution_with_pennies() {
    let values_ = c(1.01).distribute(4);

    assert_eq!(values_[0].value(), 0.26);
    assert_eq!(values_[1].value(), 0.25);
    assert_eq!(total(&values_), 1.01);
}

#[test]
fn should_create_non_equal_distribution_with_a_negative_penny() {
    let values_ = c(-0.01).distribute(2);

    assert_eq!(values_[0].value(), -0.01);
    assert_eq!(values_[1].value(), 0.);
    assert_eq!(total(&values_), -0.01);
}

#[test]
fn should_get_dollar_value() {
    let value = c(1.23);

    assert_eq!((&value + 2.).dollars(), 3);
    assert_eq!((&value + 0.8).dollars(), 2);
    assert_eq!((&value - 3.).dollars(), -1);
}

#[test]
fn should_get_cent_value() {
    let value = c(1.23);

    assert_eq!(value.cents(), 23);
    assert_eq!((&value + 0.31).cents(), 54);
}

#[test]
fn should_support_different_precision_values() {
    let c1 = CurrencyOpts::new().precision(3).from(1.234);
    let c2 = CurrencyOpts::new().symbol("¥").precision(0).from(1.234);

    assert_eq!(c1.value(), 1.234);
    assert_eq!(c2.value(), 1.);
    assert_eq!(c1.int_value(), 1234);
    assert_eq!(c2.int_value(), 1);
    assert_eq!((&c1 + 4.567).value(), 5.801);
    assert_eq!((&c2 + 4.567).value(), 6.);
    assert_eq!((&c1 - 4.567).value(), -3.333);
    assert_eq!((&c2 - 4.567).value(), -4.);
    assert_eq!(c1.cents(), 234);
    assert_eq!(c2.cents(), 0);
    assert_eq!(c1.format(), "$1.234");
    assert_eq!(c2.format(), "¥1");
    assert_eq!(values(&c1.distribute(4)), [0.309, 0.309, 0.308, 0.308]);
    assert_eq!(values(&c2.distribute(4)), [1., 0., 0., 0.]);
}

#[test]
fn should_use_source_precision_for_arithmetic_with_different_precisions() {
    let c1 = c(1.23);
    let c2 = CurrencyOpts::new().precision(3).from(1.239);

    assert_eq!((&c1 + &c2).value(), 2.47);
    assert_eq!((&c2 + &c1).value(), 2.469);
}

#[test]
fn should_use_source_formatting_for_mixed_currency_formats() {
    let c1 = p("1,234.56");
    let c2 = CurrencyOpts::new()
        .separator(" ")
        .decimal(",")
        .parse("1 234,56")
        .unwrap();

    assert_eq!((&c1 + &c2).format(), "$2,469.12");
    assert_eq!((&c2 + &c1).format(), "$2 469,12");
}

#[test]
fn should_default_rounding_when_parsing() {
    assert_eq!(c(1.2349).value(), 1.23);
    assert_eq!(c(5.6789).value(), 5.68);
    assert_eq!((c(10.00) * 0.001).value(), 0.01);
    assert_eq!((c(0.01) / 0.001).value(), 10.);
}

#[test]
fn should_have_int_value_and_real_value() {
    let value1 = c(2.51) + 0.01;
    let value2 = c(2.52) - 0.01;

    assert_eq!(value1.value(), 2.52);
    assert_eq!(value1.int_value(), 252);
    assert_eq!(value2.value(), 2.51);
    assert_eq!(value2.int_value(), 251);
}

#[cfg(feature = "serde")]
#[test]
fn should_stringify_json() {
    let json = |v: Currency| serde_json::json!({ "value": v }).to_string();

    assert_eq!(json(c(1.23)), r#"{"value":1.23}"#);
    assert_eq!(json(c(2.51) + 0.01), r#"{"value":2.52}"#);
    assert_eq!(json(c(2.52) - 0.02), r#"{"value":2.5}"#);
}

#[test]
fn should_format_value_using_defaults() {
    let value1 = c(1.23);
    let value2 = c(1234.56);
    let value3 = c(1234567.89);
    let value4 = CurrencyOpts::new().precision(4).from(1234567.8912);
    let value5 = CurrencyOpts::new().precision(0).from(1234567.);

    assert_eq!(value1.format(), "$1.23");
    assert_eq!(value2.format(), "$1,234.56");
    assert_eq!(value3.format(), "$1,234,567.89");
    assert_eq!(value4.format(), "$1,234,567.8912");
    assert_eq!(value5.format(), "$1,234,567");
    assert_eq!((value1 * -1.).format(), "-$1.23");
    assert_eq!((value2 * -1.).format(), "-$1,234.56");
    assert_eq!((value3 * -1.).format(), "-$1,234,567.89");
    assert_eq!((value4 * -1.).format(), "-$1,234,567.8912");
    assert_eq!((value5 * -1.).format(), "-$1,234,567");
}

#[test]
fn should_format_value_using_international() {
    let opts = CurrencyOpts::new().separator(".").decimal(",");

    assert_eq!(opts.from(1.23).format(), "$1,23");
    assert_eq!(opts.from(1000.00).format(), "$1.000,00");
    assert_eq!(opts.from(1000000.00).format(), "$1.000.000,00");
}

#[test]
fn should_format_vedic_groupings() {
    let o = CurrencyOpts::new().use_vedic(true);
    let o4 = o.clone().precision(4);

    assert_eq!(o.from(1.23).format(), "$1.23");
    assert_eq!(o.from(1000.00).format(), "$1,000.00");
    assert_eq!(o.from(100000.00).format(), "$1,00,000.00");
    assert_eq!(o.from(1000000.00).format(), "$10,00,000.00");
    assert_eq!(o4.from(1234567.8912).format(), "$12,34,567.8912");
}

#[test]
fn should_format_using_patterns() {
    let o = CurrencyOpts::new().pattern("# !");

    assert_eq!(o.from(1.23).format(), "1.23 $");
    assert_eq!(o.from(1234.56).format(), "1,234.56 $");
    assert_eq!(o.from(1234567.89).format(), "1,234,567.89 $");
    assert_eq!(
        o.clone().precision(4).from(1234567.8912).format(),
        "1,234,567.8912 $"
    );
    assert_eq!(
        o.clone().precision(0).from(1234567.).format(),
        "1,234,567 $"
    );
}

#[test]
fn should_format_using_negative_patterns() {
    let o = CurrencyOpts::new().negative_pattern("! (#)");

    assert_eq!(o.from(-1.23).format(), "$ (1.23)");
    assert_eq!(o.from(-1234.56).format(), "$ (1,234.56)");
    assert_eq!(o.from(-1234567.89).format(), "$ (1,234,567.89)");
    assert_eq!(
        o.clone().precision(4).from(-1234567.8912).format(),
        "$ (1,234,567.8912)"
    );
    assert_eq!(
        o.clone().precision(0).from(-1234567.).format(),
        "$ (1,234,567)"
    );
}

#[test]
fn should_parse_international_values() {
    let o = CurrencyOpts::new().separator(".").decimal(",");

    assert_eq!(o.parse("1,23").unwrap().value(), 1.23);
    assert_eq!(o.parse("1.000,00").unwrap().value(), 1000.00);
    assert_eq!(o.parse("1.000.000,00").unwrap().value(), 1000000.00);
}

#[test]
fn should_format_with_symbol() {
    assert_eq!(c(1.23).format_with(|o| o.pattern("!#")), "$1.23");
}

#[test]
fn should_format_without_symbol() {
    assert_eq!(c(1.23).format_with(|o| o.pattern("#")), "1.23");
}

#[test]
fn should_format_with_international_symbol() {
    assert_eq!(c(1.23).format_with(|o| o.symbol("£")), "£1.23");
    assert_eq!(c(1.23).format_with(|o| o.symbol("¥")), "¥1.23");
}

// JS `format(fn)` has no Rust equivalent: a custom formatter is just a function
// taking `(&Currency, &CurrencyOpts)`.
#[test]
fn should_use_a_function_for_formatting() {
    let mut called = false;
    let mut format = |_: &Currency, _: &CurrencyOpts| {
        called = true;
        "1.23".to_string()
    };

    let cur = c(1234.56);
    let val = format(&cur, cur.opts());

    assert!(called);
    assert_eq!(val, "1.23");
}

#[test]
fn should_pass_options_when_using_a_function_for_formatting() {
    let format = |cur: &Currency, opts: &CurrencyOpts| (cur.value(), opts.symbol.clone());

    let cur = CurrencyOpts::new().symbol("£").from(1234.56);
    let (value, symbol) = format(&cur, cur.opts());

    assert_eq!(value, 1234.56);
    assert_eq!(symbol, "£");
}

#[test]
fn should_override_defaults_options_when_formatting_with_options() {
    let cur = CurrencyOpts::new().symbol("$").from(1.23);
    assert_eq!(cur.format_with(|o| o.symbol("£")), "£1.23");
}

// JS passes `undefined`; the Rust equivalent is an empty string.
#[test]
fn should_return_0_00_currency_with_invalid_input() {
    assert_eq!(p("").value(), 0.);
}

#[test]
fn should_round_down_to_nearest_value_when_using_increments() {
    let o = CurrencyOpts::new().increment(0.05);

    assert_eq!(o.from(1.01).to_string(), "1.00");
    assert_eq!(o.from(1.02).to_string(), "1.00");
    assert_eq!(o.from(1.06).to_string(), "1.05");
    assert_eq!(o.from(1.07).to_string(), "1.05");
    assert_eq!(o.from(1000.01).to_string(), "1000.00");
    assert_eq!(o.from(10000.01).to_string(), "10000.00");
    assert_eq!(o.from(100000.01).to_string(), "100000.00");
    assert_eq!(o.from(1000000.01).to_string(), "1000000.00");
}

#[test]
fn should_round_up_to_nearest_value_when_using_increments() {
    let o = CurrencyOpts::new().increment(0.05);

    assert_eq!(o.from(1.03).to_string(), "1.05");
    assert_eq!(o.from(1.04).to_string(), "1.05");
    assert_eq!(o.from(1.08).to_string(), "1.10");
    assert_eq!(o.from(1.09).to_string(), "1.10");
    assert_eq!(o.from(1000.09).to_string(), "1000.10");
    assert_eq!(o.from(10000.09).to_string(), "10000.10");
    assert_eq!(o.from(100000.09).to_string(), "100000.10");
    assert_eq!(o.from(1000000.09).to_string(), "1000000.10");
}

#[test]
fn should_handle_negative_rounding_when_using_increments() {
    let o = CurrencyOpts::new().increment(0.05);

    assert_eq!(o.from(-1.01).to_string(), "-1.00");
    assert_eq!(o.from(-1.02).to_string(), "-1.00");
    assert_eq!(o.from(-1.03).to_string(), "-1.05");
    assert_eq!(o.from(-1.04).to_string(), "-1.05");
    assert_eq!(o.from(-1.06).to_string(), "-1.05");
    assert_eq!(o.from(-1.07).to_string(), "-1.05");
    assert_eq!(o.from(-1.08).to_string(), "-1.10");
    assert_eq!(o.from(-1.09).to_string(), "-1.10");
}

#[test]
fn should_round_only_the_final_value_to_nearest_increment() {
    let o = CurrencyOpts::new().increment(0.05);

    assert_eq!((o.from(1.00) + 0.01 + 0.01 + 0.01).to_string(), "1.05");
    assert_eq!((o.from(1.00) - 0.01 - 0.01 - 0.01).to_string(), "0.95");
}

#[test]
fn should_not_modify_internal_values_when_rounding() {
    let o = CurrencyOpts::new().increment(0.05);

    assert_eq!((o.from(1.00) + 0.01).int_value(), 101);
    assert_eq!((o.from(1.00) + 0.01).value(), 1.01);
    assert_eq!((o.from(1.00) + 0.04).int_value(), 104);
    assert_eq!((o.from(1.00) + 0.04).value(), 1.04);
}

#[test]
fn should_allow_arbitrary_rounding_increments() {
    let c1 = CurrencyOpts::new().increment(0.1);
    let c2 = CurrencyOpts::new().increment(0.25);
    let c3 = CurrencyOpts::new().increment(5.).precision(0);

    assert_eq!(c1.from(1.06).to_string(), "1.10");
    assert_eq!(c1.from(-1.06).to_string(), "-1.10");
    assert_eq!(c2.from(1.17).to_string(), "1.25");
    assert_eq!(c2.from(-1.17).to_string(), "-1.25");
    assert_eq!(c3.from(117.).to_string(), "115");
    assert_eq!(c3.from(-117.).to_string(), "-115");
}

#[test]
fn should_handle_max_safe_integer() {
    assert_eq!(c(90071992547409.91).int_value(), 9007199254740991);
    assert_eq!(c(-90071992547409.91).int_value(), -9007199254740991);
}

// JS passes `undefined`; the Rust equivalent is an unparsable string.
#[test]
fn should_throw_exception_with_invalid_input() {
    let err = CurrencyOpts::new()
        .error_on_invalid(true)
        .parse("")
        .unwrap_err();
    assert!(matches!(err, CurrencyErr::ParseErr(_)));
}

#[test]
fn should_allow_creation_from_cents() {
    let c1 = CurrencyOpts::new().from_cents(true).precision(2);
    let c2 = CurrencyOpts::new().from_cents(true).precision(0);
    let c3 = CurrencyOpts::new().from_cents(true).precision(3);

    assert_eq!(c1.from(500.).to_string(), "5.00");
    assert_eq!(c1.parse("455").unwrap().to_string(), "4.55");
    assert_eq!(c2.from(500.).to_string(), "500");
    assert_eq!(c2.parse("455").unwrap().to_string(), "455");
    assert_eq!(c3.from(500.).to_string(), "0.500");
    assert_eq!(c3.parse("455").unwrap().to_string(), "0.455");
}

#[test]
fn should_parse_cents_from_a_number_when_using_from_cents_option() {
    let c1 = CurrencyOpts::new().from_cents(true).from(123.);
    let c2 = CurrencyOpts::new().from_cents(true).precision(0).from(123.);
    let c3 = CurrencyOpts::new().from_cents(true).precision(3).from(123.);

    assert_eq!(c1.value(), 1.23);
    assert_eq!(c1.int_value(), 123);
    assert_eq!(c2.value(), 123.);
    assert_eq!(c2.int_value(), 123);
    assert_eq!(c3.value(), 0.123);
    assert_eq!(c3.int_value(), 123);
}

#[test]
fn should_parse_cents_from_a_string_when_using_from_cents_option() {
    let c1 = CurrencyOpts::new().from_cents(true).parse("123").unwrap();
    let c2 = CurrencyOpts::new()
        .from_cents(true)
        .precision(0)
        .parse("123")
        .unwrap();
    let c3 = CurrencyOpts::new()
        .from_cents(true)
        .precision(3)
        .parse("123")
        .unwrap();

    assert_eq!(c1.value(), 1.23);
    assert_eq!(c1.int_value(), 123);
    assert_eq!(c2.value(), 123.);
    assert_eq!(c2.int_value(), 123);
    assert_eq!(c3.value(), 0.123);
    assert_eq!(c3.int_value(), 123);
}

#[test]
fn should_handle_add_with_from_cents_option() {
    let o = CurrencyOpts::new().from_cents(true);
    let c1 = o.from(12345.);
    let c2 = o.from(123.);

    assert_eq!((&c1 + 123.).value(), 124.68);
    assert_eq!((&c1 + &c2).value(), 124.68);
}

#[test]
fn should_handle_subtract_with_from_cents_option() {
    let o = CurrencyOpts::new().from_cents(true);
    let c1 = o.from(12345.);
    let c2 = o.from(123.);

    assert_eq!((&c1 - 123.).value(), 122.22);
    assert_eq!((&c1 - &c2).value(), 122.22);
}

#[test]
fn should_handle_multiply_with_from_cents_option() {
    let c1 = CurrencyOpts::new().from_cents(true).from(12345.);
    assert_eq!((c1 * 2.).value(), 246.90);
}

#[test]
fn should_handle_divide_with_from_cents_option() {
    let c1 = CurrencyOpts::new().from_cents(true).from(12345.);
    assert_eq!((c1 / 2.).value(), 61.73);
}

#[test]
fn should_handle_distribute_with_from_cents_option() {
    let parts = CurrencyOpts::new()
        .from_cents(true)
        .from(100.)
        .distribute(4);
    assert_eq!(values(&parts), [0.25, 0.25, 0.25, 0.25]);
}

#[test]
fn should_handle_fractional_cents() {
    let v = CurrencyOpts::new().from_cents(true).from(1234.56);

    assert_eq!(v.int_value(), 1235);
    assert_eq!(v.value(), 12.35);
}

mod regressions {
    use super::*;

    #[test]
    fn cents_keeps_sign() {
        assert_eq!(c(-1.23).cents(), -23);
        assert_eq!(c(-1.23).dollars(), -1);
    }

    #[test]
    fn divide_by_negative() {
        assert_eq!((c(10.) / -2.).value(), -5.);
    }

    #[test]
    fn multi_char_separator_not_reversed() {
        let v = CurrencyOpts::new().separator(", ").from(1234567.);
        assert_eq!(v.format(), "$1, 234, 567.00");
    }

    #[test]
    fn negative_zero_prints_as_zero() {
        assert_eq!(p("-0").to_string(), "0.00");
        assert_eq!(p("(0.00)").to_string(), "0.00");
    }

    #[test]
    fn display_ties_round_like_js_to_fixed() {
        let o = CurrencyOpts::new().precision(1).increment(0.25);
        assert_eq!(o.from(0.2).to_string(), "0.3");
        assert_eq!(o.from(-0.2).to_string(), "-0.3");
    }

    #[test]
    fn distribute_zero_is_empty() {
        assert!(c(1.).distribute(0).is_empty());
    }

    #[test]
    fn from_cents_assign_currency() {
        let o = CurrencyOpts::new().from_cents(true);
        let mut a = o.from(100.);
        a += o.from(50.);
        assert_eq!(a, 1.50);
        a -= &o.from(25.);
        assert_eq!(a, 1.25);
    }
}

mod dx {
    use super::*;

    #[test]
    fn compare_with_f64_and_currency() {
        let a = c(30.01);
        assert!(a > 30.);
        assert!(a < c(30.02));
        assert_eq!(c(1.23), CurrencyOpts::new().precision(3).from(1.23));
    }

    #[test]
    fn sum_owned_and_borrowed() {
        let items = vec![c(0.1), c(0.2), c(0.3)];
        let by_ref: Currency = items.iter().sum();
        let owned: Currency = items.into_iter().sum();

        assert_eq!(by_ref, 0.6);
        assert_eq!(owned, 0.6);
        assert_eq!(std::iter::empty::<Currency>().sum::<Currency>(), 0.);
    }

    #[test]
    fn sum_keeps_first_items_options() {
        let eur = CurrencyOpts::new().symbol("€");
        let sum: Currency = [eur.from(1.), c(2.)].iter().sum();
        assert_eq!(sum.format(), "€3.00");
    }

    #[test]
    fn ref_ops_do_not_consume() {
        let a = c(1.);
        let b = c(2.);
        let _ = &a + &b;
        let _ = &a - 1.;
        let _ = &a * 2.;
        let _ = &a / 2.;
        assert_eq!(a + b, 3.);
    }

    #[test]
    fn parse_entry_points() {
        assert_eq!("1.23".parse::<Currency>().unwrap(), 1.23);
        assert_eq!(Currency::try_from("1.23").unwrap(), 1.23);
    }

    #[test]
    fn error_works_with_question_mark() -> Result<(), Box<dyn std::error::Error>> {
        let strict = CurrencyOpts::new().error_on_invalid(true);
        assert_eq!(strict.parse("1.23")?, 1.23);

        let err = strict.parse("nope").unwrap_err();
        assert!(err.to_string().starts_with("invalid currency value"));
        Ok(())
    }

    #[test]
    fn format_with_does_not_change_value() {
        let a = c(1.23);
        let _ = a.format_with(|o| o.symbol("£"));
        assert_eq!(a.format(), "$1.23");
    }

    #[test]
    fn opts_fields_are_readable() {
        let o = CurrencyOpts::new().precision(3);
        assert_eq!(o.precision, 3);
        assert_eq!(o.from(1.).opts().precision, 3);
    }

    #[test]
    fn handles_huge_values() {
        for v in [i128::MAX as f64, i64::MAX as f64, i32::MAX as f64] {
            assert_eq!(c(v).value(), v);
            assert_eq!(c(-v).value(), -v);
        }
    }
}

mod parse_and_format {
    use super::*;

    #[test]
    fn parenthesized_positive() {
        let v = p("(123.45)");
        assert_eq!(v.value(), -123.45);
        assert_eq!(v.format(), "-$123.45");
    }

    #[test]
    fn parenthesized_with_symbol() {
        let v = p("$(1,234.56)");
        assert_eq!(v.value(), -1234.56);
        assert_eq!(v.format(), "-$1,234.56");
    }

    #[test]
    fn parenthesized_international() {
        let o = CurrencyOpts::new().separator(".").decimal(",").symbol("€");
        let v = o.parse("€(1.234,56)").unwrap();
        assert_eq!(v.value(), -1234.56);
        assert_eq!(v.format(), "-€1.234,56");
    }

    #[test]
    fn parenthesized_zero() {
        let v = p("(0.00)");
        assert_eq!(v.value(), 0.00);
        assert_eq!(v.format(), "$0.00");
    }

    #[test]
    fn strip_letters_and_symbols() {
        assert_eq!(p("abc$1x2y3z.45").value(), 123.45);
    }

    #[test]
    fn strip_mixed_unicode() {
        assert_eq!(p("¥€£1,234.56$").value(), 1234.56);
    }

    #[test]
    fn negative_sign_mid_string() {
        // only leading '-' is valid; mid-string dashes produce an unparseable float
        assert_eq!(p("1-2-3.45").value(), 0.);
    }

    #[test]
    fn only_negative_sign() {
        assert_eq!(p("-").value(), 0.);
    }

    #[test]
    fn comma_decimal_plain() {
        let v = CurrencyOpts::new().decimal(",").parse("12,34").unwrap();
        assert_eq!(v.value(), 12.34);
    }

    #[test]
    fn comma_decimal_with_space_separator() {
        let o = CurrencyOpts::new().separator(" ").decimal(",").symbol("€");
        let v = o.parse("1 234,56").unwrap();
        assert_eq!(v.value(), 1234.56);
        assert_eq!(v.format(), "€1 234,56");
    }

    #[test]
    fn comma_decimal_no_separator() {
        let v = CurrencyOpts::new()
            .separator("")
            .decimal(",")
            .parse("1234,56")
            .unwrap();
        assert_eq!(v.value(), 1234.56);
    }

    #[test]
    fn dot_separator_comma_decimal_millions() {
        let o = CurrencyOpts::new().separator(".").decimal(",");
        let v = o.parse("1.234.567,89").unwrap();
        assert_eq!(v.value(), 1234567.89);
        assert_eq!(v.format(), "$1.234.567,89");
    }

    #[test]
    fn multiple_decimal_separators() {
        // "1,234,56" with decimal="," → all "," → "." → "1.234.56" → parse fails → 0
        let v = CurrencyOpts::new().decimal(",").parse("1,234,56").unwrap();
        assert_eq!(v.value(), 0.);
    }

    #[test]
    fn format_no_separator() {
        let v = CurrencyOpts::new().separator("").from(1234567.89);
        assert_eq!(v.format(), "$1234567.89");
    }

    #[test]
    fn format_tiny_number() {
        assert_eq!(c(0.01).format(), "$0.01");
    }

    #[test]
    fn format_zero() {
        assert_eq!(c(0.0).format(), "$0.00");
    }

    #[test]
    fn format_negative_zero() {
        // -0.004 rounds to 0, so the positive pattern fires.
        assert_eq!(c(-0.004).format(), "$0.00");
    }

    #[test]
    fn format_exact_million() {
        assert_eq!(c(1_000_000.0).format(), "$1,000,000.00");
    }

    #[test]
    fn format_exact_billion() {
        assert_eq!(c(1_000_000_000.0).format(), "$1,000,000,000.00");
    }

    #[test]
    fn format_precision_zero() {
        let v = CurrencyOpts::new().precision(0).from(1234.56);
        assert_eq!(v.format(), "$1,235");
    }

    #[test]
    fn format_precision_four() {
        let v = CurrencyOpts::new().precision(4).from(1.2345);
        assert_eq!(v.format(), "$1.2345");
    }

    fn vedic(v: f64) -> String {
        CurrencyOpts::new().use_vedic(true).from(v).format()
    }

    #[test]
    fn vedic_thousands() {
        assert_eq!(vedic(1000.0), "$1,000.00");
    }

    #[test]
    fn vedic_lakh() {
        assert_eq!(vedic(100_000.0), "$1,00,000.00");
    }

    #[test]
    fn vedic_crore() {
        assert_eq!(vedic(10_000_000.0), "$1,00,00,000.00");
    }

    #[test]
    fn vedic_large() {
        assert_eq!(vedic(12_34_56_78_901.23), "$12,34,56,78,901.23");
    }

    #[test]
    fn vedic_three_digits() {
        assert_eq!(vedic(123.45), "$123.45");
    }

    #[test]
    fn vedic_two_digits() {
        assert_eq!(vedic(12.34), "$12.34");
    }

    #[test]
    fn vedic_negative() {
        assert_eq!(vedic(-10_00_000.0), "-$10,00,000.00");
    }

    #[test]
    fn parse_add_format_chain() {
        let mut v = p("$1,234.56");
        v += p("($100.00)");
        v += 50.01;
        v -= 0.01;
        assert_eq!(v.value(), 1184.56);
        assert_eq!(v.format(), "$1,184.56");
    }

    #[test]
    fn international_roundtrip() {
        let o = CurrencyOpts::new().separator(".").decimal(",").symbol("€");
        let v = o.parse("€1.234.567,89").unwrap();
        assert_eq!(v.value(), 1234567.89);
        assert_eq!(v.format(), "€1.234.567,89");

        let v2 = &v + &o.parse("0,11").unwrap();
        assert_eq!(v2.value(), 1234568.00);
        assert_eq!(v2.format(), "€1.234.568,00");
    }

    #[test]
    fn from_cents_with_formatted_string() {
        // "$1.23" as cents → stripped to "1.23" → 1 (rounded) → $0.01
        let v = CurrencyOpts::new().from_cents(true).parse("$1.23").unwrap();
        assert_eq!(v.value(), 0.01);
        assert_eq!(v.int_value(), 1);
    }
}
