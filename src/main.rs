use image::{GenericImageView, Rgba};
use std::collections::HashMap;

fn find_colored_blocks(image_path: &str) -> HashMap<Rgba<u8>, Vec<(u32, u32)>> {
    let img = image::open(image_path).expect("Failed to open image");

    // Create a map to store the color and the corresponding coordinates
    let mut color_coordinates: HashMap<Rgba<u8>, Vec<(u32, u32)>> = HashMap::new();

    // Iterate through the pixels and store the coordinates for each unique color
    for (x, y, pixel) in img.pixels() {
        if pixel[3] > 0 { // If the alpha channel is not transparent
            color_coordinates.entry(pixel).or_default().push((x, y));
        }
    }

    color_coordinates
}

fn main() {
    let image_path = "100mmx100mm_leds_only.png";
    let color_coordinates = find_colored_blocks(image_path);

    for (color, coordinates) in &color_coordinates {
        println!("Color: {:?}, Coordinates: {:?}", color, coordinates);
    }
}
