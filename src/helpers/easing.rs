pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let f = (t - 1.0) * 2.0;
        0.5 * f * f * f + 1.0
    }
}