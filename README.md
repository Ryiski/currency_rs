<div align="center" markdown="1">

![currency_rs logo](https://dl.dropbox.com/s/ywaqfezvlf3n33e/logo.png?dl=1)

# currency_rs

</div>

_currency_rs_ is library for working with currency values. it was inspired by [currency.js](https://github.com/scurker/currency.js) which was built to work around floating point issues in javascript.

currency_rs works with values as integers behind the scenes, resolving some of the most basic precision problems.

```rust
2.51 + 0.01;                                          // 2.5199999999999996
Currency::new_float(2.51, None).add(0.01).value();      // 2.52

2.52 - 0.01;                                          // 2.5100000000000002
Currency::new_float(2.52, None).subtract(0.01).value(); // 2.51
```

This should work for most _reasonable_ values of currencies. As long as your currency values are less than 2<sup>53</sup> (in cents) or 90,071,992,547,409.91 you should be okay.

### Installation

```sh
[dependencies]
currency_rs = "x.y.z"
```

### Usage

You can create a currency object from float, strings, or the currency object itself as values.

```rust
Currency::new_float(123., None).to_string(); // 123.00
Currency::new_float(1.23, None).to_string(); // 1.23
Currency::new_string("1.23", None).unwrap().to_string(); // 1.23
Currency::new_string("$12.30", None).unwrap().to_string(); // 12.30

let value = Currency::new_string("123.45", None).unwrap();
Currency::new_cur(value, None).to_string(); // 123.45
```

Currency accepts decimal values (i.e. `1.23`) with a default precision of 2, but can accept a minor currency unit (e.g. cents in a dollar). This will respect the precision option when parsing.

```rust
let opt = CurrencyOpts::new().set_from_cents(true);

Currency::new_float(123., Some(opt.clone())).to_string(); // 1.23

Currency::new_string("123", Some(opt))
    .unwrap()
    .to_string();                                         // 1.23

let opt = CurrencyOpts::new()
    .set_from_cents(true)
    .set_precision(0.);

Currency::new_float(123., Some(opt)).to_string();         // 123

let opt = CurrencyOpts::new()
    .set_from_cents(true)
    .set_precision(3.);

Currency::new_float(123., Some(opt)).to_string();         // 1.23
```

There's various arithmetic methods that help take the guesswork out of trying to resolve floating point problems.

```rust
Currency::new_float(123.5, None).add(0.23).value();    // 123.73
Currency::new_float(5.0, None).subtract(0.5).value();  // 4.50
Currency::new_float(45.25, None).multiply(3.).value(); // 135.75
Currency::new_float(1.12, None)
    .distribute(5)
    .iter()
    .map(|x| x.value())
    .collect::<Vec<f64>>();                          // [0.23, 0.23, 0.22, 0.22, 0.22]
```

There's even a built in formatter that will automatically place comma delimiters in the right place.

```rust
Currency::new_string("2,573,693.75", None)
    .unwrap()
    .add_string("100,275.50")
    .unwrap()
    .format();              // "$2,673,969.25"

Currency::new_string("1,237.72", None)
    .unwrap()
    .subtract(300.)
    .format();              // "$937.72"
```

You can also change the format, localizing the decimal and/or delimiter to your locale.

```rust
fn euro(value: &str) -> Currency {
    let otp = CurrencyOpts::new()
        .set_symbol("€")
        .set_separator(".")
        .set_decimal(",");

    Currency::new_string(value, Some(otp)).unwrap()
}

euro("2.573.693,75")
    .add_string("100.275,50")
    .unwrap()
    .format();                   // "€2.673.969,25"

euro("1.237,72")
.subtract(300.)
.format();                      // "€937,72"
```

### Options

_currency_rs_ comes with its own set of default options conforming to USD. You can customize these according to your locale.

`symbol` _default_: `$`<br/>
Currency symbol included when calling `currency.format()`.

`separator` _default_: `,`<br/>
Separator dividing the number groups when calling `currency.format()`.

`decimal` _default_: `.`<br/>
Decimal used when calling `currency.format()`.

`precision` _default_: `2`<br/>
Number of decimal places to store as cents.

`pattern` _default_: `!#`<br/>
Allows you to customize the format pattern using `!` as replacement for the currency symbol and `#` as replacement for the currency amount.

`negative_pattern` _default_: `-!#`<br/>
Allows you to customize the negative format pattern using `!` as replacement for the currency symbol and `#` as replacement for the currency amount.

`error_on_invalid` _default_: `false`<br/>
If an invalid value such as `abc` is passed in to `Currency::new_string`, will throw an error.

`increment` _default_: `null`<br/>
When implementing a currency that implements rounding, setting the increment value will allow you to set the closest increment to round the display value to.

```rust
let otp = CurrencyOpts::new()
    .set_increment(0.05);

Currency::new_float(1.48, Some(otp)).format(); // "$1.50"
```

`use_vedic` _default_: `false`<br/>
Formats number groupings using the Indian Numbering System, i.e. `10,00,000.00`

`from_cents` _default_: `false`<br/>
Parse the amount value as a minor currency unit (e.g. cents in a dollar) instead of dollars.

### Internationalization Examples

```rust
let otp = CurrencyOpts::new()
    .set_separator(" ")
    .set_decimal(",")
    .set_symbol("€");

Currency::new_float(1234.45, Some(otp)).format(); // "€1 234,45"
```

If you need to work with multiple currency values, the easiest way is to setup factory functions with your required currency settings.

```rust
fn usd(value: f64) -> Currency {
    let otp = CurrencyOpts::new()
        .set_symbol("$")
        .set_precision(2.);

    Currency::new_float(value, Some(otp))
}

fn jpy(value: f64) -> Currency {
    let otp = CurrencyOpts::new()
        .set_symbol("¥")
        .set_precision(0.);

    Currency::new_float(value, Some(otp))
}

fn gas(value: f64) -> Currency {
     let otp = CurrencyOpts::new()
        .set_precision(3.);

    Currency::new_float(value, Some(otp))
}

usd(1234.56).format(); // "$1,234.56"
jpy(1234.56).format(); // "¥1,235"
gas(1234.56).format(); // "$1,234.560"
```

## License

[MIT](/license)
