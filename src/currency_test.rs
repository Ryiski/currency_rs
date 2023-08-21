use crate::Currency;
use crate::CurrencyErr;
use crate::CurrencyOpts;

#[test]
fn should_allow_numbers() {
    let c1 = Currency::new_float(1.23, None);

    assert_eq!(c1.value(), 1.23, "value is not 1.23'");
}

#[test]
fn should_allow_strings() {
    let c1 = Currency::new_string("1.23", None).unwrap();

    assert_eq!(c1.value(), 1.23, "currency.value() exists");
}

#[test]
fn should_accept_strings_with_symbols() {
    let c1 = Currency::new_string("$1234.56", None).unwrap();
    let c2 = Currency::new_string("£78.90", None).unwrap();

    assert_eq!(c1.value(), 1234.56, "value is not 1234.56");
    assert_eq!(c2.value(), 78.90, "value is not 78.90");
}

#[test]
fn should_allow_comma_delimited_strings() {
    let c1 = Currency::new_string("1,234,567.89", None).unwrap();

    assert_eq!(c1.value(), 1234567.89, "value is not 1234567.89");
}

#[test]
fn should_strip_invalid_characters() {
    let c1 = Currency::new_string("a1b2c3", None).unwrap();

    assert_eq!(c1.value(), 123., "value is not 123");
}

#[test]
fn should_default_to_0_with_invalid_strings() {
    let c1 = Currency::new_string("abc", None).unwrap();

    assert_eq!(c1.value(), 0., "value is not 0");
}

#[test]
fn should_return_value() {
    let c1 = Currency::new_float(1.23, None);

    assert_eq!(c1.value(), 1.23, "currency.value() exists");
}

#[test]
fn should_return_int_value() {
    let c1 = Currency::new_float(1.23, None);
    let c2 = Currency::new_float(1080.98, None);

    assert_eq!(c1.int_value(), 123., "intValue exists");
    assert_eq!(c2.int_value(), 108098., "intValue exists");
}

#[test]
fn should_allow_negative_strings() {
    let c1 = Currency::new_float(-12.34, None);

    assert_eq!(c1.value(), -12.34, "value is -12.34");
}

#[test]
fn should_add_floating_point_value() {
    let mut c1 = Currency::new_float(2.51, None);

    c1 += 0.01;

    assert_eq!(c1.value(), 2.52, "value equals decimal value 2.52");
    assert_ne!(c1.value(), 2.51 + 0.01, "does not equal 2.5199999999999996");
}

#[test]
fn should_subtract_floating_point() {
    let c1 = Currency::new_float(2.52, None) - 0.01;

    assert_eq!(c1.value(), 2.51, "value equals decimal value 2.51");
    assert_ne!(
        c1.value(),
        2.52 - 0.01,
        "value does not equal 2.5100000000000002"
    );
}

#[test]
fn should_round_half_up() {
    let c1 = Currency::new_float(17.955, None);
    let c2 = Currency::new_float(17.855, None);
    let c3 = Currency::new_float(17.455, None);
    let cs1 = Currency::new_string("17.955", None).unwrap();
    let cs2 = Currency::new_string("17.855", None).unwrap();
    let cs3 = Currency::new_string("17.455", None).unwrap();

    assert_eq!(c1.value(), 17.96, "value f64 rounds half up to 17.96");
    assert_eq!(c2.value(), 17.86, "value f64 rounds half up to 17.86");
    assert_eq!(c3.value(), 17.46, "value f64 rounds half up to 17.46");
    assert_eq!(cs1.value(), 17.96, "value string rounds half up to 17.96");
    assert_eq!(cs2.value(), 17.86, "value string rounds half up to 17.86");
    assert_eq!(cs3.value(), 17.46, "value string rounds half up to 17.46");
}

#[test]
fn should_round_negative_values_half_up() {
    let c1 = Currency::new_float(-17.955, None);
    let c2 = Currency::new_float(-17.855, None);
    let c3 = Currency::new_float(-17.455, None);
    let cs1 = Currency::new_string("-17.955", None).unwrap();
    let cs2 = Currency::new_string("-17.855", None).unwrap();
    let cs3 = Currency::new_string("-17.455", None).unwrap();

    assert_eq!(c1.value(), -17.96, "value f64 rounds half up to -17.96");
    assert_eq!(c2.value(), -17.86, "value f64 rounds half up to -17.86");
    assert_eq!(c3.value(), -17.46, "value f64 rounds half up to -17.46");
    assert_eq!(cs1.value(), -17.96, "value string rounds half up to -17.96");
    assert_eq!(cs2.value(), -17.86, "value string rounds half up to -17.86");
    assert_eq!(cs3.value(), -17.46, "value string rounds half up to -17.46");
}

#[test]
fn currency_multiplication() {
    let opts = Some(CurrencyOpts::new().set_precision(2).set_increment(0.01));
    let mut c1 = Currency::new_float(1.23, opts.clone());
    let mut c2 = Currency::new_float(0.1, opts);

    c1 *= 2.;
    c2 *= 0.2;

    assert_eq!(c1.value(), 2.46, "value is 2.46");
    assert_eq!(c2.value(), 0.02, "value equals 0.02");
    assert_ne!(
        c2.value(),
        0.1 * 0.2,
        "value does not equal 0.020000000000000004"
    );
}

#[test]
fn currency_multiplication_with_precision() {
    let opts = Some(CurrencyOpts::new().set_precision(3).set_increment(0.01));

    let mut c1 = Currency::new_float(1.369, opts);

    c1 *= 3.;

    assert_eq!(c1.value(), 4.107, "value is 4.107");
}

#[test]
fn currency_division() {
    let mut c1 = Currency::new_float(9.87, None);

    c1 /= 2.;

    assert_eq!(c1.value(), 4.94, "value is 4.94");
}

#[test]
fn currency_division_with_precision() {
    let opts = Some(CurrencyOpts::new().set_precision(3));

    let mut c1 = Currency::new_float(4.107, opts.clone());

    c1 /= 3.;

    assert_eq!(c1.value(), 1.369, "value is  1.369");
}

#[test]
fn should_parse_negative_values() {
    let pos = Currency::new_float(1.23, None);
    let neg = Currency::new_float(-1.23, None);
    let distribute = neg.clone().distribute(4);

    let mut total = 0.;

    let mut i = 0;

    while i < distribute.len() {
        total += distribute[i].value();
        i += 1;
    }

    assert_eq!(neg.value(), -1.23, "value is -1.23");
    assert_eq!((pos - 2.01).value(), -0.78, "value is -0.78");
    assert_eq!(
        distribute[0].value(),
        -0.31,
        "value, first distributed value is not -0.31"
    );
    assert_eq!(total, -1.23, "value sum is -1.23");
}

#[test]
fn should_create_equal_distribution() {
    let values = Currency::new_float(1., None).distribute(4);

    let mut real_values: Vec<f64> = vec![];
    let mut i = 0;

    while i < values.len() {
        real_values.push(values[i].value());
        i += 1;
    }

    assert_eq!(values.len(), 4, "values creates 4 distributed items");
    assert_eq!(
        real_values,
        [0.25, 0.25, 0.25, 0.25],
        "all distributed items are equal"
    );
}

#[test]
fn should_use_source_formatting_for_mixed_currency_formats() {
    let opts = Some(CurrencyOpts::new().set_separator(" ").set_decimal(","));

    let c1 = Currency::new_string("1,234.56", None).unwrap();
    let c2 = Currency::new_string("'1 234,56", opts).unwrap();

    assert_eq!((c1.clone() + c2.value()).format(), "$2,469.12");
    assert_eq!((c2 + c1.value()).format(), "$2 469,12");
}

#[test]
fn should_parse_international_values() {
    let opts = Some(CurrencyOpts::new().set_separator(".").set_decimal(","));

    let c = |v| Currency::new_string(v, opts.clone()).unwrap();

    assert_eq!(c("1,23").value(), 1.23, "value is not 1.23");
    assert_eq!(c("1.000,00").value(), 1000.00, "value is not 1,000.00");
    assert_eq!(
        c("1.000.000,00").value(),
        1000000.00,
        "value is not 1,000,000.00"
    );
}

#[test]
fn should_throw_exception_with_invalid_input() {
    let opts = Some(CurrencyOpts::new().set_error_on_invalid(true));
    let value_err = Currency::new_string("abc", opts).unwrap_err();

    assert!(matches!(value_err, CurrencyErr::ParseErr(_)));
}

#[test]
fn should_return_0_currency_with_invalid_input() {
    let value = Currency::new_string("abc", None).unwrap();

    assert_eq!(value.value(), 0., "value is not 0.00");
}

#[test]
fn should_allow_currency() {
    let mut value = Currency::new_cur(Currency::new_float(1.23, None), None);

    value += 0.02;

    assert_eq!(value.value(), 1.25, "value is not 1.25");
}

#[test]
fn should_create_non_equal_distribution_with_pennies() {
    let values = Currency::new_float(1.01, None).distribute(4);

    assert_eq!(values[0].value(), 0.26, "first value is not 0.26");
    assert_eq!(values[1].value(), 0.25, "next value is not 0.25");

    let mut total = 0.;
    let mut i = 0;

    while i < values.len() {
        total += values[i].value();
        i += 1;
    }

    assert_eq!(total, 1.01, "sum of values matches our original amount");
}

#[test]
fn should_create_non_equal_distribution_with_a_negative_penny() {
    let values = Currency::new_float(-0.01, None).distribute(2);

    assert_eq!(values[0].value(), -0.01, "first value is not -0.01");
    assert_eq!(values[1].value(), 0., "second value is not 0");

    let mut total = 0.;
    let mut i = 0;

    while i < values.len() {
        total += values[i].value();
        i += 1;
    }

    assert_eq!(total, -0.01, "sum of values matches our original amount");
}

#[test]
fn should_get_dollar_value() {
    let value = Currency::new_float(1.23, None);

    assert_eq!((value.clone() + 2.).dollars(), 3, "is dollar amount");
    assert_eq!((value.clone() + 0.8).dollars(), 2, "is dollar amount");
    assert_eq!((value - 3.).dollars(), -1, "is negative dollar amount");
}

#[test]
fn should_get_cent_value() {
    let value = Currency::new_float(1.23, None);
    assert_eq!(value.cents(), 23, "is cent amount");
    assert_eq!((value + 0.31).cents(), 54, "is cent amount");
}

#[test]
fn should_support_different_precision_values() {
    let opts = Some(CurrencyOpts::new().set_precision(3));

    let opts2 = Some(CurrencyOpts::new().set_precision(0).set_symbol("¥"));

    let c1 = Currency::new_float(1.234, opts);
    let c2 = Currency::new_float(1.234, opts2);

    assert_eq!(c1.value(), 1.234);
    assert_eq!(c2.value(), 1.);
    assert_eq!(c1.int_value(), 1234.);
    assert_eq!(c2.int_value(), 1.);
    assert_eq!((c1.clone() + 4.567).value(), 5.801);
    assert_eq!((c2.clone() + 4.567).value(), 6.);
    assert_eq!((c1.clone() - 4.567).value(), -3.333);
    assert_eq!((c2.clone() - 4.567).value(), -4.);
    assert_eq!(c1.cents(), 234);
    assert_eq!(c2.cents(), 0);
    assert_eq!(c1.format(), "$1.234");
    assert_eq!(c2.format(), "¥1");

    let c1_distribute: Vec<f64> = c1.distribute(4).iter().map(|x| x.value()).collect();
    let c2_distribute: Vec<f64> = c2.distribute(4).iter().map(|x| x.value()).collect();

    assert_eq!(c1_distribute, [0.309, 0.309, 0.308, 0.308]);
    assert_eq!(c2_distribute, [1., 0., 0., 0.]);
}

#[test]
fn should_use_source_precision_for_arithmetic_with_different_precisions() {
    let opts = Some(CurrencyOpts::new().set_precision(3));

    let c1 = Currency::new_float(1.23, None);
    let c2 = Currency::new_float(1.239, opts);

    assert_eq!((c1.clone() + c2.value()).value(), 2.47);
    assert_eq!((c2 + c1.value()).value(), 2.469);
}

#[test]
fn should_default_rounding_when_parsing() {
    let round1 = Currency::new_float(1.2349, None);
    let round2 = Currency::new_float(5.6789, None);
    let multiply = Currency::new_float(10.00, None);
    let divide = Currency::new_float(0.01, None);

    assert_eq!(round1.value(), 1.23, "value is not rounded to nearest cent");
    assert_eq!(round2.value(), 5.68, "value is not rounded to nearest cent");
    assert_eq!(
        (multiply * 0.001).value(),
        0.01,
        "multiply value is not not rounded"
    );
    assert_eq!(
        (divide / 0.001).value(),
        10.,
        "divide value is not not rounded"
    );
}

#[test]
fn should_have_int_value_and_real_value() {
    let value1 = Currency::new_float(2.51, None) + 0.01;
    let value2 = Currency::new_float(2.52, None) - 0.01;

    assert_eq!(value1.value(), 2.52, "real value is not 2.52");
    assert_eq!(value1.int_value(), 252., "int value is not 252");
    assert_eq!(value2.value(), 2.51, "real value is not 2.51");
    assert_eq!(value2.int_value(), 251., "int value is not 251");
}

#[test]
fn should_format_value_using_defaults() {
    let opts = CurrencyOpts::new().set_precision(4);

    let mut value1 = Currency::new_float(1.23, None);
    let mut value2 = Currency::new_float(1234.56, None);
    let mut value3 = Currency::new_float(1234567.89, None);
    let mut value4 = Currency::new_float(1234567.8912, Some(opts.clone()));
    let mut value5 = Currency::new_float(1234567., Some(opts.set_precision(0)));

    assert_eq!(value1.format(), "$1.23", "value is not \"$1.23\"");
    assert_eq!(value2.format(), "$1,234.56", "value is not \"$1,234.56\"");
    assert_eq!(
        value3.format(),
        "$1,234,567.89",
        "value is not \"$1,234,567.89\""
    );
    assert_eq!(
        value4.format(),
        "$1,234,567.8912",
        "value is not \"$1,234,567.8912\""
    );
    assert_eq!(value5.format(), "$1,234,567", "value is not \"$1,234,567\"");

    value1 *= -1.;

    assert_eq!(value1.format(), "-$1.23", "value is not \"-$1.23\"");

    value2 *= -1.;

    assert_eq!(value2.format(), "-$1,234.56", "value is not \"-$1,234.56\"");

    value3 *= -1.;

    assert_eq!(
        value3.format(),
        "-$1,234,567.89",
        "value is not \"-$1,234,567.89\""
    );

    value4 *= -1.;

    assert_eq!(
        value4.format(),
        "-$1,234,567.8912",
        "value is not \"-$1,234,567.8912\""
    );

    value5 *= -1.;

    assert_eq!(
        value5.format(),
        "-$1,234,567",
        "value is not \"-$1,234,567\""
    );
}

#[test]
fn should_format_value_using_international() {
    let opts = Some(CurrencyOpts::new().set_separator(".").set_decimal(","));

    let c = |v: f64| -> Currency { Currency::new_float(v, opts.clone()) };

    assert_eq!(c(1.23).format(), "$1,23", "value is not \"$1,23\"");
    assert_eq!(
        c(1000.00).format(),
        "$1.000,00",
        "value is not \"$1.000,00\""
    );
    assert_eq!(
        c(1000000.00).format(),
        "$1.000.000,00",
        "value is not \"$1.000.000,00\""
    );
}

#[test]
fn should_format_using_patterns() {
    let opts = CurrencyOpts::new().set_pattern("# !");

    let opts2 = Some(opts.clone().set_precision(4));
    let opts3 = Some(opts.clone().set_precision(0));

    let value1 = Currency::new_float(1.23, Some(opts.clone()));
    let value2 = Currency::new_float(1234.56, Some(opts.clone()));
    let value3 = Currency::new_float(1234567.89, Some(opts));
    let value4 = Currency::new_float(1234567.8912, opts2);
    let value5 = Currency::new_float(1234567., opts3);

    assert_eq!(value1.format(), "1.23 $", "value is not \"1.23 $\"");
    assert_eq!(value2.format(), "1,234.56 $", "value is not \"1,234.56 $\"");
    assert_eq!(
        value3.format(),
        "1,234,567.89 $",
        "value is not \"1,234,567.89 $\""
    );
    assert_eq!(
        value4.format(),
        "1,234,567.8912 $",
        "value is not \"1,234,567.8912 $\""
    );
    assert_eq!(
        value5.format(),
        "1,234,567 $",
        "value is not \"1,234,567 $\""
    );
}

#[test]
fn should_format_using_negative_patterns() {
    let opts = Some(CurrencyOpts::new().set_negative_pattern("! (#)"));

    let opts2 = Some(
        CurrencyOpts::new()
            .set_precision(4)
            .set_negative_pattern("! (#)"),
    );
    let opts3 = Some(
        CurrencyOpts::new()
            .set_precision(0)
            .set_negative_pattern("! (#)"),
    );

    let value1 = Currency::new_float(-1.23, opts.clone());
    let value2 = Currency::new_float(-1234.56, opts.clone());
    let value3 = Currency::new_float(-1234567.89, opts);
    let value4 = Currency::new_float(-1234567.8912, opts2);
    let value5 = Currency::new_float(-1234567., opts3);

    assert_eq!(value1.format(), "$ (1.23)", "value is not \"$ (1.23)\"");
    assert_eq!(
        value2.format(),
        "$ (1,234.56)",
        "value is not \"$ (1,234.56)\""
    );
    assert_eq!(
        value3.format(),
        "$ (1,234,567.89)",
        "value is not \"$ (1,234,567.89)\""
    );
    assert_eq!(
        value4.format(),
        "$ (1,234,567.8912)",
        "value is not \"$ (1,234,567.8912)\""
    );
    assert_eq!(
        value5.format(),
        "$ (1,234,567)",
        "value is not \"$ (1,234,567)\""
    );
}

#[test]
fn should_format_with_symbol() {
    let opts = Some(CurrencyOpts::new().set_pattern("!#"));

    let c1 = Currency::new_float(1.23, opts);

    assert_eq!(c1.format(), "$1.23", "value is not \"$1.23\"");
}

#[test]
fn should_format_without_symbol() {
    let opts = Some(CurrencyOpts::new().set_pattern("#"));

    let c1 = Currency::new_float(1.23, opts);

    assert_eq!(c1.format(), "1.23", "value is not \"1.23\"");
}

#[test]
fn should_format_with_international_symbol() {
    let opts = CurrencyOpts::new().set_symbol("£");

    let c1 = Currency::new_float(1.23, Some(opts.clone()));
    let c2 = Currency::new_float(1.23, Some(opts.set_symbol("¥")));

    assert_eq!(c1.format(), "£1.23", "value is not \"£1.23\"");
    assert_eq!(c2.format(), "¥1.23", "value is not \"¥1.23\"");
}

#[test]
fn should_round_down_to_nearest_value_when_using_increments() {
    let opts = Some(CurrencyOpts::new().set_increment(0.05));

    let c = |v| -> Currency { Currency::new_float(v, opts.clone()) };

    assert_eq!(c(1.01).to_string(), "1.00", "value is not rounded to 1.00");
    assert_eq!(c(1.02).to_string(), "1.00", "value is not rounded to 1.00");
    assert_eq!(c(1.06).to_string(), "1.05", "value is not rounded to 1.05");
    assert_eq!(c(1.07).to_string(), "1.05", "value is not rounded to 1.05");
    assert_eq!(
        c(1000.01).to_string(),
        "1000.00",
        "value is not rounded to 1000.00"
    );
    assert_eq!(
        c(10000.01).to_string(),
        "10000.00",
        "value is not rounded to 10000.00"
    );
    assert_eq!(
        c(100000.01).to_string(),
        "100000.00",
        "value is not rounded to 100000.00"
    );
    assert_eq!(
        c(1000000.01).to_string(),
        "1000000.00",
        "value is not rounded to 1000000.00"
    );
}

#[test]
fn should_round_up_to_nearest_value_when_using_increments() {
    let opts = Some(CurrencyOpts::new().set_increment(0.05));

    let c = |v| -> Currency { Currency::new_float(v, opts) };

    assert_eq!(
        c.clone()(1.03).to_string(),
        "1.05",
        "value is not rounded to 1.05"
    );
    assert_eq!(
        c.clone()(1.04).to_string(),
        "1.05",
        "value is not rounded to 1.05"
    );
    assert_eq!(
        c.clone()(1.08).to_string(),
        "1.10",
        "value is not rounded to 1.10"
    );
    assert_eq!(
        c.clone()(1.09).to_string(),
        "1.10",
        "value is not rounded to 1.10"
    );
    assert_eq!(
        c.clone()(1000.09).to_string(),
        "1000.10",
        "value is not rounded to 1000.10"
    );
    assert_eq!(
        c.clone()(10000.09).to_string(),
        "10000.10",
        "value is not rounded to 10000.10"
    );
    assert_eq!(
        c.clone()(100000.09).to_string(),
        "100000.10",
        "value is not rounded to 100000.10"
    );
    assert_eq!(
        c(1000000.09).to_string(),
        "1000000.10",
        "value is not rounded to 1000000.10"
    );
}

#[test]
fn should_handle_negative_rounding_when_using_increments() {
    let opts = Some(CurrencyOpts::new().set_increment(0.05));

    let c = |v| -> Currency { Currency::new_float(v, opts) };

    assert_eq!(
        c.clone()(-1.01).to_string(),
        "-1.00",
        "value is not rounded to -1.00"
    );
    assert_eq!(
        c.clone()(-1.02).to_string(),
        "-1.00",
        "value is not rounded to -1.00"
    );
    assert_eq!(
        c.clone()(-1.03).to_string(),
        "-1.05",
        "value is not rounded to -1.05"
    );
    assert_eq!(
        c.clone()(-1.04).to_string(),
        "-1.05",
        "value is not rounded to -1.05"
    );
    assert_eq!(
        c.clone()(-1.06).to_string(),
        "-1.05",
        "value is not rounded to -1.05"
    );
    assert_eq!(
        c.clone()(-1.07).to_string(),
        "-1.05",
        "value is not rounded to -1.05"
    );
    assert_eq!(
        c.clone()(-1.08).to_string(),
        "-1.10",
        "value is not rounded to -1.10"
    );
    assert_eq!(
        c(-1.09).to_string(),
        "-1.10",
        "value is not rounded to -1.10"
    );
}

#[test]
fn should_round_only_the_final_value_to_nearest_increment() {
    let opts = Some(CurrencyOpts::new().set_increment(0.05));

    let c = |v| -> Currency { Currency::new_float(v, opts) };

    let mut c1 = c.clone()(1.00);

    c1 += 0.01;
    c1 += 0.01;
    c1 += 0.01;

    assert_eq!(c1.to_string(), "1.05", "value is not rounded to 1.05");

    let mut c2 = c(1.00);

    c2 -= 0.01;
    c2 -= 0.01;
    c2 -= 0.01;

    assert_eq!(c2.to_string(), "0.95", "value is not rounded to 0.95");
}

#[test]
fn should_not_modify_internal_values_when_rounding() {
    let opts = Some(CurrencyOpts::new().set_increment(0.05));

    let c = |v| -> Currency { Currency::new_float(v, opts) };
    let c1 = c(1.00);
    assert_eq!(
        (c1.clone() + 0.01).int_value(),
        101.,
        "int_value is not to 101"
    );
    assert_eq!((c1.clone() + 0.01).value(), 1.01, "value is not to 1.01");
    assert_eq!(
        (c1.clone() + 0.04).int_value(),
        104.,
        "intValue is not to 104"
    );
    assert_eq!((c1 + 0.04).value(), 1.04, "value is not to 1.04");
}

#[test]
fn should_allow_arbitrary_rounding_increments() {
    let opts = Some(CurrencyOpts::new().set_symbol("").set_increment(0.1));

    let opts2 = Some(CurrencyOpts::new().set_symbol("").set_increment(0.25));

    let opts3 = Some(
        CurrencyOpts::new()
            .set_symbol("")
            .set_precision(0)
            .set_increment(5.),
    );

    let c1 = |v| -> Currency { Currency::new_float(v, opts) };
    let c2 = |v| -> Currency { Currency::new_float(v, opts2) };
    let c3 = |v| -> Currency { Currency::new_float(v, opts3) };

    assert_eq!(
        c1.clone()(1.06).format(),
        "1.10",
        "value is not rounded to 1.10"
    );
    assert_eq!(c1(-1.06).format(), "-1.10", "value is not rounded to -1.10");
    assert_eq!(
        c2.clone()(1.17).format(),
        "1.25",
        "value is not rounded to 1.25"
    );
    assert_eq!(c2(-1.17).format(), "-1.25", "value is not rounded to -1.25");
    assert_eq!(
        c3.clone()(117.).format(),
        "115",
        "value is not rounded to 120"
    );
    assert_eq!(c3(-117.).format(), "-115", "value is not rounded to 120");
}

#[test]
fn should_handle_max_safe_integer() {
    let max1 = Currency::new_float(i128::MAX as f64, None);
    let max2 = Currency::new_float(-i128::MAX as f64, None);
    let max3 = Currency::new_float(i64::MAX as f64, None);
    let max4 = Currency::new_float(-i64::MAX as f64, None);
    let max5 = Currency::new_float(i32::MAX as f64, None);
    let max6 = Currency::new_float(-i32::MAX as f64, None);

    assert_eq!(
        max1.value(),
        i128::MAX as f64,
        "currency does not handle i128 correctly"
    );
    assert_eq!(
        max2.value(),
        -i128::MAX as f64,
        "currency does not handle -i128 correctly"
    );
    assert_eq!(
        max3.value(),
        i64::MAX as f64,
        "currency does not handle i64 correctly"
    );
    assert_eq!(
        max4.value(),
        -i64::MAX as f64,
        "currency does not handle -i64 correctly"
    );
    assert_eq!(
        max5.value(),
        i32::MAX as f64,
        "currency does not handle i32 correctly"
    );
    assert_eq!(
        max6.value(),
        -i32::MAX as f64,
        "currency does not handle -i32 correctly"
    );
}

#[test]
fn should_allow_creation_from_cents() {
    let opts = Some(CurrencyOpts::new().set_precision(2).set_from_cents(true));
    let opts2 = Some(CurrencyOpts::new().set_precision(0).set_from_cents(true));
    let opts3 = Some(CurrencyOpts::new().set_precision(3).set_from_cents(true));

    let c1 = |v| Currency::new_float(v, opts.clone());
    let c2 = |v| Currency::new_float(v, opts2.clone());
    let c3 = |v| Currency::new_float(v, opts3.clone());

    assert_eq!(
        c1(500.).to_string(),
        "5.00",
        "value is not parsed from cents to 5.00"
    );
    assert_eq!(
        c1(455.).to_string(),
        "4.55",
        "value is not parsed from a string to cents"
    );
    assert_eq!(
        c2(500.).to_string(),
        "500",
        "value is not parsed from cents to 5.00"
    );
    assert_eq!(
        c2(455.).to_string(),
        "455",
        "value is not parsed from a string to cents"
    );
    assert_eq!(
        c3(500.).to_string(),
        "0.500",
        "value is not parsed from cents to 5.00"
    );
    assert_eq!(
        c3(455.).to_string(),
        "0.455",
        "value is not parsed from a string to cents"
    );
}

#[test]
fn should_parse_cents_from_a_number_when_using_from_cents_option() {
    let opts = Some(CurrencyOpts::new().set_precision(2).set_from_cents(true));
    let opts2 = Some(CurrencyOpts::new().set_precision(0).set_from_cents(true));
    let opts3 = Some(CurrencyOpts::new().set_precision(3).set_from_cents(true));

    let c1 = Currency::new_float(123., opts);
    let c2 = Currency::new_float(123., opts2);
    let c3 = Currency::new_float(123., opts3);

    assert_eq!(c1.value(), 1.23);
    assert_eq!(c1.int_value(), 123.);
    assert_eq!(c2.value(), 123.);
    assert_eq!(c2.int_value(), 123.);
    assert_eq!(c3.value(), 0.123);
    assert_eq!(c3.int_value(), 123.);
}

#[test]
fn should_parse_cents_from_a_string_when_using_from_cents_option() {
    let opts = Some(CurrencyOpts::new().set_from_cents(true));
    let opts2 = Some(CurrencyOpts::new().set_from_cents(true).set_precision(0));
    let opts3 = Some(CurrencyOpts::new().set_from_cents(true).set_precision(3));

    let c1 = Currency::new_string("123", opts).unwrap();
    let c2 = Currency::new_string("123", opts2).unwrap();
    let c3 = Currency::new_string("123", opts3).unwrap();

    assert_eq!(c1.value(), 1.23);
    assert_eq!(c1.int_value(), 123.);
    assert_eq!(c2.value(), 123.);
    assert_eq!(c2.int_value(), 123.);
    assert_eq!(c3.value(), 0.123);
    assert_eq!(c3.int_value(), 123.);
}

#[test]
fn should_handle_add_with_from_cents_option() {
    let opts = Some(CurrencyOpts::new().set_from_cents(true));

    let c1 = Currency::new_float(12345., opts.clone());
    let c2 = Currency::new_float(123., opts);

    assert_eq!((c1.clone() + 123.).value(), 124.68);
    assert_eq!((c1 + c2.int_value()).value(), 124.68);
}

#[test]
fn should_handle_subtract_with_from_cents_option() {
    let opts = Some(CurrencyOpts::new().set_from_cents(true));

    let c1 = Currency::new_float(12345., opts.clone());
    let c2 = Currency::new_float(12345., opts);

    assert_eq!((c1.clone() - 123.).value(), 122.22);
    assert_eq!((c1 - c2).value(), 122.22);
}

#[test]
fn should_handle_multiply_with_from_cents_option() {
    let opts = Some(CurrencyOpts::new().set_from_cents(true));

    let mut c1 = Currency::new_float(12345., opts);

    c1 *= 2.;

    assert_eq!(c1.value(), 246.90);
}

#[test]
fn should_handle_divide_with_from_cents_option() {
    let opts = Some(CurrencyOpts::new().set_from_cents(true));

    let c1 = Currency::new_float(12345., opts) / 2.;

    assert_eq!(c1.value(), 61.73);
}

#[test]
fn should_handle_distribute_with_from_cents_option() {
    let opts = Some(CurrencyOpts::new().set_from_cents(true));

    let distributions = Currency::new_float(100., opts).distribute(4);
    let values: Vec<f64> = distributions.iter().map(|c| c.value()).collect();

    assert_eq!(values, [0.25, 0.25, 0.25, 0.25]);
}

#[test]
fn should_handle_fractional_cents() {
    let opts = Some(CurrencyOpts::new().set_from_cents(true));

    let c = Currency::new_float(1234.56, opts);

    assert_eq!(c.int_value(), 1235.);
    assert_eq!(c.value(), 12.35);
}

#[test]
fn should_format_vedic_groupings() {
    let opts = Some(CurrencyOpts::new().set_use_vedic(true));

    let opts2 = Some(CurrencyOpts::new().set_precision(4).set_use_vedic(true));

    let c1 = |v| Currency::new_float(v, opts);
    let c2 = |v| Currency::new_float(v, opts2);

    assert_eq!(c1.clone()(1.23).format(), "$1.23", "value is not $1.23");
    assert_eq!(
        c1.clone()(1000.00).format(),
        "$1,000.00",
        "value is not $1,000"
    );
    assert_eq!(
        c1.clone()(100000.00).format(),
        "$1,00,000.00",
        "value is not $1,00,000,00"
    );
    assert_eq!(
        c1(1000000.00).format(),
        "$10,00,000.00",
        "value is not $10,00,000,00"
    );
    assert_eq!(
        c2(1234567.8912).format(),
        "$12,34,567.8912",
        "value is not $12,34,567.8912"
    );
}
