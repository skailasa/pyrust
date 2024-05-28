use rlst::{rlst_dynamic_array2, MatrixSvd, MultIntoResize, RawAccess};

extern crate blas_src;
extern crate lapack_src;

#[no_mangle]
pub extern "C" fn hello() {

    let a = rlst_dynamic_array2!(f32, [2, 2]);
    let b = rlst_dynamic_array2!(f32, [2, 2]);

    let mut c = rlst_dynamic_array2!(f32, [2, 2]);

    c.view_mut().simple_mult_into_resize(a.view(), b.view());

    println!("HERE {:?}", c.data());

    println!("Hello from Rust")
}

