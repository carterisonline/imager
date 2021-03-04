use std::{
    convert::TryInto,
    fs::File,
    io::{Read, Write},
    path::Path,
};
use yansi::Paint;

pub fn save_file(path: &str) {
    let mut imagefile = match File::with_options().read(true).open(path) {
        Ok(e) => e,
        Err(_) => {
            eprintln!(
                "{}",
                Paint::red(format!("Error: {} is not a valid file.", path))
            );
            std::process::exit(1);
        }
    };

    let mut v = vec![];
    imagefile.read_to_end(&mut v).unwrap();

    let l = v.len() as f32 / 3.0;
    let mut lx = 0u32;
    let mut ly = 0u32;
    for i in (2..(l.sqrt().floor() as usize)).rev() {
        if (l / i as f32) % 1.0 == 0.0 {
            lx = (l / i as f32) as u32;
            ly = i as u32;
            break;
        }
    }
    println!("{} = {}x{}", l, lx, ly);
    println!("{}", l.sqrt().floor());

    // let ls = ((&v).len() as f32 / 3.0).sqrt().round() as u32;

    image::save_buffer(
        &Path::new(format!("{}.bmp", path).as_str()),
        &v,
        lx,
        ly,
        image::ColorType::Rgb8,
    )
    .unwrap();
}

pub fn from_image(path: &str) {
    let imagefile = match image::open(path) {
        Ok(e) => e,
        Err(_) => {
            eprintln!(
                "{}",
                Paint::red(format!("Error: {} is not a valid file.", path))
            );
            std::process::exit(1);
        }
    };

    let y = imagefile.to_rgb8();
    let x = y.as_raw();

    let mut f = File::create(path.trim_end_matches(".bmp")).expect("Unable to create file");
    f.write_all(x).expect("Unable to write data");
}
