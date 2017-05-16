use parity_wasm::interpreter::{self, ModuleInstanceInterface};
use {alloc, runtime};

use {DEFAULT_MEMORY_INDEX, WasmMemoryPtr};

fn write_u32(dst: &mut [u8], val: u32) {
    dst[0] = (val & 0x000000ff) as u8;
    dst[1] = ((val & 0x0000ff00) >> 8) as u8;
    dst[2] = ((val & 0x00ff0000) >> 16) as u8;
    dst[3] = ((val & 0xff000000) >> 24) as u8;
}

#[derive(Debug)]
pub enum Error {
    Allocator(alloc::Error),
    Interpreter(interpreter::Error),
}

impl From<alloc::Error> for Error {
    fn from(err: alloc::Error) -> Self {
        Error::Allocator(err)
    }
}

impl From<interpreter::Error> for Error {
    fn from(err: interpreter::Error) -> Self {
        Error::Interpreter(err)
    }
}

pub fn init(
    env: &interpreter::ModuleInstanceInterface, 
    runtime: &runtime::Runtime,
    input: &[u8],
) -> Result<WasmMemoryPtr, Error> {
    let mut input_ptr_slc = [0u8; 4];
    let mut input_length = [0u8; 4];

    let allocator = runtime.allocator();

    let descriptor_ptr = allocator.alloc(16)?;

    println!("descriptor_ptr: {}", descriptor_ptr);
    let memory = env.memory(DEFAULT_MEMORY_INDEX)?;

    if input.len() > 0 {
        let input_ptr = allocator.alloc(input.len() as u32)?;
        write_u32(&mut input_ptr_slc, input_ptr);
        write_u32(&mut input_length, input.len() as u32);
        memory.set(input_ptr, input)?;
        println!("input_ptr: {}", input_ptr);
    } else {
        write_u32(&mut input_ptr_slc, 0);
        write_u32(&mut input_length, 0);
    }

    memory.set(descriptor_ptr, &input_ptr_slc)?;
    memory.set(descriptor_ptr+4, &input_length)?;

    // zero result ptr/len
    memory.set(descriptor_ptr+8, &[0u8; 4])?;
    memory.set(descriptor_ptr+12, &[0u8; 4])?;

    println!("descriptor: {:?}", memory.get(descriptor_ptr, 16));

    Ok(descriptor_ptr as i32)
}

fn read_u32(slc: &[u8]) -> u32 {
    use std::ops::Shl;
    (slc[0] as u32) + (slc[1] as u32).shl(8) + (slc[2] as u32).shl(16) + (slc[3] as u32).shl(24)
}

// pub fn retrieve(env: 
//     env: &interpreter::ModuleInstanceInterface, 
//     runtime: &runtime::Runtime,
//     descriptor_ptr: u32,
//     result_ptr: u32,
// ) -> Result<Vec<u8>, Error> {
//     let memory = env.memory(DEFAULT_MEMORY_INDEX)?;

// }