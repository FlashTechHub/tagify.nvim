use serde::Deserialize;

use super::BorderStyle;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Border {
    /// Whether to enable the border.
    pub enable: bool,

    /// The style of the border. Can be any of the values listed in `:h
    /// nvim_open_win`.
    pub style: BorderStyle,
}

/// Helper struct used to deserialize the borders in the completion menu and in
/// the details window with different defaults.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IncompleteBorder {
    #[serde(rename = "enable")]
    pub maybe_enable: Option<bool>,

    #[serde(rename = "style")]
    pub maybe_style: Option<BorderStyle>,
}
