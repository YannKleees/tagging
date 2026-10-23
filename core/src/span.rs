use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Email,
    Url,
    IpAddress,
    Phone,
    Iban,
    CreditCard,
    NationalId,
    DateOfBirth,
    Date,
    Person,
    Organization,
    Location,
    Address,
}

impl EntityType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::Url => "url",
            Self::IpAddress => "ip_address",
            Self::Phone => "phone",
            Self::Iban => "iban",
            Self::CreditCard => "credit_card",
            Self::NationalId => "national_id",
            Self::DateOfBirth => "date_of_birth",
            Self::Date => "date",
            Self::Person => "person",
            Self::Organization => "organization",
            Self::Location => "location",
            Self::Address => "address",
        }
    }

    pub fn label(self) -> String {
        self.as_str().to_uppercase()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    Rule,
    Gazetteer,
    Ner,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Span {
    #[serde(rename = "type")]
    pub kind: EntityType,
    pub start: usize,
    pub end: usize,
    pub text: String,
    pub score: f64,
    pub source: Source,
}

impl Span {
    pub fn new(text: &str, kind: EntityType, start: usize, end: usize, score: f64, source: Source) -> Self {
        Self {
            kind,
            start,
            end,
            text: text[start..end].to_string(),
            score,
            source,
        }
    }

    pub fn rule(text: &str, kind: EntityType, start: usize, end: usize, score: f64) -> Self {
        Self::new(text, kind, start, end, score, Source::Rule)
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn overlaps(&self, other: &Span) -> bool {
        self.start < other.end && other.start < self.end
    }
}
