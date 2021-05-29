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
        rustacuda::init(CudaFlags::empty())?;
        let device = Device::get_device(0)?;
        let ctx = Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;
        let ptx = CString::new(include_str!("../kernel/kernel.ptx"))?;
        let cuda_context = CudaContext{
            conv_layer: DeviceBox::new(&cnn.conv_layer).unwrap(),
            output_layer: DeviceBox::new(&cnn.output_layer).unwrap(),
            module: Module::load_from_string(&ptx)?,
            stream: Stream::new(StreamFlags::NON_BLOCKING, None)?,
            _context: ctx,
        };
        Ok(cuda_context)
    }

    pub fn compute(&mut self, input: &InputMatrix) -> Result<OutputVec, Box<dyn Error>> {
        // Initialize input/output buffers
        let mut input = DeviceBox::new(input).unwrap();
        let mut _conv_output = DeviceBox::new(&[[[0.0; CONV_OUT_DIM]; CONV_OUT_DIM]; CONV_LAYER_SIZE])?;
        let mut output = OutputVec([0.0; OUT_LAYER_SIZE]);
        let mut _output = DeviceBox::new(&output)?;

        let block_size = BlockSize::xy(CONV_OUT_DIM as u32, CONV_OUT_DIM as u32);
        let stream = &self.stream;
        let module = &self.module;

        unsafe {
            // Layer 1: Convolution Layer + ReLu
            let _ = launch!(module.ConvolutionLayerAndReLU<<<CONV_LAYER_SIZE as u32, block_size, 0, stream>>>(
                input.as_device_ptr(),
                self.conv_layer.as_device_ptr(),
                _conv_output.as_device_ptr()
            ));
            // Layer 2: Output Layer
            let _ = launch!(module.OutputLayer<<<OUT_LAYER_SIZE as u32, 32 as u32, 0, stream>>>(
                _conv_output.as_device_ptr(),
                self.output_layer.as_device_ptr(),
                _output.as_device_ptr()
            ));
        }

        // Wait for Kernel to finish executing
        self.stream.synchronize()?;
        _output.copy_to(&mut output)?;
        Ok(output)
    }
}
