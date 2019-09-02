mod vec3;

use std::env;

const DEFAULT_WIDTH: u32 = 200;
const DEFAULT_HEIGHT: u32 = 100;
const DEFAULT_SAMPLES: u32 = 100;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut width: u32 = DEFAULT_WIDTH;
    let mut height: u32 = DEFAULT_HEIGHT;
    let mut samples: u32 = DEFAULT_SAMPLES;
    for idx in 0..args.len() {
        match args[idx].as_str() {
            "-r" => {
                if idx + 1 >= args.len() {
                    continue;
                }
                width = match args[idx + 1].parse::<u32>() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("Argument {} must be an unsigned integer!", idx);
                        DEFAULT_WIDTH
                    }
                };
                if idx + 2 >= args.len() {
                    continue;
                }
                height = match args[idx + 2].parse::<u32>() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("Argument {} must be an unsigned integer!", idx);
                        DEFAULT_HEIGHT
                    }
                };
            }
            "-s" => {
                if idx + 1 >= args.len() {
                    continue;
                }
                samples = match args[idx + 1].parse::<u32>() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("Argument {} must be an unsigned integer!", idx);
                        DEFAULT_SAMPLES
                    }
                };
            }
            &_ => {}
        };
    }

    let width: u32 = width;
    let height: u32 = height;
    let _samples: u32 = samples;

    println!("P3 {} {} 255", width, height);

    for y in (0..height).rev() {
        for x in 0..width {
            let r: u32 = (255.99f32 * (x as f32 / width as f32)) as u32;
            let g: u32 = (255.99f32 * (y as f32 / height as f32)) as u32;
            println!("{} {} 0", r, g);
        }
    }
}
