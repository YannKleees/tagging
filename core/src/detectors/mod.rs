use crate::{Result, Span, Token};

pub trait Detector: Send + Sync {
    fn detect(&self, text: &str, tokens: &[Token]) -> Vec<Span>;
}

macro_rules! stub {
    ($($name:ident),*) => {$(
        pub struct $name;
        impl $name {
            pub fn new() -> Result<Self> { Ok(Self) }
        }
        impl Detector for $name {
            fn detect(&self, _: &str, _: &[Token]) -> Vec<Span> { Vec::new() }
        }
    )*};
}

stub!(EmailDetector, UrlDetector, IpDetector, PhoneDetector, IbanDetector, CardDetector, NationalIdDetector, DateDetector);
