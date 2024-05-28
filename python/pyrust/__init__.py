import numpy as np

from ._pyrust import lib, ffi

def calling_blas():
    return lib.calling_blas()

def pass_numpy_to_rust():
    data = np.arange(10).astype(np.float32)
    data_ptr = ffi.from_buffer('float *', data)
    return lib.pass_numpy_to_rust(data_ptr, 10)
