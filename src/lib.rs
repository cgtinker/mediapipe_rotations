use pyo3::prelude::{pymodule, PyModule, PyResult, Python, pyfunction};
use pyo3::wrap_pyfunction;

pub mod pose;
pub mod hand;
pub mod face;


#[pyfunction]
fn pose(data: Vec<[f32;3]>) -> PyResult<Vec<[f32; 4]>> {
    // Exposed python function for mediapipe detection results.
    // Input:   [[f32; 3]; 33]
    // Output:  [[f32; 4]; 36]
    if data.len() == 33 {
        let rotations = pose::main(&data);
        let mut result: Vec<[f32; 4]> = Vec::new();
        for x in rotations.iter() {
            result.push(x.to_array());
        }
        return Ok(result);
    }
    else {
        return Ok(vec![[f32::NAN, f32::NAN, f32::NAN, f32::NAN]; 36]);
    }
}

#[pyfunction]
fn face(data: Vec<[f32;3]>) -> PyResult<Vec<[f32; 4]>> {
    // Exposed python function for mediapipe detection results.
    // Input:   [[f32; 3]; 468]
    // Output:  [[f32; 4]; 4]
    if data.len() == 468 {
        let rotations = face::main(&data);
        let mut result: Vec<[f32; 4]> = Vec::new();
        for x in rotations.iter() {
            result.push(x.to_array());
        }
        return Ok(result);
    }
    else {
        return Ok(vec![[f32::NAN, f32::NAN, f32::NAN, f32::NAN]; 36]);
    }
}

fn _hand(data: Vec<[f32; 3]>) -> Vec<[f32; 4]> {
    if data.len() == 21 {
        let rotations = hand::main(&data);
        let mut result: Vec<[f32; 4]> = Vec::new();
        for x in rotations.iter() {
            result.push(x.to_array());
        }
        return result;
    }
    else {
        return vec![[f32::NAN, f32::NAN, f32::NAN, f32::NAN]; 21];
    }
}


#[pyfunction]
fn hand(data: Vec<[f32;3]>) -> PyResult<Vec<[f32; 4]>> {
    // Exposed python function for mediapipe detection results.
    // Input:   [[f32; 3]; 21]
    // Output:  [[f32; 4]; 22]
    return Ok(_hand(data));
}

#[pyfunction]
fn hands(data: Vec<Vec<[f32; 3]>>) -> PyResult<Vec<Vec<[f32; 4]>>> {
    // Exposed python function for mediapipe detection results.
    // Input:   [[[f32; 3]; 21]; 2]
    // Output:  [[[f32; 4]; 22]; 2]
    let mut result: Vec<Vec<[f32; 4]>> = Vec::new();
    for cur in data.iter() {
        let _res = _hand(cur.to_vec());
        result.push(_res);
    }
    return Ok(result);
}


#[pymodule]
fn mediapipe_rotations(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pose, m)?)?;
    m.add_function(wrap_pyfunction!(face, m)?)?;
    m.add_function(wrap_pyfunction!(hand, m)?)?;
    m.add_function(wrap_pyfunction!(hands, m)?)?;
    Ok(())
}

