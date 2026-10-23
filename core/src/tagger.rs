use crate::config::read_to_string;
use crate::detectors::{
    CardDetector, DateDetector, Detector, EmailDetector, IbanDetector, IpDetector, NationalIdDetector, PhoneDetector,
    UrlDetector,
};
use crate::gazetteer::Gazetteer;
use crate::{Config, DetectorKind, Result, Span, resolve, tokenize};

pub struct Tagger {
    detectors: Vec<Box<dyn Detector>>,
    threshold: f64,
}

impl Tagger {
    pub fn new(config: &Config) -> Result<Self> {
        let mut detectors: Vec<Box<dyn Detector>> = Vec::new();
        for (i, kind) in config.detectors.iter().enumerate() {
            if config.detectors[..i].contains(kind) {
                continue;
            }
            detectors.push(match kind {
                DetectorKind::Email => Box::new(EmailDetector::new()?),
                DetectorKind::Url => Box::new(UrlDetector::new()?),
                DetectorKind::Ip => Box::new(IpDetector::new()?),
                DetectorKind::Phone => Box::new(PhoneDetector::new()?),
                DetectorKind::Iban => Box::new(IbanDetector::new()?),
                DetectorKind::CreditCard => Box::new(CardDetector::new()?),
                DetectorKind::NationalId => Box::new(NationalIdDetector::new()?),
                DetectorKind::Date => Box::new(DateDetector::new()?),
                DetectorKind::Gazetteer => Box::new(gazetteer(config)?),
            });
        }
        Ok(Self {
            detectors,
            threshold: config.threshold,
        })
    }

    pub fn detect(&self, text: &str) -> Vec<Span> {
        let tokens = tokenize(text);
        let spans = self
            .detectors
            .iter()
            .flat_map(|d| d.detect(text, &tokens))
            .filter(|s| s.score >= self.threshold)
            .collect();
        resolve(spans)
    }
}

fn gazetteer(config: &Config) -> Result<Gazetteer> {
    let mut g = if config.builtin_gazetteers {
        Gazetteer::builtin()
    } else {
        Gazetteer::new()
    };
    for file in &config.gazetteers {
        g.add_list(&read_to_string(&file.path)?, file.kind);
    }
    for list in &config.wordlists {
        g.add_words(&list.words, list.kind);
    }
    Ok(g)
}
