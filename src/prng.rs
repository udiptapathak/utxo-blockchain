/*
 * prng.rs
 *
 * This file is part of the project - utxo-blockchain
 * authored by Udipta Pathak (udiptapathak00@gmail.com)
 *
 * Source code may be used and modified by anyone to produce their work in any
 * form under the condition: give credit to this project where it is used.
 *
 * This project comes without warranty.
 *
 * Further refer to the license attached to the project root.
 */

use crate::arithmetic256::*;
use core::ffi::*;
use std::mem;

#[link(name = "c")]
extern {
	fn srand(value: u32);
	fn rand() -> u32;
	fn fopen(fname: *const u8, access: *const u8) -> *const u64;
	fn fread(buf: *const u8, size: usize, n: usize, file: *const u64) -> usize;
}

pub fn random32() -> u32 {
	let mut seed = 0_u32;
	let mut res = 0_u32;
	unsafe {
		let urandom = fopen(String::from("/dev/urandom").as_ptr(),
			String::from("r").as_ptr());
		let seed_ptr = &mut seed as *mut u32;
		fread(seed_ptr as *mut u8, 4, 1, urandom);
		srand(*seed_ptr);
		let buf = &mut res as *mut u32;
		let mut i = 0;
		while i < 4 {
			*buf.offset(i) = rand();
			i += 1;
		}
	}
	res
}

pub fn random256() -> u256 {
	let mut seed = 0_u32;
	let mut res = u256::new(&[0;2]);
	unsafe {
		let urandom = fopen(String::from("/dev/urandom").as_ptr(),
			String::from("r").as_ptr());
		let seed_ptr = &mut seed as *mut u32;
		fread(seed_ptr as *mut u8, 4, 1, urandom);
		srand(*seed_ptr);
		let buf = mem::transmute::<&mut u256, *mut u32>(&mut res);
		let mut i = 0;
		while i < 8 {
			*buf.offset(i) = rand();
			i += 1;
		}
	}
	res
}
