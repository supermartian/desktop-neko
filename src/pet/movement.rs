use crate::package::behavior::MovementDef;

pub struct MovementState {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub screen_width: f32,
    pub screen_height: f32,
    pub is_falling: bool,
}

#[cfg(target_os = "windows")]
fn get_highest_surface_below(x: f32, width: f32, test_y: f32, max_y: f32) -> f32 {
    use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowRect, IsWindowVisible, GetWindowLongW, GWL_EXSTYLE, GWL_STYLE, WS_EX_TOOLWINDOW};
    use windows::Win32::Foundation::{HWND, RECT, LPARAM, BOOL};

    struct SurfaceState {
        pet_x: f32,
        pet_w: f32,
        test_y: f32,
        best_y: f32,
    }

    unsafe extern "system" fn enum_callbacks(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let state = &mut *(lparam.0 as *mut SurfaceState);
        if IsWindowVisible(hwnd).as_bool() {
            // Ignore windows like overlays or small tooltips
            let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
            if (ex_style & WS_EX_TOOLWINDOW.0) == 0 {
                let mut rect = RECT::default();
                if GetWindowRect(hwnd, &mut rect).is_ok() {
                    let wx1 = rect.left as f32 - 10.0;
                    let wx2 = rect.right as f32 + 10.0;
                    let wy = rect.top as f32;
                    
                    // Use center of pet for stability
                    let center_x = state.pet_x + state.pet_w / 2.0;

                    if center_x >= wx1 && center_x <= wx2 {
                        // Is the top of the window below our check altitude?
                        // Allow a small overlap tolerance up to 5 pixels
                        if wy >= (state.test_y - 5.0) && wy < state.best_y {
                            // Don't collide with extremely thin/zero-width bounds
                            if wy > 10.0 {
                                state.best_y = wy;
                            }
                        }
                    }
                }
            }
        }
        BOOL(1)
    }

    let mut state = SurfaceState {
        pet_x: x,
        pet_w: width,
        test_y: test_y,
        best_y: max_y,
    };

    unsafe {
        let _ = EnumWindows(Some(enum_callbacks), LPARAM(&mut state as *mut _ as isize));
    }
    state.best_y
}

#[cfg(not(target_os = "windows"))]
fn get_highest_surface_below(_x: f32, _width: f32, _test_y: f32, max_y: f32) -> f32 { max_y }

impl MovementState {
    pub fn new(w: f32, h: f32, sw: f32, sh: f32) -> Self {
        Self {
            x: (sw / 2.0) - (w / 2.0),
            y: (sh / 2.0) - (h / 2.0),
            width: w,
            height: h,
            screen_width: sw,
            screen_height: sh,
            is_falling: true, // starts falling!
        }
    }

    pub fn resize_screen(&mut self, screen_width: f32, screen_height: f32) {
        self.screen_width = screen_width;
        self.screen_height = screen_height;
    }

    pub fn tick(&mut self, movement: &Option<MovementDef>, dt_ms: u64, cursor_pos: (f32, f32)) {
        let dt = dt_ms as f32 / 1000.0;
        
        let ground_y = (self.screen_height - self.height).max(0.0);
        
        // Find the absolute highest standing surface under the pet
        let highest_surface_y = get_highest_surface_below(self.x, self.width, self.y + self.height, self.screen_height);
        
        let target_y = (highest_surface_y - self.height).clamp(0.0, ground_y);
        
        if self.y < target_y {
            self.y += 400.0 * dt; // gravity
            if self.y > target_y {
                self.y = target_y;
            }
        } else if self.y > target_y + 10.0 && !self.is_falling {
            // We got dragged above the target or the window moved away! Force fall.
            self.y += 400.0 * dt;
        } else {
            // Align tightly
            self.y = target_y;
        }
        
        // If we are significantly misaligned from the target platform, we are falling
        self.is_falling = self.y < target_y - 2.0;

        if let Some(m) = movement {
            let speed = m.speed_px_s * dt;
            if m.direction == "right" {
                self.x += speed;
            } else if m.direction == "left" {
                self.x -= speed;
            } else if m.direction == "cursor" {
                let dx = cursor_pos.0 - (self.x + self.width / 2.0);
                if dx > 0.0 {
                    self.x += speed.min(dx);
                } else {
                    self.x -= speed.min(-dx);
                }
            }
        }

        self.x = self.x.clamp(0.0, (self.screen_width - self.width).max(0.0));
        self.y = self.y.clamp(0.0, ground_y);
    }
    
    pub fn is_at_edge(&self, edge: &str) -> bool {
        match edge {
            "left" => self.x <= 0.0,
            "right" => self.x >= self.screen_width - self.width,
            _ => false,
        }
    }
}
