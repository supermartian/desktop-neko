use slint::{SharedPixelBuffer, Rgba8Pixel};

pub struct Animator {
    frames: Vec<SharedPixelBuffer<Rgba8Pixel>>,
    fps: u32,
    current_index: usize,
    elapsed_ms: u64,
    looped: bool,
}

impl Animator {
    pub fn new(frames: Vec<SharedPixelBuffer<Rgba8Pixel>>, fps: u32) -> Self {
        Self {
            frames,
            fps,
            current_index: 0,
            elapsed_ms: 0,
            looped: false,
        }
    }

    pub fn reset(&mut self, frames: Vec<SharedPixelBuffer<Rgba8Pixel>>, fps: u32) {
        self.frames = frames;
        self.fps = fps;
        self.current_index = 0;
        self.elapsed_ms = 0;
        self.looped = false;
    }

    pub fn tick(&mut self, dt_ms: u64) -> bool {
        if self.frames.is_empty() { return false; }
        let frame_duration = 1000 / self.fps.max(1) as u64;
        self.elapsed_ms += dt_ms;
        let mut changed = false;
        while self.elapsed_ms >= frame_duration {
            self.elapsed_ms -= frame_duration;
            self.current_index += 1;
            if self.current_index >= self.frames.len() {
                self.current_index = 0;
                self.looped = true;
            }
            changed = true;
        }
        changed
    }

    pub fn is_done(&self) -> bool {
        self.looped
    }

    pub fn current_frame(&self) -> Option<SharedPixelBuffer<Rgba8Pixel>> {
        if self.frames.is_empty() {
            None
        } else {
            Some(self.frames[self.current_index].clone())
        }
    }
}
