fn main() {
    let width: i32 = 256;
    let height: i32 = 256;

    println!("P3\n{} {}\n255", width, height);

    for line in (0..height).rev() {
        for column in 0..width {
            let r: f32 = (column as f32) / ((width - 1) as f32);
            let g: f32 = (line as f32) / ((height - 1) as f32);
            let b: f32 = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
