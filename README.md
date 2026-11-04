# currency_rs

_currency_rs_ is library for working with currency values. it is inspired by [currency.js](https://github.com/scurker/currency.js) which was built to work around floating point issues in javascript.

currency_rs works with values as integers behind the scenes, resolving some of the most basic precision problems.

```rust
2.51 + 0.01;                              // 2.5199999999999996
(Currency::from(2.51) + 0.01).value();    // 2.52

2.52 - 0.01;                              // 2.5100000000000002
(Currency::from(2.52) - 0.01).value();    // 2.51
```

This should work for most _reasonable_ values of currencies. As long as your currency values are less than 2<sup>53</sup> (in cents) or 90,071,992,547,409.91 you should be okay.

## Installation

```toml
[dependencies]
currency_rs = "2"
```

Optional feature: `serde` serializes a `Currency` as its plain number, like currency.js `toJSON`.

## Usage

Create a currency from a float or a string. Strings can contain symbols, separators and parentheses for negatives.

```rust
use currency_rs::{Currency, CurrencyOpts};

Currency::from(123.).to_string();                     // "123.00"
Currency::from(1.23).to_string();                     // "1.23"
"1.23".parse::<Currency>()?.to_string();              // "1.23"
"$12.30".parse::<Currency>()?.to_string();            // "12.30"
"($5.24)".parse::<Currency>()?.value();               // -5.24
```

`CurrencyErr` implements `std::error::Error`, so `?` works with `Box<dyn Error>`, `anyhow` and friends.

For anything other than the USD defaults, build a `CurrencyOpts` once and create values from it with `from` (numbers) or `parse` (strings):

```rust
let eur = CurrencyOpts::new().symbol("€").separator(".").decimal(",");

eur.from(1234.5).format();              // "€1.234,50"
eur.parse("2.573.693,75")?.format();    // "€2.573.693,75"
```

Currency accepts decimal values (i.e. `1.23`) with a default precision of 2, but can accept a minor currency unit (e.g. cents in a dollar). This will respect the precision option when parsing.

```rust
let cents = CurrencyOpts::new().from_cents(true);

cents.from(123.).to_string();                         // "1.23"
cents.parse("123")?.to_string();                      // "1.23"
cents.clone().precision(0).from(123.).to_string();    // "123"
cents.clone().precision(3).from(123.).to_string();    // "0.123"
```

## Arithmetic

`+`, `-`, `*`, `/` and their `+=`-style forms work with `f64` and (for `+`/`-`) other currencies. Use references to avoid moving values. The result always keeps the left-hand side's options.

```rust
let price = Currency::from(123.5);

(&price + 0.23).value();                 // 123.73
(&price - 0.5).value();                  // 123.00
(&price * 3.).value();                   // 370.50
(&price / 2.).value();                   // 61.75
(&price + &price).value();               // 247.00

let mut total = Currency::from(1.33);
total *= 3.;
if total > 3. {                          // compare with f64 or Currency
    total -= 1.;
}
total.format();                          // "$2.99"
```

Sum an iterator, or split an amount without losing pennies:

```rust
let items = [Currency::from(0.1), Currency::from(0.2)];
let sum: Currency = items.iter().sum();   // 0.30

Currency::from(1.12).distribute(5);       // [0.23, 0.23, 0.22, 0.22, 0.22]
```

`dollars()` and `cents()` return the whole and fractional parts, `int_value()` the amount in minor units:

```rust
let c = Currency::from(-1.23);
c.dollars();     // -1
c.cents();       // -23
c.int_value();   // -123
```

## Formatting

`format()` applies the symbol, separators and patterns; `to_string()` gives the plain rounded number.

```rust
("2,573,693.75".parse::<Currency>()? + "100,275.50".parse::<Currency>()?).format(); // "$2,673,969.25"
("1,237.72".parse::<Currency>()? - 300.).format();                                  // "$937.72"
```

Override options for a single call with `format_with`:

```rust
let c = Currency::from(1.23);
c.format_with(|o| o.symbol("£"));    // "£1.23"
c.format_with(|o| o.pattern("#"));   // "1.23"
```

Need fully custom output? Write a function: `c.value()`, `c.int_value()` and `c.opts()` give you everything currency.js passes to a format function.

## Options

_currency_rs_ comes with its own set of default options conforming to USD. Each option is a chainable setter on `CurrencyOpts` and a public field you can read (e.g. `opts.symbol`).

`symbol` _default_: `$`<br/>
Currency symbol included when calling `format()`.

`separator` _default_: `,`<br/>
Separator dividing the number groups when calling `format()`.

`decimal` _default_: `.`<br/>
Decimal used when calling `format()`.

`precision` _default_: `2`<br/>
Number of decimal places to store as cents.

`pattern` _default_: `!#`<br/>
Allows you to customize the format pattern using `!` as replacement for the currency symbol and `#` as replacement for the currency amount.

`negative_pattern` _default_: `-!#`<br/>
Allows you to customize the negative format pattern using `!` as replacement for the currency symbol and `#` as replacement for the currency amount.

`error_on_invalid` _default_: `false`<br/>
If an unparsable value such as `abc` is passed to `parse`, return `Err` instead of `0`.

`increment` _default_: one minor unit<br/>
When implementing a currency that implements rounding, setting the increment value will allow you to set the closest increment to round the display value to.

```rust
CurrencyOpts::new().increment(0.05).from(1.48).format(); // "$1.50"
```

`use_vedic` _default_: `false`<br/>
Formats number groupings using the Indian Numbering System, i.e. `10,00,000.00`

`from_cents` _default_: `false`<br/>
Parse the amount value as a minor currency unit (e.g. cents in a dollar) instead of dollars.

## Working with multiple currencies

Keep one `CurrencyOpts` per currency and create values from it:

```rust
let usd = CurrencyOpts::new();
let jpy = CurrencyOpts::new().symbol("¥").precision(0);
let gas = CurrencyOpts::new().precision(3);

usd.from(1234.56).format(); // "$1,234.56"
jpy.from(1234.56).format(); // "¥1,235"
gas.from(1234.56).format(); // "$1,234.560"
```

## Migrating from 1.x

| 1.x                                                        | 2.0                                        |
| ---------------------------------------------------------- | ------------------------------------------ |
| `Currency::new_float(v, None)`                             | `Currency::from(v)`                        |
| `Currency::new_float(v, Some(opts))`                       | `opts.from(v)`                             |
| `Currency::new_string(s, None)`                            | `s.parse::<Currency>()`                    |
| `Currency::new_string(s, Some(opts))`                      | `opts.parse(s)`                            |
| `.set_symbol("€")` (all `set_*`)                           | `.symbol("€")`                             |
| `opts.symbol()` (getters)                                  | `opts.symbol` (fields)                     |
| `.add(x)` / `.subtract(x)` / `.multiply(x)` / `.divide(x)` | `+` `-` `*` `/`                            |
| `.add_string(s)?`                                          | `c + opts.parse(s)?`                       |
| `c.clone() + other.value()`                                | `&c + &other`                              |
| `c.value() > 30.`                                          | `c > 30.`                                  |
| `features = ["regex"]`                                     | removed: same output, ~100× slower parsing |
| `int_value() -> f64`                                       | `int_value() -> i64`                       |
| `cents() -> u64`                                           | `cents() -> i64` (keeps sign)              |
| `distribute(i64)`                                          | `distribute(usize)`                        |

Behaviour changes, to match currency.js:

- Negative halves round toward +∞ like JS `Math.round`: `-17.955 → -17.95` (was `-17.96`).
- `c / -2.` divides (was a no-op for any divisor ≤ 0).
- With `from_cents`, `a + b` between currencies adds their minor units (was off by a factor of 100).
- `to_string()`/`format()` round exact ties like JS `toFixed` (`0.25` at precision 1 → `0.3`, was `0.2`) and never print `-0.00`.
- Multi-character separators (e.g. `", "`, `&nbsp;`) are no longer reversed by `format()`.

## Benchmarks

Run with `cargo bench --bench currency`.

Each benchmark iterates over 500 values (200 for e2e). Median times on an Apple M1 Pro:

| Benchmark                       |     Time |
| ------------------------------- | -------: |
| `parse` (`"$1.23".parse()`)     | 302.2 µs |
| `from` (`Currency::from(1.23)`) | 291.6 µs |
| `format`                        | 184.8 µs |
| `format_vedic`                  | 181.7 µs |
| `operator +`                    | 310.0 µs |
| `operator -`                    | 308.0 µs |
| `operator *`                    |  1.95 ms |
| `operator /`                    | 262.5 µs |
| `distribute`                    |   3.9 µs |
| `e2e` (parse→add→format)        | 205.6 µs |

## License

[MIT](/license)
