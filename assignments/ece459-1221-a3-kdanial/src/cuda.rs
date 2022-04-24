// This is the skeleton for the CUDA implementation

use crate::cnn::*;
use rustacuda::function::BlockSize;
use rustacuda::launch;
use rustacuda::memory::DeviceBox;
use rustacuda::prelude::*;
use std::error::Error;
use std::ffi::CString;

// Fields need to be ordered this way so the DeviceBoxes are
// dropped before the Context. Otherwise the drop will panic.

pub struct CudaContext {
    conv_layer: DeviceBox<ConvLayer>,
    output_layer: DeviceBox<OutputLayer>,
    module: Module,
    stream: Stream,
    _context: Context,
}

impl CudaContext {
    pub fn init(cnn: &Cnn) -> Result<Self, Box<dyn Error>> {
        // Use code from lecture 22 Part 2
        rustacuda::init(CudaFlags::empty())?;
        let device = Device::get_device(0)?;
        let _ctx = Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;

        let ptx = CString::new(include_str!("../kernel/kernel.ptx"))?;

        // Declare CudaContext struct
        let convolutional_layer_tmp = DeviceBox::new(&cnn.conv_layer).unwrap();
        let output_layer_tmp = DeviceBox::new(&cnn.output_layer).unwrap();

        /*
        // Check to make sure matrix not empty. Realized generate does not provide null matrices
        // and erased functionality
        
        if convolutional_layer_tmp == None {
            Err("Empty Convolutional Layer");
        }
        if output_layer_tmp == None {
            Err("Empty Output Layer");
        }
        */

        let module_tmp = Module::load_from_string(&ptx)?;
        let stream_tmp = Stream::new(StreamFlags::NON_BLOCKING, None)?;

        let cuda_struct = CudaContext {
            conv_layer: convolutional_layer_tmp,
            output_layer: output_layer_tmp,
            module: module_tmp,
            stream: stream_tmp,
            _context: _ctx
        };

        // println!("Cuda Context Established");
        Ok(cuda_struct)
    }

    pub fn compute(&mut self, input: &InputMatrix) -> Result<OutputVec, Box<dyn Error>> {
        // Note, capital case constants are defined in kernal.cu which were coppied from the provided cnn.rs file.
        
        // Documentation Used
        // BlockSize: https://docs.rs/rustacuda/0.1.0/rustacuda/function/struct.BlockSize.html#method.xy

        let impl_module = &self.module;
        let impl_stream = &self.stream;
        let sub_matrix_size = BlockSize::xy(CONV_OUT_DIM as u32, CONV_OUT_DIM as u32);
        let mut image_matrix = DeviceBox::new(input).unwrap();
        
        /*
        if image_matrix == None {
            Err("Image Is Empty");
        }
        
        image_matrix = image_matrix.unwrap();
        */

        let mut convolution_output = DeviceBox::new(&[[[0.0; CONV_OUT_DIM]; CONV_OUT_DIM]; CONV_LAYER_SIZE]).unwrap();
        
        // OutputVec declared in cnn.rs starter code. Hinted to be used when imported in the
        // beginning of this file.
        let mut result = OutputVec([0.0; OUT_LAYER_SIZE]);
        let mut output_vec = DeviceBox::new(&result)?;
        
        // println!("size of sub_matrix: {}", sub_matrix_size);

        unsafe {
            // Convolution
            // Note, added as u32 to CONV_LAYER_SIZE to avoid From<usize> error
            let conv_result = launch!(impl_module.Convolution<<<CONV_LAYER_SIZE as u32, &sub_matrix_size, 0, impl_stream>>>(
                image_matrix.as_device_ptr(),
                self.conv_layer.as_device_ptr(),
                convolution_output.as_device_ptr()
            ));

            // Rectified Linear Unit
            let relu_result = launch!(impl_module.LinearRectifier<<<CONV_LAYER_SIZE as u32, sub_matrix_size, 0, impl_stream>>>(
                convolution_output.as_device_ptr()
            ));
            
            // Output
            let output_result = launch!(impl_module.Output<<<OUT_LAYER_SIZE as u32, 100 as u32, 0, impl_stream>>>(
                convolution_output.as_device_ptr(),
                self.output_layer.as_device_ptr(),
                output_vec.as_device_ptr()
            ));
        }

        self.stream.synchronize()?;
        // println!("Kernel Completed!");
        output_vec.copy_to(&mut result)?;
        Ok(result)
    }
}
