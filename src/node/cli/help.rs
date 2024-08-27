use super::traits::ArgName;

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) enum CliHelp {
    Enabled,
    #[default]
    Disabled,
}

impl ArgName for CliHelp {
    fn arg_name() -> String {
        "--help".into()
    }
}
