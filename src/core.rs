//! core
//!

use std::{result::Result, error::Error};

use pyo3::prelude::*;
use pyo3::types::{PyDict, IntoPyDict};

/// py_test
pub fn py_test() -> Result<(), Box<dyn Error>> {
  Python::attach(|py| { // feature = ["auto-initialize"] auto init when attach
    let sys = py.import("sys")?;
    let ver: String = sys.getattr("version")?.extract()?;
    let locals = [("os", py.import("os")?)].into_py_dict(py)?;
    let code = c"os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
    let user: String = py.eval(code, None, Some(&locals))?.extract()?;
    println!("Hello python: {}, {}", user, ver);
    Ok(())
  })
}

/// tempita_sub_test
/// s = tempita.sub('n = {{a * b}}', a='x', b=12) # n = xxxxxxxxxxxx
pub fn tempita_sub_test() -> Result<(), Box<dyn Error>> {
  Python::attach(|py| { // not found with_gil
    let tempita = PyModule::import(py, "tempita")?;
    // let tempita = py.import("tempita")?;
    let t = "n = {{a * b}}";
    let kw = PyDict::new(py);
    kw.set_item("a", "x")?;
    kw.set_item("b", 12)?;
    let s: String = tempita.getattr("sub")?.call((t, ), Some(&kw))?.extract()?;
    println!("Tempita sub: {}", s);
    Ok(())
  })
}

/// tempita_substitute_test
/// tpl = tempita.Template('{{upper(n)}} = {{a * b}}',
///   namespace=dict(upper=lambda s: s.upper()))
/// s = tpl.substitute(a='X', b=12, n='o') # O = XXXXXXXXXXXX
pub fn tempita_substitute_test() -> Result<(), Box<dyn Error>> {
  Python::attach(|py| { // not found with_gil
    // let tempita = PyModule::import(py, "tempita")?;
    let tempita = py.import("tempita")?;
    let t = "{{upper(n)}} = {{a * b}}";
    let kw = PyDict::new(py);
    // kw.set_item("namespace", PyDict::new(py))?;
    kw.set_item("namespace", [
      ("upper", py.eval(c"lambda s: s.upper()", None, None)?)
    ].into_py_dict(py)?)?;
    let tpl = tempita.getattr("Template")?.call((t, ), Some(&kw))?;
    let kw = PyDict::new(py);
    kw.set_item("a", "X")?;
    kw.set_item("b", 12)?;
    kw.set_item("n", "o")?;
    let s: String = tpl.getattr("substitute")?.call((), Some(&kw))?.extract()?;
    println!("Tempita substitute: {}", s);
    Ok(())
  })
}

/// lib_test
pub fn lib_test() -> Result<(), Box<dyn Error>> {
  assert_eq!(py_test()?, ());
  assert_eq!(tempita_sub_test()?, ());
  assert_eq!(tempita_substitute_test()?, ());
  Ok(())
}
