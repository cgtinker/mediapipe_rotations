use ndarray;
use numpy::{IntoPyArray, PyArray1, PyArray2, PyArrayDyn, PyReadonlyArrayDyn};
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

// NOTE
// * numpy defaults to np.float64, if you use other type than f64 in Rust
//   you will have to change type in Python before calling the Rust function.

// The name of the module must be the same as the rust package name
#[pymodule]
fn mediapipe_rotations(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // This is a pure function (no mutations of incoming data).
    // You can see this as the python array in the function arguments is readonly.
    // The object we return will need ot have the same lifetime as the Python.
    // Python will handle the objects deallocation.
    // We are having the Python as input with a lifetime parameter.
    // Basically, none of the data that comes from Python can survive
    // longer than Python itself. Therefore, if Python is dropped, so must our Rust Python-dependent variables.
    #[pyfn(m)]
    fn max_min<'py>(py: Python<'py>, x: PyReadonlyArrayDyn<f64>) -> &'py PyArray1<f64> {
        // Here we have a numpy array of dynamic size. But we could restrict the
        // function to only take arrays of certain size
        // e.g. We could say PyReadonlyArray3 and only take 3 dim arrays.
        // These functions will also do type checking so a
        // numpy array of type np.float32 will not be accepted and will
        // yield an Exception in Python as expected
        //
        // like that we can convert (finally) - directly to vecs if u ask me
        let array = x.as_array();
        let sl = array.to_slice().unwrap();
        for chunk in sl.chunks(3) {
            println!("arr {:?}", chunk);
        }

        let result_array = rust_fn::max_min(&array);
        result_array.into_pyarray(py)
    }

    #[pyfn(m)]
    fn eye<'py>(py: Python<'py>, size: usize) -> &PyArray2<f32> {
        // Simple demonstration of creating an ndarray inside Rust and return
        let arr: [[f32; 4]; 3] = [[1.3, 1.1, 2.0, 1.0], [1.0, 2.1, 2.1, 2.8], [-2.1, 1.4, 1.1, 1.0]];
        let array = ndarray::arr2(&arr);
        // let array = ndarray::Array::eye(size);
        array.into_pyarray(py)
    }

    Ok(())
}

// The rust side functions
// Put it in mod to separate it from the python bindings
// These are just some random operations
// you probably want to do something more meaningful.
mod rust_fn {
    use ndarray::{arr1, Array1};
    use numpy::ndarray::{ArrayViewD, ArrayViewMutD};

    pub fn max_min(x: &ArrayViewD<'_, f64>) -> Array1<f64> {
        println!("{}", x.len());
        if x.len() == 0 {
            return arr1(&[]); // If the array has no elements, return empty array
        }
        let result_array = arr1(&[12.0, 2.0]);
        result_array
    }
}

