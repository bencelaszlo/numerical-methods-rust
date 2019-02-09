fn linear_interpolation(x1: f32, y1: f32, x0: f32, y0: f32, x: f32) -> f32 {
    let delta: f32;
    delta = x1 - x0;

    let y;

    if delta == 0.0 {
        y = y0;
    } else {
        y = y0 + (x - x0 / delta) * y1;
    }

    return y;
}

pub fn linear_interpolation_array(X: &[f32], Y: &[f32], x: &[f32], y: &mut [f32] ) {
    for i in 0..x.len() {
        let lower_bound_index: usize;
        let higher_bound_index: usize;
        
        let mut j: usize = 0;
        while X[j] < x[i] {
            j += 1;
        }
        lower_bound_index = j - 1;

        j = X.len() - 1;
        while X[j] > x[i] {
            j -= 1;
        }
        higher_bound_index = j+1;

        y[i] = linear_interpolation(X[higher_bound_index], Y[higher_bound_index], X[lower_bound_index], Y[lower_bound_index], x[i]);
    }
}