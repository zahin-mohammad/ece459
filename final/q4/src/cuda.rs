use crate::{ElectionOutcome, InputMatrix, OutputMatrix, NUM_RUNS};
use rustacuda::memory::DeviceBox;
use rustacuda::prelude::*;
use rustacuda::{function::GridSize, launch};
use std::error::Error;
use std::ffi::CString;

pub struct CudaContext {
    module: Module,
    stream: Stream,
    _context: Context,
}

impl CudaContext {
    pub fn init() -> Result<Self, Box<dyn Error>> {
        println!("Initializing CudaContext...");
        // Initialize the CUDA API
        rustacuda::init(CudaFlags::empty()).unwrap();

        println!("Init OK");
        // Get the first device
        let device = Device::get_device(0)?;

        println!("Got Device");
        // Create a context associated to this device
        let context =
            Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;

        println!("Ready to load Kernel...");
        // Load the module containing the function we want to call
        let module_data = CString::new(include_str!("../kernel/kernel.ptx"))?;
        let module = Module::load_from_string(&module_data)?;

        // Create a stream to submit work to
        let stream = Stream::new(StreamFlags::NON_BLOCKING, None)?;

        Ok(Self {
            module,
            stream,
            _context: context,
        })
    }

    pub fn compute(&mut self, input: &InputMatrix) -> Result<Vec<ElectionOutcome>, Box<dyn Error>> {
        let mut input = DeviceBox::new(input).unwrap();

        let mut out = OutputMatrix([[0; 2]; NUM_RUNS]);
        let mut _out = DeviceBox::new(&out)?;

        let stream = &self.stream;
        let module = &self.module;

        let threads_per_block = 32;
        let num_blocks = GridSize::xy(800, 625); // 800 * 625 = NUM_RUNS

        unsafe {
            let _ = launch!(module.Vote<<<num_blocks, threads_per_block, 0, stream>>>(
                input.as_device_ptr(),
                _out.as_device_ptr()
            ));
        }
        self.stream.synchronize()?;
        _out.copy_to(&mut out)?;

        let mut output: Vec<ElectionOutcome> = Vec::new();
        for i in 0..NUM_RUNS {
            output.push(ElectionOutcome {
                a_votes: out.0[i][0],
                b_votes: out.0[i][1],
            });
        }
        Ok(output)
    }
}
