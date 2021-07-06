fn main() {
    let width = 256;
    let height = 256;

    let mut image = format!("P3\n{} {}\n255\n", width, height);

    for row in (0..height - 1).rev() {
        for col in 0..width {
            let red   = col as f64 / (width  as f64 - 1.0);
            let green = row as f64 / (height as f64 - 1.0);
            let blue  = 0.25;

            let convert = |amount| (amount * 255.999) as u8;
            let pixel = format!(
                "{} {} {}\n",
                convert(red),
                convert(green),
                convert(blue)
            );

            image.push_str(&pixel);
        }
    }

    std::fs::write("image.ppm", image).unwrap();
}
