// This file is part of Recsync-rs.
// Copyright (c) 2024 UK Research and Innovation, Science and Technology Facilities Council
//
// This project is licensed under both the MIT License and the BSD 3-Clause License.
// You must comply with both licenses to use, modify, or distribute this software.
// See the LICENSE file for details.


#![allow(missing_docs)]
use std::{collections::HashMap, sync::Arc};

use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::future_into_py_with_locals;
use reccaster::{Record, Reccaster};
use tokio::sync::Mutex;
       
#[pyclass]
pub struct PyRecord(Record);

#[pymethods]
impl PyRecord {
    #[new]
    #[pyo3(signature = (name, r#type, alias=None, properties=HashMap::new()))]
    fn new(name: String, r#type: String, alias: Option<String>, properties: HashMap<String, String>) -> Self {
        PyRecord(Record { name, r#type, alias, properties })
    }

    #[getter]
    fn name(&self) -> &str {
        &self.0.name
    }

    #[getter]
    fn r#type(&self) -> &str {
        &self.0.r#type
    }

    #[getter]
    fn alias(&self) -> Option<&String> {
        self.0.alias.as_ref()
    }

    #[getter]
    fn properties(&self) -> PyResult<HashMap<String, String>> {
        Ok(self.0.properties.clone())
    }

}

impl FromPyObject<'_> for PyRecord {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let name: String = ob.getattr("name")?.extract().unwrap_or_else(|_| "OPS no name !!!!!!!!!!!".to_string());
        let r#type: String = ob.getattr("type")?.extract()?;
        let alias: Option<String> = ob.getattr("alias")?.extract()?;
        let properties: HashMap<String, String> = ob.getattr("properties")?.extract()?;
        Ok(PyRecord (Record { name, r#type, alias, properties }))
    }
}

#[pyclass]
struct PyReccaster {
    reccaster: Arc<Mutex<Reccaster>>,
}

#[pymethods]
impl PyReccaster {

    #[staticmethod]
    fn setup(py: Python<'_>, records: Vec<PyRecord>, props: Option<HashMap<String, String>>) -> PyResult<Bound<'_, pyo3::PyAny>> {
        let locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
        let pvs = records.iter().map(|record: &PyRecord| record.0.clone()).collect::<Vec<Record>>();
        future_into_py_with_locals(py, locals, async move {
            let recc = Reccaster::new(pvs, props).await;
            let pyrecc = PyReccaster { reccaster: Arc::new(Mutex::new(recc)) };
            Python::with_gil(|_py| Ok(pyrecc))
        })
    }

    fn run<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let recc_arc = self.reccaster.clone();
        let locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
        future_into_py_with_locals(py, locals, async move {
            let mut recc = recc_arc.lock().await;
            recc.run().await;
            Ok(())
        })
    }
}

#[pymodule]
fn pyreccaster(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyReccaster>()?;
    m.add_class::<PyRecord>()?;
    Ok(())
}
