use super::traits::ArgFields;

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) enum CliVerboseMode {
    Enabled,
    #[default]
    Disabled,
}

impl ArgFields for CliVerboseMode {
    fn long() -> &'static str {
        "--verbose"
    }

    fn description() -> &'static str {
        "Show raw contents from both Request and Response"
    }
}
