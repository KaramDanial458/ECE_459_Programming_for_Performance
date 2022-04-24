// This is the skeleton for the CUDA implementation

use rustacuda::memory::DeviceCopy;
use rustacuda::memory::DeviceBox;
use rustacuda::launch;
use rustacuda::prelude::*;
use std::error::Error;
use std::ffi::CString;
use rustacuda::function::BlockSize;
use crate::SIZE;

// References for declaring struct:
// https://bheisler.github.io/RustaCUDA/rustacuda/memory/trait.DeviceCopy.html#how-can-i-implement-devicecopy

pub struct CudaContext {
    module: Module,
    stream: Stream,
    _context: Context,
}

impl CudaContext {
    pub fn init() -> Result<Self, Box<dyn Error>> {
        // Set up code from lecture 22
        rustacuda::init(CudaFlags::empty())?;
        let device = Device::get_device(0)?;
        let _ctx = Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;

        let ptx = CString::new(include_str!("../resources/weightedsum.ptx"))?;

        let module_tmp = Module::load_from_string(&ptx)?;
        let stream_tmp = Stream::new(StreamFlags::NON_BLOCKING, None)?;

        let cuda_struct = CudaContext {
            module: module_tmp,
            stream: stream_tmp,
            _context: _ctx
        };

        Ok(cuda_struct)
    }

    #[allow(non_snake_case)]
    pub fn compute(&mut self, P:Vec<f32>, A:Vec<f32>, sums:&mut Vec<f32>) -> Result<usize, Box<dyn Error>> {

        let mut device_P = DeviceBuffer::from_slice( &P ).unwrap();
        let mut device_A = DeviceBuffer::from_slice( &A ).unwrap();
        let mut device_S = DeviceBuffer::from_slice( &sums ).unwrap(); 

        let output_vec = vec![0.0; (SIZE + 2)];
        let output_idx_vec = vec![0; (SIZE + 2)];

        let mut output = DeviceBuffer::from_slice( &output_vec ).unwrap();
        let mut output_idx = DeviceBuffer::from_slice( &output_idx_vec ).unwrap();

        let impl_module = &self.module;
        let impl_stream = &self.stream;
        let sub_matrix_size = BlockSize::xy(16 as u32, 16 as u32);

        unsafe {
            let sum_result = launch!(impl_module.add<<<1, (SIZE + 2) as u32, 0, impl_stream>>>(
                device_P.as_device_ptr(),
                device_A.as_device_ptr(),
                device_S.as_device_ptr()
            ));

            let max_result = launch!(impl_module.find_max_index<<<1, (SIZE + 2) as u32, 0, impl_stream>>>(
                output.as_device_ptr(),
                output_idx.as_device_ptr(),
                device_S.as_device_ptr()
            ));
        }

        self.stream.synchronize()?;

        let mut output_vector = vec![0.0; (SIZE + 2)];
        let mut output_idx_vector = vec![0; (SIZE + 2)];

        output.copy_to(&mut output_vector)?;
        output_idx.copy_to(&mut output_idx_vector)?;

        let mut result = output_vector[0];
        let mut result_idx = output_idx_vector[0];

        println!("twoD Index: {}", result_idx);
        
        /*
        println!("output vector {:?}", output_vector);
        println!("output idx vector {:?}", output_idx_vector);

        for i in 0..output_idx_vector.len() {
            if output_vector[i] > result {
                result = output_vector[i];
                result_idx = output_idx_vector[i];
            }
        }*/

        Ok(result_idx as usize)
    }
}
