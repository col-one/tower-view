


pub fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}

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
