use image::{imageops::{blur, crop_imm, FilterType}, open, DynamicImage, GenericImageView, ImageBuffer, ImageFormat, Rgba};
use rayon::prelude::*;
use std::fs;
use std::path::Path;

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

fn rotate_image_90_clockwise(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
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
    const PATH: &str = "assets/";
    // let img = open(format!("{}sample_img.jpg", PATH)).expect("Failure to load/open the image");

    // img.save_with_format(format!("{}sample_img.png", PATH), ImageFormat::Png).expect("Failed to save image as png");

    // img.save_with_format(format!("{}sample_img.webp", PATH), ImageFormat::WebP).expect("Failed to save image as webp");

    println!("Processing image");

    // let resized_image = resize_image(format!("{}sample_img.jpg", PATH), 512, 512);
    // save_image(&resized_image, format!("{}sample_img_resized.jpg", PATH));

    // let rotate_image = rotate_image(format!("{}sample_img.jpg", PATH), 90);

    // save_image(&rotate_image, format!("{}sample_img_rotated.jpg", PATH));

    // let resized_image = resize_image_maintain_ratio(format!("{}sample_img.jpg", PATH), Some(800), None);

    // save_image(&resized_image, format!("{}sample_img_resized.jpg", PATH));

    // let image = open(format!("{}sample_img.jpg", PATH)).expect("Failed to open image");
    // let cropped_img = crop_image(&image, 50, 50, 500, 500);

    // save_image(&cropped_img, format!("{}sample_img_cropped.jpg", PATH));

    // let image = open(format!("{}sample_img.png", PATH)).expect("Failed to open image");
    // let rotate_img = rotate_image_90_clockwise(&image.to_rgba8());
    // rotate_img.save(format!("{}processed_img.png", PATH)).expect("Failed to save image");

    // let image_dir = Path::new("assets/parallel");
    // let image_paths = fs::read_dir(image_dir).expect("Failed to read directory").into_iter().filter_map(Result::ok).filter(|entry| entry.file_type().is_ok() && entry.file_type().unwrap().is_file()).map(|entry| entry.path()).collect::<Vec<_>>();
    
    // image_paths.par_iter().for_each(|img_path| {
    //     let img = open(img_path.to_str().unwrap()).expect("Failed to load image");
    //     let rotated_img = rotate_image_90_clockwise(&img.to_rgba8());
    //     let processed_path = format!("assets/parallel/processed_{}", img_path.file_name().unwrap().to_str().unwrap());
    //     rotated_img.save(processed_path).expect("Failed to save processed file");
    // })

    let image = open(format!("{}blur.jpg", PATH)).expect("Failed to open image");
    let blurred_image = blur_image(&image, 5.0);
    save_image(&blurred_image, format!("{}/blur_processed.jpg", PATH));
}
