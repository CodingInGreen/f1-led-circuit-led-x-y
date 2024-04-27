use image::{GenericImageView, Rgba};
use std::collections::HashSet;

// Convert a hex color string to an RGB tuple
fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).expect("Invalid hex format");
    let g = u8::from_str_radix(&hex[2..4], 16).expect("Invalid hex format");
    let b = u8::from_str_radix(&hex[4..6], 16).expect("Invalid hex format");
    (r, g, b)
}

// Find all pixels with the specified RGB color
fn find_pixels_with_hex_color(image_path: &str, hex_color: &str) -> HashSet<(u32, u32)> {
    let rgb_color = hex_to_rgb(hex_color);
    let img = image::open(image_path).expect("Failed to open image");

    let mut color_coordinates = HashSet::new();

    // Iterate through all pixels to find those with the specified RGB color
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            
            if (pixel[0], pixel[1], pixel[2]) == rgb_color {
                color_coordinates.insert((x, y));
            }
        }
    }

    color_coordinates
}

fn main() {
    let image_path = "100mmx100mm_top_left_pixel.png";
    let hex_color = "#00ff24";  // Hex value for the green color

    let color_coordinates = find_pixels_with_hex_color(image_path, hex_color);

    for coordinate in &color_coordinates {
        println!("Coordinate: {:?}", coordinate);
    }

    println!("Total number of pixels with specified hex color: {}", color_coordinates.len());
}
