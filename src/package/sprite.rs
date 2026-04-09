use image::GenericImageView;
use slint::{SharedPixelBuffer, Rgba8Pixel};

pub fn load_sprite_sheet(bytes: &[u8], frames: usize, frame_width: u32, frame_height: u32) -> Result<Vec<SharedPixelBuffer<Rgba8Pixel>>, String> {
    let img = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
    let mut buffers = Vec::new();
    
    for i in 0..frames {
        let x = (i as u32) * frame_width;
        if x + frame_width > img.width() {
            return Err("Sprite sheet image too small for declared frame count".into());
        }
        let sub_img = img.view(x, 0, frame_width, frame_height);
        
        let mut pixels = Vec::with_capacity((frame_width * frame_height * 4) as usize);
        for y in 0..frame_height {
            for px in 0..frame_width {
                let p = sub_img.get_pixel(px, y);
                pixels.push(p[0]);
                pixels.push(p[1]);
                pixels.push(p[2]);
                pixels.push(p[3]);
            }
        }
        
        let rgba_pixels: &[u8] = unsafe {
            std::slice::from_raw_parts(
                pixels.as_ptr() as *const u8,
                (frame_width * frame_height * 4) as usize
            )
        };
        
        let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(rgba_pixels, frame_width, frame_height);
        buffers.push(buffer);
    }
    
    Ok(buffers)
}
