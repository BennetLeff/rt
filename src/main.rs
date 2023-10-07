use std::io::{self, Write};

use log::{info, Level, debug};

mod color;

use color::Color;

fn main() -> io::Result<()> {
    env_logger::init();

    // println!("Hello, world!");
    let img_width = 256;
    let img_height = 256;

    let ppm_header = format!("P3\n {} {} \n255\n", img_width, img_height);
    io::stdout().write_all(ppm_header.as_bytes())?;


    let mut ppm_data: String = "".to_owned();
    for j in 0..img_height {
        info!("Scanlines remaining: {}", img_height - j);

        for i in 0..img_width {
            let r = (i as f32) / (img_width as f32 - 1.0);
            let g = (j as f32) / (img_height as f32 - 1.0);
            let b = 0.0;
            let pixel_color = Color::new(r, g, b).to_str();
            ppm_data.push_str(&pixel_color);
        }
    }

    io::stdout().write_all(ppm_data.as_bytes())?;

    // info!(target: "Progress", "Finished rendering.");
    info!("Finished rendering.");

    Ok(())
}
