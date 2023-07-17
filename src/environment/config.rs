use std::path::Path;

use noir::EnvironmentConfig;
use pyo3::{pyclass, pymethods};

#[pyclass]
#[derive(Clone)]
pub struct PyEnvironmentConfig(pub EnvironmentConfig);

#[pymethods]
impl PyEnvironmentConfig {
    #[staticmethod]
    pub fn local(num_cores: u64) -> Self {
        Self(EnvironmentConfig::local(num_cores))
    }

    #[staticmethod]
    pub fn remote(path: &str) -> Self {
        Self(EnvironmentConfig::remote(Path::new(path)).unwrap())
    }
}
