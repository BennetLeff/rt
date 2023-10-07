use std::io::{self, Write};

use log::{info, Level};


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

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            let ppm_line = format!("{} {} {}\n", ir, ig, ib);

            ppm_data.push_str(&ppm_line);
        }
    }

    io::stdout().write_all(ppm_data.as_bytes())?;

    // info!(target: "Progress", "Finished rendering.");
    info!("Finished rendering.");

    Ok(())
}
