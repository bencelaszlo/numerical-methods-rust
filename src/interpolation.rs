fn linear_interpolation(x1: f32, y1: f32, x0: f32, y0: f32, x: f32) -> f32 {
    let delta: f32;
    delta = x1 - x0;
    let y;

    if delta == 0.0 {
        y = y0;
    } else {
        y = y0 + ((x - x0) / delta) * y1;
    }

    return y;
}

pub fn linear_interpolation_vec(x_known: &[f32], y_known: &[f32], x: Vec<f32>, y: &mut Vec<f32>) {
    for i in 0..x.len() {
        let mut lower_bound_index: usize = 0;
        let mut higher_bound_index: usize = 0;

        for j in 0..x_known.len() {
            if x_known[j] < x_known[lower_bound_index] {
                lower_bound_index = j;
            }

            if x_known[j] > x_known[higher_bound_index] {
                higher_bound_index = j;
            }
        }

        for j in 0..x_known.len() {
            if x_known[j] <= x[i] && x_known[j] > x_known[lower_bound_index] {
                lower_bound_index = j;
            }

            if x_known[j] >= x[i] && x_known[j] < x_known[higher_bound_index] {
                higher_bound_index = j;
            }
        }

        y.push(linear_interpolation(
            x_known[higher_bound_index],
            y_known[higher_bound_index],
            x_known[lower_bound_index],
            y_known[lower_bound_index],
            x[i],
        ));
    }
}
