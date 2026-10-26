use std::path::Path;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde::Deserialize;

use crate::{Config, EntityType, Error, Mode, Source, Span, Tagger};

fn value_error(e: Error) -> PyErr {
    PyValueError::new_err(e.to_string())
}

#[pyclass(frozen, name = "Tagger")]
struct PyTagger(Tagger);

#[pymethods]
impl PyTagger {
    #[new]
    #[pyo3(signature = (config=None))]
    fn new(config: Option<&str>) -> PyResult<Self> {
        let config = config.map(Config::from_json).transpose().map_err(value_error)?;
        Ok(Self(Tagger::new(&config.unwrap_or_default()).map_err(value_error)?))
    }

    #[staticmethod]
    fn from_file(path: &str) -> PyResult<Self> {
        let config = Config::from_path(Path::new(path)).map_err(value_error)?;
        Ok(Self(Tagger::new(&config).map_err(value_error)?))
    }

    /// Returns the spans as a JSON string; the Python wrapper decodes it.
    fn detect(&self, py: Python<'_>, text: &str) -> PyResult<String> {
        let spans = py.detach(|| self.0.detect(text));
        serde_json::to_string(&spans).map_err(|e| value_error(e.into()))
    }
}

/// Only type and offsets matter for redaction, so callers may pass bare `{type, start, end}`.
#[derive(Deserialize)]
struct Target {
    #[serde(rename = "type")]
    kind: EntityType,
    start: usize,
    end: usize,
}

#[pyfunction]
fn redact(text: &str, spans: &str, mode: &str) -> PyResult<String> {
    let targets: Vec<Target> = serde_json::from_str(spans).map_err(|e| value_error(e.into()))?;
    let mode: Mode = serde_json::from_value(mode.into()).map_err(|e| value_error(e.into()))?;
    let spans: Vec<Span> = targets
        .into_iter()
        .map(|t| Span {
            kind: t.kind,
            start: t.start,
            end: t.end,
            text: String::new(),
            score: 1.0,
            source: Source::Rule,
        })
        .collect();
    crate::redact(text, &spans, mode).map_err(value_error)
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTagger>()?;
    m.add_function(wrap_pyfunction!(redact, m)?)?;
    Ok(())
}
