#[derive(Debug, Clone)]
pub struct CurrencyOpts {
    pub symbol: String,
    pub separator: String,
    pub decimal: String,
    pub precision: f64,
    pub pattern: String,
    pub negative_pattern: String,
    pub from_cents: bool,
    pub increment: Option<f64>,
    pub use_vedic: bool,
    pub error_on_invalid: bool,
}

impl Default for CurrencyOpts {
    fn default() -> Self {
        Self {
            symbol: "$".into(),
            separator: ",".into(),
            decimal: ".".into(),
            precision: 2.,
            pattern: "!#".into(),
            negative_pattern: "-!#".into(),
            from_cents: false,
            increment: None,
            use_vedic: false,
            error_on_invalid: false,
        }
    }
}
