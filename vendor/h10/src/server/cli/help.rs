use super::traits::ArgName;

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct CliHelp;

impl ArgName for CliHelp {
    fn arg_name() -> String {
        "--help".into()
    }
}
