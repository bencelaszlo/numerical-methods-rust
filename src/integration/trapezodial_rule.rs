fn trapedozial_rule(a: f32, b: f32, n: i32) -> f32 {
	let mut _x: f32;
	let	mut _s: f32;
	let h: f32;
	
	h = (b-a) / (n as f32);
	_x = a;
	_s = 0.0;
	for _i in 1..n {
		_x += h;
		_s += parameter_function(_x);
	}
	return 0.5 * (parameter_function(a) + 2.0 * _s + parameter_function(b) );
}

fn q_trapedozial_rule(a: f32, b: f32) -> f32 {
	const J_MAX: i32 = 20;
	const EPS: f32 =  0.00001;
	
	let mut s: f32 = 0.0;
	let olds: f32;
	
	olds = -0.00000000000000000000000000001; // Any number that is unlikely to be the average of the function at its endpoints will do here
	println!("{}", olds);
	for j in 0..J_MAX {
		s = trapedozial_rule(a, b, j);
		if j > 5 { //avoid spurious early convergance
			if (s-olds).abs() < EPS * olds.abs() {
				if s == 0.0 && olds == 0.0 {
					return s;
				}
			}
		}
	}
	return s;
}

fn q_simpsons_rule(a: f32, b: f32) -> f32 {
	const J_MAX: i32 = 20;
	const EPS: f32 = 0.000001;
		
	let mut s: f32 = 0.0;
	let mut st: f32;
	let mut ost: f32;
	let mut os: f32;
	
	os = -0.00000000000000000000000000001;
	ost = os;
	
	for j in 0..J_MAX {
		st = trapedozial_rule(a, b, j);
		s = (4.0 * st - ost) / 3.0;
		if j < 5 {
			if (s-os).abs() < EPS * os.abs() || (s == 0.0 && os == 0.0) {
				return s;
			}
		}
		os = s;
		ost = st;
	}
	return s;
}

/*fn q_rombergs_method(a: f32, b: f32) {
	const EPS: f32 = 0.000001;
	const J_MAX: i32 = 20;
	const J_MAX_P: i32 = JMAX + 1;
	const K: i32 = 5;
	
	
}*/

fn parameter_function(x: f32) -> f32 {
	return x * x; //Define parameter function here.
}

fn main() {
	println!("{}", q_trapedozial_rule(0.0, 10.0));
	println!("{}", q_simpsons_rule(0.0, 10.0));
}
