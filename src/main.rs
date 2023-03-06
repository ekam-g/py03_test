use pyo3::prelude::*;

fn main() {
    let py_app = include_str!("long.py");
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let app: Py<PyAny> = PyModule::from_code(py, py_app, "long.py", "").unwrap()
            .getattr("test")?
            .into();
        app.call0(py)
    });
    println!("py: {}", from_python.unwrap());
}
