use super::traits::ArgName;

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) enum CliVerboseMode {
    Enabled,
    #[default]
    Disabled,
}

impl ArgName for CliVerboseMode {
    fn arg_name() -> String {
        "--verbose".into()
    }
}
