use pyo3::prelude::*;

#[pyfunction]
fn gc_content(seq: &str) -> PyResult<f32> {
    let mut length = 0;
    let mut count = 0;
    for base in seq.chars() {
        length += 1;
        if base == 'G' || base == 'C' {
            count += 1;
        }
    }
    Ok(count as f32 / length as f32)
}

#[pymodule]
fn rusty_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gc_content, m)?)?;
    Ok(())
}