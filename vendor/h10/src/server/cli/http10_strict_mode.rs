use super::traits::ArgName;

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) enum CliHttp10StrictMode {
    Enabled,
    #[default]
    Disabled,
}

impl ArgName for CliHttp10StrictMode {
    fn arg_name() -> String {
        "--http1.0".into()
    }
}
