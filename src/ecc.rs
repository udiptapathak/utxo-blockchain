// refer: https://www.ams.org/journals/mcom/1987-48-177/S0025-5718-1987-0866109-5/S0025-5718-1987-0866109-5.pdf

use crate::arithmetic256::*;

#[derive(Clone)]
struct EcPoint {
	x: u256,
	y: u256
}

struct EllCurve {
	x: u256,
	y: u256,
	p: u256
}

impl EllCurve {
	pub fn add(&self, a: &mut EcPoint, b: &EcPoint) {
		let mut base = &mut self.p.clone();
		let mut t0 = a.x.clone();
		t0.subm(&b.x, base); // t0 = x1 - x2
		t0.inv(&mut base); // t0 = 1 / (x1 - x2)
		a.y.subm(&b.y, base); // y1 = y1 - y2
		a.y.mulm(&t0, base); // y1 = (y1 - y2) * 1 / (x1 - x2) = alpha
		let mut t1 = u256::new(&[0;2]); // t1 = 0
		t1.subm(&a.x, base); // t1 = - x1
		t1.subm(&b.x, base); // t1 = - x1 - x2
		t0.copy(&a.y); // t0 = alpha
		t0.mulm(&a.y, base); // t0 = alpha * alpha
		t1.addm(&t0, base); // t1 = - x1 - x2 + alpha * alpha
		a.x.copy(&t1); // x1 = t1 = x3
		t0 = u256::new(&[0;2]); // t0 = 0
		t0.subm(&b.y, base); // t0 = - y2
		t1.copy(&b.x); // t1 = x2
		t1.subm(&a.x, base); // t1 = x2 - x3
		t1.mulm(&a.y, base); // t1 = alpha * (x2 - x3)
		t0.addm(&t1, base); // t0 = -y2 + alpha * (x2 - x3);
		a.y.copy(&t0) // y1 = t0 = y3
	}
}

