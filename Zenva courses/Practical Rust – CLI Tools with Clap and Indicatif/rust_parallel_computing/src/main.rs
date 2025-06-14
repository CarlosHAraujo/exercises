use image::{imageops::{blur, crop_imm, FilterType}, open, DynamicImage, GenericImageView, ImageBuffer, ImageFormat, Rgb, Rgba};
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use clap::{Arg,Command};
use indicatif::{ProgressBar,ProgressStyle,MultiProgress};
use std::time::Duration;

fn resize_image(path: String, width: u32, height: u32) -> DynamicImage {
    let img = open(path).expect("Failed to open image");
    img.resize(width, height, FilterType::Lanczos3)    
}

fn save_image(img: &DynamicImage, output_path: String) {
    img.save_with_format(output_path, image::ImageFormat::Png).expect("Failed to save image")
}

fn rotate_image(path: String, degrees: u32) -> DynamicImage {
    let img = open(path).expect("Failed to load image");

    match degrees {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => {
            println!("Unsupported rotation angle. Please give 90, 180, 270");
            img
        }
    }
}

fn resize_image_maintain_ratio(path: String, new_width: Option<u32>, new_height: Option<u32>) -> DynamicImage {
    let img = open(path).expect("Failed to open image");
    let (width, height) = img.dimensions();
    
    let ratio = width as f32 / height as f32;
    let (resize_width, resize_height) = match (new_width, new_height) {
        (Some(w), None) => (w, (w as f32 / ratio).round() as u32),
        (None, Some(h)) => ((h as f32 * ratio).round() as u32, h),
        (Some(w), Some(h)) => (w, h),
        (None, None) => (width, height)
    };

    img.resize(resize_width, resize_height, FilterType::Lanczos3)
}

fn crop_image(img: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
    let cropped_img = crop_imm(img, x, y, width, height);
    DynamicImage::ImageRgba8(cropped_img.to_image())
}

fn rotate_image_90_clockwise(img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();

    let mut new_img = ImageBuffer::new(height, width);

    img.enumerate_pixels().for_each(|(x, y, pixel)| {
        let new_x = height - y - 1;
        let new_y = x;

        new_img.put_pixel(new_x, new_y, *pixel);
    });

    new_img
}

fn blur_image(img: &DynamicImage, strength: f32) -> DynamicImage {
    let blurred_image = blur(img, strength);
    DynamicImage::ImageRgba8(blurred_image)
}

fn main() {
    println!("Image Processing - Parallel processing with thread pool -rayon");

    let matches = Command::new("CLI Image Processor")
        .version("1.2")
        .author("Zenva Academy")
        .about("Processes images by rotating them 90 degrees clockwise")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("INPUT DIRECTORY")
                .help("Sets the input directory to use")
                .required(true)
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT DIRECTORY")
                .help("Sets the output directory to save processed images")
                .required(true)
        )
        .arg(
            Arg::new("formats")
                .short('f')
                .long("formats")
                .value_name("FORMATS")
                .help("Sets the file format to process (e.g., png, jpg)")
                .num_args(1..)
        )
        .get_matches();
        
    let input_dir = matches.get_one::<String>("input").unwrap();
    let output_dir = matches.get_one::<String>("output").unwrap();
    let formats: Option<Vec<&str>> = matches.get_many::<String>("formats").map(|vals| vals.map(|s| s.as_str()).collect());

    fs::create_dir_all(output_dir).expect("Failed to create output directory.");

    let image_dir = Path::new(input_dir);
    let image_paths = fs::read_dir(image_dir)
        .expect("Failed to read directory")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_ok() && entry.file_type().unwrap().is_file())
        .filter(|entry| {
            if let Some(ref formats) = formats {
                formats.iter().any(|&format| entry.path().extension().map_or(false, |ext| ext == format))
            } else {
                true
            }
        })
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    let multi_progress = MultiProgress::new();

    let main_progress = multi_progress.add(ProgressBar::new(image_paths.len() as u64));

    main_progress.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.magenta/blue}] {pos}/{len} files processed ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );

    let spinner = multi_progress.add(ProgressBar::new_spinner());

    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} Processing images... {wide_msg}")
            .unwrap()
    );

    spinner.enable_steady_tick(Duration::from_millis(100));

    image_paths.par_iter().for_each(|img_path| {
        let img = open(img_path.to_str().unwrap()).expect("Failed to load image");
        let rotated_img = rotate_image_90_clockwise(&img.to_rgb8());
        
        let output_file = Path::new(output_dir).join(img_path.file_name().unwrap());
        rotated_img.save(output_file).expect("Failed to save processed file.");

        main_progress.inc(1);
        spinner.set_message(format!("Processing {}", img_path.file_name().unwrap().to_str().unwrap()));
    });
    
    main_progress.finish_with_message("All files processed");
    spinner.finish_with_message("Processing complete");
}
