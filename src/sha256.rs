/*
 * sha256.rs
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

// refer: https://eips.ethereum.org/assets/eip-2680/sha256-384-512.pdf

use std::mem;
use core::slice::from_raw_parts;
use core::slice::from_raw_parts_mut;

// These are the first thirty-two bits of the fractional parts of the cube
// roots of the first sixty-four primes.
const K: [u32;64] = [
	0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
	0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
	0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
	0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
	0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
	0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
	0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
	0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
	0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
	0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
	0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
];

#[allow(non_snake_case)]
fn Ch(x: u32, y: u32, z: u32) -> u32 {
	(x & y) ^ (!x & z) 
}

#[allow(non_snake_case)]
fn Maj(x: u32, y: u32, z: u32) -> u32 {
	(x & y) ^ (x & z) ^ (y & z)
}

#[allow(non_snake_case)]
fn S(x: u32, bit: u8) -> u32 {
	let t = x & (!0 >> (32 - bit));
	(x >> bit) | (t << (32 - bit))
}

#[allow(non_snake_case)]
fn R(x: u32, bit: u8) -> u32 {
	x >> bit
}

#[allow(non_snake_case)]
fn Sigma_0(x: u32) -> u32 {
	S(x, 2) & S(x, 13) & S(x, 22)
}

#[allow(non_snake_case)]
fn Sigma_1(x: u32) -> u32 {
	S(x, 6) & S(x, 11) & S(x, 25)
}

fn sigma_0(x: u32) -> u32 {
	S(x, 7) & S(x, 18) & R(x, 3)
}

fn sigma_1(x: u32) -> u32 {
	S(x, 17) & S(x, 19) & R(x, 10)
}

fn get_t1(r: &[u32], j: usize, w: &[u32]) -> u32 {
	let a = r[7].wrapping_add(Sigma_1(r[4]));
	let b = Ch(r[4], r[5], r[6]).wrapping_add(K[j]);
	return a.wrapping_add(b).wrapping_add(w[j])
}

fn get_t2(r: &[u32]) -> u32 {
	Sigma_0(r[0]).wrapping_add(Maj(r[0], r[1], r[2]))
}



struct Data<'a> {
	buf: &'a[u8],
	cur: usize,
	total: usize
}

impl<'a> Data<'a> {
	#[inline(always)]
	fn blind_copy_block(&mut self, buf: &mut [u32]) -> usize {
		let mut i = 0;
		let block = unsafe {
			from_raw_parts_mut(buf.as_ptr() as *mut u64, 8)
		};
		let data = unsafe {
			from_raw_parts(self.buf.as_ptr().add(self.cur) as *mut u64, 8)
		};
		while i < 8 {
			block[i] = data[i];
			i += 1;
		}
		self.cur += 64;
		i
	}

	#[inline(always)]
	fn safe_copy_block(&mut self, buf: &mut [u32]) -> usize {
		let mut i = 0;
		let block = unsafe {
			from_raw_parts_mut(buf.as_ptr() as *mut u8, 64)
		};
		while i < 64 && self.cur < self.total {
			block[i] = self.buf[self.cur];
			i += 1;
			self.cur += 1;
		}
		i
	}

	#[inline(always)]
	fn get_next_block(&mut self, buf: &mut [u32]) -> bool {
		if self.cur == self.total {
			reset_block(buf);
			pad_length(buf, self.total);
			return false;
		}
		let i = if self.cur + 64 < self.total {self.blind_copy_block(buf)}
		else {self.safe_copy_block(buf)};
		if i == 64 {
			if self.cur == self.total {false}
			else {true}
		} else {pad_block(i, self.total, buf)}
	}
}

#[inline(always)]
fn pad_length(block: &mut [u32], len: usize) {
	unsafe {
		let len = &len as *const usize as *const u32;
		block[14] = *len.offset(0);
		block[15] = *len.offset(1);
	}
}

#[inline(always)]
fn reset_block(buf: &mut [u32]) {
	let block = unsafe {from_raw_parts_mut(buf.as_ptr() as *mut u64, 8)};
	let mut i = 0;
	while i < 8 {
		block[i] = 0;
		i += 1;
	}
}

#[inline(always)]
fn pad_block(i: usize, total: usize, buf: &mut [u32]) -> bool {
	let mut i = i;
	let block = unsafe {
		from_raw_parts_mut(buf.as_ptr() as *mut u8, 64)
	};
	block[i] = 1 << 7;
	i += 1;
	if i > 56 {
		while i < 64 {
			block[i] = 0;
			i += 1;
		}
		true
	}
	else {
		while i < 56 {
			block[i] = 0;
			i += 1;
		}
		pad_length(buf, total);
		false
	}
}

#[allow(non_snake_case)]
fn compute_W(data: &mut Data, w: &mut [u32;64]) -> bool {
	let buf = unsafe {from_raw_parts_mut(w.as_ptr() as *mut u32, 16)};
	let result = data.get_next_block(buf);
	let mut i = 16;
	while i < 64 {
		w[i] = ((sigma_1(w[i - 2]) as u64
			+ w[i - 7] as u64) as u64
			+ (sigma_0(w[i - 15]) as u64
			+ w[i - 16] as u64) as u64) as u32;
		i += 1;
	}
	result
}

fn do_round(j: usize, r: &mut [u32;8], w: &[u32;64]) {
	let t1 = get_t1(r, j, w);
	let t2 = get_t2(r);
	let mut i = 7;
	while i > 0 {
		r[i] = r[i - 1];
		i -= 1;
	}
	r[4] = r[4].wrapping_add(t1);
	r[0] = t1.wrapping_add(t2);	
}

pub fn sha256(data: &[u8]) -> [u64;4] {
	// initial hash value is obtained by taking the fractional parts of the
	// square roots of the first eight primes
	let mut data = Data {buf: data, cur: 0, total: data.len()};
	let mut state: [u32;8] = [
		0x6a09e667,
		0xbb67ae85,
		0x3c6ef372,
		0xa54ff53a,
		0x510e527f,
		0x9b05688c,
		0x1f83d9ab,
		0x5be0cd19
	];
	let mut r: [u32;8] = [0;8];
	let mut w: [u32;64] = [0;64];
	let mut result = true;
	while result {
		let mut i = 0; 
		while i < 8 {
			r[i] = state[i];
			i += 1;
		}
		let mut i = 0;
		result = compute_W(&mut data, &mut w);
		while i < 64 {
			do_round(i, &mut r, &w);
			i += 1
		}
		let mut i = 0;
		while i < 8 {
			state[i] = state[i].wrapping_add(r[i]);
			i += 1;
		}
		let mut i = 0;
		while i < 8 {
			i += 1
		}
	}
	unsafe {mem::transmute::<[u32;8],[u64;4]>(state)}
}
