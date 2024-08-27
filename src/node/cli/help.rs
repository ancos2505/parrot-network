use super::traits::ArgFields;

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) enum CliHelp {
    Enabled,
    #[default]
    Disabled,
}

impl ArgFields for CliHelp {
    fn long() -> &'static str {
        "--help"
    }

    fn description() -> &'static str {
        "Display this message"
    }
}
