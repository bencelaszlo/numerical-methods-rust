fn parameter_function(x: f32) -> f32 {
    return x * x - 1.0; //Define parameter function here.
}

pub fn trapedozial_rule(a: f32, b: f32, n: i32) -> f32 {
    let mut x: f32;
    let mut s: f32;
    let h: f32;

    h = (b - a) / (n as f32);
    x = a;
    s = 0.0;

    for _ in 1..n {
        x += h;
        s += parameter_function(x);
    }
    return ((b - a) / n as f32) * 0.5 * (parameter_function(a) + 2.0 * s + parameter_function(b));
}

pub fn q_trapedozial_rule(a: f32, b: f32, j_max: i32) -> f32 {
    const EPS: f32 = 0.00000001; //accurancy
    const J_MIN_ITERATION_COUNT: i32 = 5;

    let mut s: f32 = 0.0;
    let olds: f32;

    olds = -0.00000000000000000000000000001; // Any number that is unlikely to be the average of the function at its endpoints will do here
    for j in 0..j_max {
        s = trapedozial_rule(a, b, j);
        if j > J_MIN_ITERATION_COUNT {
            //avoid spurious early convergance
            if (s - olds).abs() < EPS * olds.abs() {
                if s == 0.0 && olds == 0.0 {
                    return s;
                }
            }
        }
    }
    return s;
}

pub fn q_simpsons_rule(a: f32, b: f32, j_max: i32) -> f32 {
    const EPS: f32 = 0.000000001; //accurancy
    const J_MIN_ITERATION_COUNT: i32 = 5;

    let mut s: f32 = 0.0;
    let mut st: f32;
    let mut ost: f32;
    let mut os: f32;

    os = -0.00000000000000000000000000001;
    ost = os;

    for j in 0..j_max {
        st = trapedozial_rule(a, b, j);
        s = (4.0 * st - ost) / 3.0;
        if j < J_MIN_ITERATION_COUNT {
            if (s - os).abs() < EPS * os.abs() || (s == 0.0 && os == 0.0) {
                return s;
            }
        }
        os = s;
        ost = st;
    }
    return s;
}

pub fn q_gauss_legendre(a: f32, b: f32, max_iteration: i32) -> f32 {
    let xr: f32;
    let xm: f32;
    let mut dx: f32;
    let mut s: f32;

    const X: [f32; 6] = [
        0.0,
        0.1488743389,
        0.4333953941,
        0.6794095682,
        0.8650633666,
        0.9739065285,
    ];
    const W: [f32; 6] = [
        0.0,
        0.2955242247,
        0.2692667193,
        0.2190863625,
        0.1494513491,
        0.0666713443,
    ];

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
