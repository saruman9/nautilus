// Nautilus
// Copyright (C) 2024  Daniel Teuchert, Cornelius Aschermann, Sergej Schumilo

use std::ffi::CString;

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

use crate::Context;

#[pyclass]
struct PyContext {
    ctx: Context,
}
impl PyContext {
    fn get_context(&self) -> Context {
        self.ctx.clone()
    }
}

#[pymethods]
impl PyContext {
    #[new]
    fn new() -> Self {
        PyContext {
            ctx: Context::new(),
        }
    }

    fn rule(&mut self, py: Python, nt: &str, format: Py<PyAny>) /* -> PyResult<()> */
    {
        if let Ok(s) = format.extract::<&str>(py) {
            self.ctx.add_rule(nt, s.as_bytes());
        } else if let Ok(s) = format.extract::<&[u8]>(py) {
            self.ctx.add_rule(nt, s);
        } else {
            panic!("format argument should be string or bytes");
            // return Err(pyo3::exceptions::PyValueError::new_err(
            //     "format argument should be string or bytes",
            // ));
        }
        // Ok(())
    }

    fn script(&mut self, nt: &str, nts: Vec<String>, script: Py<PyAny>) {
        self.ctx.add_script(nt, nts, script);
    }

    fn regex(&mut self, nt: &str, regex: &str) {
        self.ctx.add_regex(nt, regex);
    }
}

fn main_(py: Python, grammar_path: &str) -> PyResult<Context> {
    let py_ctx = Bound::new(py, PyContext::new()).unwrap();
    let locals = [("ctx", &py_ctx)].into_py_dict(py)?;
    py.run(
        CString::new(std::fs::read_to_string(grammar_path).expect("couldn't read grammar file"))?
            .as_c_str(),
        None,
        Some(&locals),
    )?;
    Ok(py_ctx.borrow().get_context())
}

pub fn load_python_grammar(grammar_path: &str) -> Context {
    return Python::attach(|py| {
        main_(py, grammar_path)
            .map_err(|e| e.print_and_set_sys_last_vars(py))
            .unwrap()
    });
}
