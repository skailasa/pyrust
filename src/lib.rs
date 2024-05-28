use rlst::{rlst_array_from_slice1, rlst_dynamic_array2, MultIntoResize, RawAccess};

extern crate blas_src;
extern crate lapack_src;

#[no_mangle]
pub extern "C" fn calling_blas() {

    // Practice linking BLAS libraries
    let a = rlst_dynamic_array2!(f32, [2, 2]);
    let b = rlst_dynamic_array2!(f32, [2, 2]);
    let mut c = rlst_dynamic_array2!(f32, [2, 2]);
    c.view_mut().simple_mult_into_resize(a.view(), b.view());
    println!("Calling BLAS function with RLST")
}


#[no_mangle]
pub extern "C" fn pass_numpy_to_rust<'a>(data: *mut f32, ndata: usize)  -> *const RustStruct<'a> {

    // Practice passing data pointers to Rust
    let data_slice = unsafe { std::slice::from_raw_parts(data, ndata) };
    let rlst = rlst_array_from_slice1!(data_slice, [ndata]);
    let b = Box::new(RustStruct {foo: data_slice});
    println!("Passing Numpy array to RLST, {:?}", rlst.data());
    b.foo();
    Box::into_raw(b)
}

pub struct RustStruct<'a> {
    pub foo: &'a [f32]
}

impl RustStruct <'_> {
    fn foo(&self) {
        println!("Calling foo from a Rust struct {:?}", self.foo);
    }
}
