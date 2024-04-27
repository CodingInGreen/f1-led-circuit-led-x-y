use image::{GenericImageView, Rgba};
use std::collections::HashSet;

fn find_top_left_of_squares(image_path: &str) -> HashSet<(u32, u32)> {
    let img = image::open(image_path).expect("Failed to open image");

    let mut top_left_coordinates = HashSet::new();

    // Iterate through the pixels in increments of 2 to find the top-left pixel of each 2x2 block
    for y in (0..img.height() - 1).step_by(2) {
        for x in (0..img.width() - 1).step_by(2) {
            let pixel = img.get_pixel(x, y);

            // Ignore black pixels
            if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 {
                continue;
            }

            // If pixel is not black and has some alpha, consider it as a top-left corner of a square
            if pixel[3] > 0 {
                top_left_coordinates.insert((x, y));
            }
        }
    }

    top_left_coordinates
}

fn main() {
    let image_path = "100mmx100mm_leds_only.png";
    let top_left_coordinates = find_top_left_of_squares(image_path);

    for coordinate in &top_left_coordinates {
        println!("Coordinate: {:?}", coordinate);
    }

    println!("Total number of squares: {}", top_left_coordinates.len());
}
