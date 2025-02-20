//! Poseidon Hash Functions for Ethereum
//!
//! Implements poseidon family of hash function over finite fields.
//! Hash functions depend on a set of parameters tightly linked to security.
//! Multiple sets of parameters are included in the crate. However, if users
//! wish so, they can provide their own set of parameters.
//!
//! Hash functions are exported through a C ABI in a shared library.
//! This allows the functions to be called in geth from golang.
//!
//! # Examples
//! Hash functions are named hash_<params>, where <params> is the name of the
//! set of parameters to be used. They take slices of field elements as input
//! and returns a vector of field elements representing the hash value. For
//! example:
//!
//! ```
//! use poseidon::hash_s128b;
//! use poseidon::parameters::s128b::GF;
//! let inputs = vec![GF::from(7), GF::from(54)];
//! let h = hash_s128b(&inputs);
//! ```

// Implementation is done for PrimeFields.
// Question remains of how to handle BinaryFields.
// Other fields are probably not useful at this point.
#![cfg_attr(
    any(target_arch = "wasm32", not(feature = "std")),
    no_std,
    feature(alloc_error_handler)
)]

#[macro_use]
extern crate alloc;

use alloc::alloc::*;
use alloc::vec::Vec;

#[derive(Default)]
pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        libc::malloc(layout.size()) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        libc::free(ptr as *mut libc::c_void);
    }
}

#[cfg(not(test))]
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout)
}

/// The static global allocator.
#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

pub mod convert;
use convert::{felts_from_u8s, u8s_from_felts};

pub mod fields;

pub mod permutation;
pub use permutation::{hash, Poseidon};

pub mod parameters;
pub use parameters::pallas;
pub use parameters::s128b;
pub use parameters::sw2;
pub use parameters::sw3;
pub use parameters::sw4;
pub use parameters::sw8;
pub use parameters::vesta;
// add more parameters here.

pub fn hash_s128b(inputs: &[s128b::GF]) -> Vec<s128b::GF> {
    hash::<s128b::GF>(inputs, &s128b::PARAMS).unwrap()
}

// C-Interface for the hash function
#[no_mangle]
pub extern "C" fn c_hash_s128b(
    input: *const u8,
    input_len: usize,
    output: *mut u8,
    output_len: usize,
) -> usize {
    let input = unsafe {
        assert!(!input.is_null());
        core::slice::from_raw_parts(input, input_len)
    };
    let input = felts_from_u8s(&input);

    let result = hash_s128b(&input);
    let result = u8s_from_felts(&result);

    let count = result.len().min(output_len);
    // let src = result.as_ptr();
    let output = unsafe {
        assert!(!output.is_null());
        core::slice::from_raw_parts_mut(output, output_len)
    };
    output.copy_from_slice(&result);
    count
}

pub fn hash_sw2(inputs: &[sw2::GF]) -> Vec<sw2::GF> {
    hash::<sw2::GF>(inputs, &sw2::PARAMS).unwrap()
}

// C-Interface for the hash function
#[no_mangle]
pub extern "C" fn c_hash_sw2(
    input: *const u8,
    input_len: usize,
    output: *mut u8,
    output_len: usize,
) -> usize {
    let input = unsafe {
        assert!(!input.is_null());
        core::slice::from_raw_parts(input, input_len)
    };
    let input = felts_from_u8s(&input);

    let result = hash_sw2(&input);
    let result = u8s_from_felts(&result);

    let count = result.len().min(output_len);
    // let src = result.as_ptr();
    let output = unsafe {
        assert!(!output.is_null());
        core::slice::from_raw_parts_mut(output, output_len)
    };
    output.copy_from_slice(&result);
    count
}

pub fn hash_sw3(inputs: &[sw3::GF]) -> Vec<sw3::GF> {
    hash::<sw3::GF>(inputs, &sw3::PARAMS).unwrap()
}

pub fn hash_sw4(inputs: &[sw4::GF]) -> Vec<sw4::GF> {
    hash::<sw4::GF>(inputs, &sw4::PARAMS).unwrap()
}

pub fn hash_sw8(inputs: &[sw8::GF]) -> Vec<sw8::GF> {
    hash::<sw8::GF>(inputs, &sw8::PARAMS).unwrap()
}

pub fn hash_pallas(inputs: &[pallas::GF]) -> Vec<pallas::GF> {
    hash::<pallas::GF>(inputs, &pallas::PARAMS).unwrap()
}

pub fn hash_vesta(inputs: &[vesta::GF]) -> Vec<vesta::GF> {
    hash::<vesta::GF>(inputs, &vesta::PARAMS).unwrap()
}

#[cfg(not(test))]
#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
