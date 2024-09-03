use super::{HeaderEntry, HeaderName, HeaderValue};

#[derive(Debug)]
pub struct HeaderEntryBuilder;

impl HeaderEntryBuilder {
    pub fn name(self, header_name: HeaderName) -> Step1 {
        Step1 { header_name }
    }
}

#[derive(Debug)]
pub struct Step1 {
    header_name: HeaderName,
}

impl Step1 {
    pub fn value(self, header_value: HeaderValue) -> Step2 {
        let Self { header_name } = self;
        Step2 {
            header_name,
            header_value,
        }
    }
}

#[derive(Debug)]
pub struct Step2 {
    header_name: HeaderName,
    header_value: HeaderValue,
}

impl Step2 {
    pub fn finish(self) -> HeaderEntry {
        let Self {
            header_name,
            header_value,
        } = self;
        HeaderEntry {
            name: header_name,
            value: header_value,
        }
    }
}
