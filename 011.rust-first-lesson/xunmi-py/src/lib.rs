use pyo3::{exceptions, prelude::*};
use xunmi::{self as x};

#[pyclass]
pub struct Indexer(x::Indexer);

#[pyclass]
pub struct InputConfig(x::InputConfig);

#[pyclass]
pub struct IndexUpdater(x::IndexUpdater);

#[pymodule]
fn xunmi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Indexer>()?;
    m.add_class::<InputConfig>()?;
    m.add_class::<IndexUpdater>()?;
    Ok(())
}

#[pyfunction]
pub fn example_sql() -> PyResult<String> {
    Ok(queryer::example_sql())
}

#[pyfunction]
pub fn query(sql: &str, output: Option<&str>) -> PyResult<String> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let data = rt.block_on(async { queryer::query(sql).await.unwrap() });
    match output {
        Some("csv") | None => Ok(data.to_csv().unwrap()),
        Some(v) => Err(exceptions::PyTypeError::new_err(format!(
            "Output type {} not supported",
            v
        ))),
    }
}

#[pymodule]
fn queryer_py(_py: Python, m: &PyModule) -> PyResult<()> {
    // 通过 m.add_function 可以注册函数
    m.add_function(wrap_pyfunction!(query, m)?)?;
    m.add_function(wrap_pyfunction!(example_sql, m)?)?;
    Ok(())
}
