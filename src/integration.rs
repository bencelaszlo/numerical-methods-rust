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
    const EPS: f32 = 0.000001; //accurancy
    const J_MIN_ITERATION_COUNT: i32 = 5;

    let mut s: f32 = 0.0;
    let mut olds: f32;

    olds = -0.00000000000000000000000000001; // Any number that is unlikely to be the average of the function at its endpoints will do here
    for j in 0..j_max {
        s = trapedozial_rule(a, b, j);
        if j > J_MIN_ITERATION_COUNT {
            //avoid spurious early convergance
            if (s - olds).abs() <  EPS * olds.abs() {
                if s == 0.0 && olds == 0.0 {
                    return s;
                }
            }
        }
        olds = s;
    }
    return s;
}

pub fn q_simpsons_rule(a: f32, b: f32, j_max: i32) -> f32 {
    const EPS: f32 = 0.000001; //accurancy
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
        if j > J_MIN_ITERATION_COUNT {
            if (s - os).abs() < EPS * os.abs() {
                return s;
            }
        }
        os = s;
        ost = st;
    }
    return s;
}
