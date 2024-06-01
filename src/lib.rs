use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyModule;
use pyo3::{pymodule, PyResult};

#[cfg(target_os = "linux")]
use jemallocator::Jemalloc;

#[global_allocator]
#[cfg(target_os = "linux")]
static ALLOC: Jemalloc = Jemalloc;

#[pyfunction]
fn create_api_client() -> PyResult<String> {
    Ok("API Client Created".to_string())
}

#[pyfunction]
fn version() -> &'static str {
    "0.1.0"
}

#[pymodule]
fn demopyrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(create_api_client, m)?)?;
    Ok(())
}
