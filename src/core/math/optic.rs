pub fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let sq_r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = sq_r0 * sq_r0;

    (1.0 - r0).mul_add(r0, (1.0 - cosine).powi(5))
}
