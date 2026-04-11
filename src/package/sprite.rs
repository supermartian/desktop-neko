// Copyright (c) 2026 Desktop Neko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
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
