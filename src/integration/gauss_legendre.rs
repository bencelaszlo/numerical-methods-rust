fn q_gauss_legendre(a: f32, b: f32) -> f32 {
	let xr: f32;
	let xm: f32;
	let mut dx: f32;
	let mut s: f32;
	
	const X: [f32; 6] = [0.0, 0.1488743389, 0.4333953941, 0.6794095682, 0.8650633666,0.9739065285];
	const W: [f32; 6] = [0.0, 0.2955242247, 0.2692667193, 0.2190863625, 0.1494513491, 0.0666713443];
	
	xm = 0.5 * (b + a);
	xr = 0.5 * (b - a);
	
	s = 0.0;
	
	for j in 0..5 {
		dx = xr * X[j];
		s += W[j] * parameter_function(xm + dx) + parameter_function(xm - dx);
	}
	
	let result: f32 = s * xr;
	return result;
}

/*fn gauss_hermite(a: f32, b: f32) {
	const EPS: f32 = 3.0 / 100000000000000;
	const PI_M4: f32 = 0.7511255444649425;
	const MAXIT: i32 = 10;
	
	
}*/

fn parameter_function(x: f32) -> f32 {
	return x * x; //Define parameter function here.
}

fn main() {
	println!("{}", q_gauss_legendre(0.0, 10.0));
}