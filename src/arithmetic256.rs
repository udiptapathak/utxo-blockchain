/*
 * arithmetic256.rs
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

use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct u256 {
	data: [u128;2],
}

impl u256 {
	pub fn new(data: &[u128;2]) -> u256 {
		let mut ret_data = [0_u128;2];
		let mut i = 0;
		while i < 2 {
			ret_data[i] = data[1 - i];
			i += 1;
		}
		u256 {data: ret_data}
	}

	pub fn add(&mut self, other: &u256) {
		let carry: bool;
		(self.data[0], carry) = self.data[0]
			.overflowing_add(other.data[0]);
		self.data[1] = self.data[1].wrapping_add(other.data[1])
			.wrapping_add(if carry {1} else {0});
	}

	pub fn sub(&mut self, other: &u256) {
		let carry: bool;
		(self.data[0], carry) = self.data[0]
			.overflowing_sub(other.data[0]);
		self.data[1] = self.data[1].wrapping_sub(other.data[1])
			.wrapping_sub(if carry {1} else {0});
	}

	pub fn shl(&mut self) {
		self.data[1] <<= 1;
		self.data[1] |= self.data[0] >> 127;
		self.data[0] <<= 1;
	}

	pub fn shr(&mut self) {
		self.data[0] >>= 1;
		self.data[0] |= self.data[1] & 1;
		self.data[1] >>= 1;
	}

	fn isset(&self, bit: u8) -> bool {
		if bit < 128 {
			self.data[0] & (1 << bit) != 0
		} else {
			self.data[1] & (1 << (bit - 128)) != 0
		}
	}

	pub fn mul(&mut self, other: &u256) {
		let mut res = u256 {data: [0_u128;2]};
		let mut multiplier = other.clone();
		let mut i = 0;
		loop {
			if self.isset(i) {res.add(&multiplier);}
			multiplier.shl();
			if i == 255 {break;} 
			i += 1
		}
		self.data = res.data;
	}

	pub fn div(&mut self, other: &mut u256) -> u256 {
		let mut bit: u8 = 0;
		let mut rem = self.clone();
		while other.le(&rem) {
			other.shl();
			bit += 1;
		}
		self.data = [0_u128;2];
		loop {
			self.shl();
			if other.le(&rem) {
				rem.sub(&other);
				self.data[0] |= 1;
			}
			if bit == 0 {break;}
			other.shr();
			bit -= 1;
		}
		rem
	}

	pub fn inv(&mut self, base: &mut u256) {
		if self.data[1] == 0 && self.data[0] <= 1 {return;}
		let mut m = base.clone();
		self.data = m.div(self).data;
		self.inv(base);
		self.mul(&m);
		self.data = self.div(base).data;
		m = base.clone();
		m.sub(self);
		self.data = m.data;
	}

	pub fn le(&self, other: &u256) -> bool {
		if self.data[1] > other.data[1] {false}
		else if self.data[1] == other.data[1]
		&& self.data[0] > other.data[0] {false}
		else {true}
	}

	pub fn lt(&self, other: &u256) -> bool {
		if self.data[1] < other.data[1] {true}
		else if self.data[1] == other.data[1]
		&& self.data[0] < other.data[0] {true}
		else {false}
	}

	pub fn residue(&mut self, other: &mut u256) {
		let mut bit: u8 = 0;
		while other.le(self) {
			other.shl();
			bit += 1;
		}
		self.data = [0_u128;2];
		loop {
			self.shl();
			if other.le(self) {self.sub(&other);}
			if bit == 0 {break;}
			other.shr();
			bit -= 1;
		}
	}

	pub fn copy(&mut self, other: &u256) {
		self.data = other.data;
	}

	#[inline(always)]
	pub fn addm(&mut self, other: &u256, base: &mut u256) {
		self.add(other);
		self.residue(base);
	}

	#[inline(always)]
	pub fn subm(&mut self, other: &u256, base: &mut u256) {
		if self.le(other) {self.add(base);}
		self.sub(other);
		self.residue(base);
	}

	#[inline(always)]
	pub fn mulm(&mut self, other: &u256, base: &mut u256) {
		self.mul(other);
		self.residue(base);
	}
}

impl fmt::Display for u256 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "{:032x}{:032x}", self.data[1], self.data[0])
	}
}
