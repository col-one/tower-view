
pub fn premultiply_by_alpha(pixels: &Vec<u8>) -> Vec<u8> {
    let mut pixels_mult = Vec::new();
    for (mut i, pixel) in pixels.iter().enumerate() {
        i += 1;
        if i % 4 == 0 {
            i -= 1;
            let alpha = pixels[i] as f32 / 255.0f32;
            let red = pixels[i - 3] as f32 / 255.0f32;
            let green = pixels[i - 2] as f32 / 255.0f32;
            let blue = pixels[i - 1] as f32 / 255.0f32;
            pixels_mult.push(((red * alpha) * 255.0f32) as u8);
            pixels_mult.push(((green * alpha) * 255.0f32) as u8);
            pixels_mult.push(((blue * alpha) * 255.0f32) as u8);
            pixels_mult.push((alpha * 255.0f32) as u8);
        }
    }
    pixels_mult
}


pub fn add_alpha_channel(pixels: &Vec<u8>) -> Vec<u8> {
    let mut pixels_alpha = Vec::new();
    let offset_size = pixels.iter().count() / 4;
    for (mut i, pixel) in pixels.iter().enumerate() {
        if i % 3 == 0 && i != 0 {
            pixels_alpha.push(255);
        }
        pixels_alpha.push(*pixel);
    }
    pixels_alpha.push(255);
    pixels_alpha
}
