pub mod python_interface;
pub mod python_dataframe_model;
mod time_and_dates;
mod dialogs;

use pyo3_polars::*;
use pyo3::prelude::*;
use polars::prelude::*;
use pyo3::types::{IntoPyDict, PyDict};
use pyo3::ffi::c_str;
use std::ffi::CStr;
use std::pin::Pin;
use cxx_qt::casting::Upcast;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QQmlEngine, QString, QUrl};
use chameleon_settings::{get_global_settings_python_base_directory, get_global_settings_qml_directory};
use chameleon_format_dialog::format::OutputFor;
use chameleon_format_dialog::locale::get_locale_information;

const CODE: &CStr = c_str!(r#"
import polars as pl
def function(*args, **kwargs):
    assert args == ("hello",)
    assert kwargs == {"cruel": "world"}
    return "called with args and kwargs"
"#);

const CODE2: &CStr = c_str!(r#"
import numpy as np
import polars as pl

num_rows = 5000
rng = np.random.default_rng(seed=7)

buildings_data = {
      "sqft": rng.exponential(scale=1000, size=num_rows),
      "year": rng.integers(low=1995, high=2023, size=num_rows),
      "building_type": rng.choice(["A", "B", "C"], size=num_rows),
 }
buildings = pl.DataFrame(buildings_data)
buildings
"#);

fn pyo3_test_3() {
    let mut df: DataFrame = DataFrame::empty();


    let result : Result<(), PyErr> = Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let path = sys.getattr("path")?;
        let mut python_base_directory = get_global_settings_python_base_directory();
        python_base_directory.push("Lib");
        python_base_directory.push("site-packages");

        path.call_method1("append", (python_base_directory.to_str().unwrap(),))?;

        let module = PyModule::from_code(py, CODE2, c_str!(""), c_str!(""))?;
        let dir = module.dir()?;
        for i in dir.iter() {
            let s = i.extract::<String>()?;
            let t = module.getattr(&s)?.get_type();
            let m = t.getattr("__module__")?.extract::<String>()?;
            if m.contains("polars") && t.name()? == "DataFrame" {

                let res = PyDataFrame::extract_bound(&module.getattr(i.extract::<String>()?)?)?;
                let res2 : DataFrame = res.into();
                df = res2.clone();
                println!("{:?}", res2);
                // println!("{} : {}.{}", i.extract::<String>()?, m, t.name()?);
            }
        }
        Ok(())
    });
    let schema = df.schema();
    for field in schema.iter_fields() {
        println!("{} : {:?}", field.name, field.dtype());
    }
    println!("{:?}", df);
    if result.is_ok() {
        println!("pyo3_test_3: OK");
    } else {
        let e = result.unwrap_err();
        println!("pyo3_test_3: Error: {:?}", e);
    }
}


fn pyo3_test_2() {
    let result : Result<(), PyErr> = Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let path = sys.getattr("path")?;
        let mut python_base_directory = get_global_settings_python_base_directory();
        python_base_directory.push("Lib");
        python_base_directory.push("site-packages");

        path.call_method1("append", (python_base_directory.to_str().unwrap(),))?;

        let module = PyModule::from_code(py, CODE, c_str!(""), c_str!(""))?;
        let fun = module.getattr("function")?;
        let args = ("hello",);
        let kwargs = PyDict::new(py);
        kwargs.set_item("cruel", "world")?;
        let result = fun.call(args, Some(&kwargs))?;
        assert_eq!(result.extract::<String>()?, "called with args and kwargs");
        Ok(())
    });
    if result.is_ok() {
        println!("OK");
    } else {
        let e = result.unwrap_err();
        println!("Error: {:?}", e);
    }
}

fn polars_main() {


    use chrono::prelude::*;

    let df: DataFrame = df!(
    "name" => ["Alice Archer", "Ben Brown", "Chloe Cooper", "Daniel Donovan"],
    "birthdate" => [
        NaiveDate::from_ymd_opt(1997, 1, 10).unwrap(),
        NaiveDate::from_ymd_opt(1985, 2, 15).unwrap(),
        NaiveDate::from_ymd_opt(1983, 3, 22).unwrap(),
        NaiveDate::from_ymd_opt(1981, 4, 30).unwrap(),
    ],
    "weight" => [57.9, 72.5, 53.6, 83.1],  // (kg)
    "height" => [1.56, 1.77, 1.65, 1.75],  // (m)
)
        .unwrap();
    println!("{df}");

    // Assume `df` is an existing DataFrame:
    //    let column = df.column("column_name").unwrap();
    //    let column_by_index = df.select_at_idx(0).unwrap();

    let value = &df.get(0).unwrap()[1];
    println!("{value}");
}

fn pymain() -> PyResult<()> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py)?;
        let code = c_str!("os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'");
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}



pub fn run_main() {

    pyo3_test_2();
    pyo3_test_3();

    polars_main();
    let _ = pymain();
    // Create the application and engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Load the QML path into the engine
    if let Some(engine) = engine.as_mut() {
        let qml_directory = get_global_settings_qml_directory();
        //engine.add_import_path(&QString::from(qml_directory.to_str().unwrap()));
    }

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/chameleon/main/qml/main.qml"));
    }

    if let Some(engine) = engine.as_mut() {
        let engine: Pin<&mut QQmlEngine> = engine.upcast_pin();
        // Listen to a signal from the QML Engine
        engine
            .on_quit(|_| {
                println!("QML Quit!");
            })
            .release();
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}