use pyo3::prelude::*;

fn main() {
    for _ in 0..200 {
        println!("python says: {}", python());
    }
}

fn python() -> String {
    let py_app = include_str!("long.py");
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let app: Py<PyAny> = PyModule::from_code(py, py_app, "", "")
            .unwrap()
            .getattr("test").unwrap()
            .into();
        app.call0(py)
    });
    from_python.unwrap().to_string()
}
