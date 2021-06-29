struct Image {
    width: usize,
    height: usize,
}

fn main() {
    let image = Image { width: 256, height: 256 };
    let mut pixels = format!("P3\n{} {}\n255\n", image.width, image.height);

    for row in (0..image.height - 1).rev() {
        for col in 0..image.width {
            let red   = col as f64 / (image.width  as f64 - 1.0);
            let green = row as f64 / (image.height as f64 - 1.0);
            let blue  = 0.25;

            let convert = |amount| (amount * 255.999) as u8;
            let pixel = format!(
                "{} {} {}\n",
                convert(red),
                convert(green),
                convert(blue)
            );

            pixels.push_str(&pixel);
        }
    }

    std::fs::write("image.ppm", pixels).unwrap();
}
