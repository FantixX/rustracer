use rustracer::vector3d;
use std::fs;
fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    let mut image_string = String::from(format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT} \n255\n"));

    for i in 0..IMAGE_HEIGHT {
        println!("{} % done", get_percentage(&i, &IMAGE_HEIGHT).to_string());
        for j in 0..IMAGE_WIDTH {
            let r_clamped: f32 = i as f32 / IMAGE_WIDTH as f32;
            let g = 0;
            let b_clamped: f32 = j as f32 / IMAGE_WIDTH as f32;

            let r: i32 = (255.0 * r_clamped).round() as i32;
            let b: i32 = (255.0 * b_clamped).round() as i32;

            image_string.push_str(&format!("{r} {g} {b} "));
        }
        image_string.push_str("\n");
    }

    fs::write("render.ppm", image_string).unwrap();

    let point = vector3d::Point3::from(255.0, 255.0, 255.0);
    println!("{}", point)
}

fn get_percentage(row: &i32, max: &i32) -> i32 {
    return ((*row as f32 / *max as f32) * 100.0).round() as i32;
}
