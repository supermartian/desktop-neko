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
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::PetWindow;
use slint::ComponentHandle;
use crate::package::loader::{load_package_from_dir, LoadedPackage};
use crate::pet::instance::PetInstance;
use crate::window::overlay::setup_overlay;
use crate::window::tray::setup_tray;

#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN, SystemParametersInfoW, SPI_GETWORKAREA, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS};
#[cfg(target_os = "windows")]
use core::ffi::c_void;
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::RECT;

static PET_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct App {
    packages: Vec<Arc<LoadedPackage>>,
}

pub enum AppMessage {
    SpawnPet(usize),
    Quit,
}

impl App {
    pub fn new() -> Self {
        Self { packages: Vec::new() }
    }

    /// Scan the `packages/` directory for subfolders. Each subfolder that contains
    /// a valid `manifest.toml` and `behaviors.toml` is loaded as a package.
    pub fn load_packages(&mut self) {
        let mut packages_dir = std::path::PathBuf::from("packages");

        if !packages_dir.exists() {
            if let Ok(exe_path) = std::env::current_exe() {
                if let Some(exe_dir) = exe_path.parent() {
                    let adj = exe_dir.join("packages");
                    if adj.exists() {
                        packages_dir = adj;
                    } else if let Some(contents) = exe_dir.parent() {
                        let res = contents.join("Resources").join("packages");
                        if res.exists() {
                            packages_dir = res;
                        }
                    }
                }
            }
        }
        
        if !packages_dir.is_dir() {
            log_error("packages/ directory not found — no pets to load.");
            return;
        }

        let entries = match std::fs::read_dir(packages_dir) {
            Ok(e) => e,
            Err(err) => {
                log_error(&format!("Failed to read packages/ directory: {}", err));
                return;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue; // skip files (.petpkg, images, etc.)
            }
            match load_package_from_dir(&path) {
                Ok(pkg) => {
                    self.packages.push(Arc::new(pkg));
                }
                Err(e) => {
                    log_error(&format!("Skipping '{}': {}", path.display(), e));
                }
            }
        }

        if self.packages.is_empty() {
            log_error("No valid packages found in packages/ — nothing will spawn.");
        }
    }

    pub fn run(mut self) {
        self.load_packages();

        if self.packages.is_empty() {
            // Nothing to do — exit cleanly rather than panic.
            return;
        }

        let package_names: Vec<String> = self.packages.iter()
            .map(|pkg| pkg.manifest.package.name.clone())
            .collect();

        let (tx, rx) = std::sync::mpsc::channel();
        let _tray = setup_tray(tx.clone(), &package_names);

        let instances: Arc<Mutex<Vec<PetInstance>>> = Arc::new(Mutex::new(Vec::new()));
        let mut last_tick = Instant::now();
        let packages_clone = self.packages.clone();

        let timer = slint::Timer::default();
        let instances_timer = instances.clone();

        let tx_clone = tx.clone();
        // Auto-spawn the first package on launch
        Self::spawn_pet(0, &packages_clone, &instances_timer, tx_clone);

        timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(16), move || {
            let now = Instant::now();
            let dt = now.duration_since(last_tick).as_millis() as u64;
            last_tick = now;

            if let Ok(msg) = rx.try_recv() {
                match msg {
                    AppMessage::SpawnPet(idx) => Self::spawn_pet(idx, &packages_clone, &instances_timer, tx.clone()),
                    AppMessage::Quit => {
                        slint::quit_event_loop().unwrap();
                        return;
                    }
                }
            }

            let cursor_pos = (0.0, 0.0); // TODO: actual cursor

            #[allow(unused_assignments)]
            let mut w = 1920.0;
            #[allow(unused_assignments)]
            let mut h = 1080.0;

            #[cfg(target_os = "windows")]
            unsafe {
                let mut rect = RECT::default();
                if SystemParametersInfoW(SPI_GETWORKAREA, 0, Some(&mut rect as *mut _ as *mut c_void), SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0)).is_ok() {
                    w = rect.right as f32;
                    h = rect.bottom as f32;
                } else {
                    w = GetSystemMetrics(SM_CXSCREEN) as f32;
                    h = GetSystemMetrics(SM_CYSCREEN) as f32;
                }
            }

            let mut inst_lock = instances_timer.lock().unwrap();
            for inst in inst_lock.iter_mut() {
                inst.update(dt, cursor_pos, w, h);
            }
        });

        slint::run_event_loop().unwrap();
    }

    fn spawn_pet(pkg_idx: usize, packages: &[Arc<LoadedPackage>], instances: &Arc<Mutex<Vec<PetInstance>>>, tx: std::sync::mpsc::Sender<AppMessage>) {
        if let Some(pkg) = packages.get(pkg_idx) {
            let window = PetWindow::new().unwrap();
            let sw = pkg.manifest.sprite.width as f32;
            let sh = pkg.manifest.sprite.height as f32;
            window.set_sprite_width(sw);
            window.set_sprite_height(sh);
            window.window().set_size(slint::PhysicalSize::new(sw as u32, sh as u32));

            let pet_id = PET_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
            let weak_window = window.as_weak();

            #[allow(unused_assignments)]
            let mut w = 1920.0;
            #[allow(unused_assignments)]
            let mut h = 1080.0;
            #[cfg(target_os = "windows")]
            unsafe {
                let mut rect = RECT::default();
                if SystemParametersInfoW(SPI_GETWORKAREA, 0, Some(&mut rect as *mut _ as *mut c_void), SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0)).is_ok() {
                    w = rect.right as f32;
                    h = rect.bottom as f32;
                } else {
                    w = GetSystemMetrics(SM_CXSCREEN) as f32;
                    h = GetSystemMetrics(SM_CYSCREEN) as f32;
                }
            }

            let inst = PetInstance::new(pkg.clone(), weak_window.clone(), w, h, pet_id);

            // --- Callbacks ---
            let inst_clone_left = instances.clone();
            weak_window.upgrade().unwrap().on_left_clicked({
                move || {
                    let mut lock = inst_clone_left.lock().unwrap();
                    if let Some(i) = lock.iter_mut().find(|p| p.pet_id == pet_id) {
                        i.state_machine.on_click("left");
                    }
                }
            });

            let tx_spawn = tx.clone();
            weak_window.upgrade().unwrap().on_spawn_package(move |idx| {
                let _ = tx_spawn.send(AppMessage::SpawnPet(idx as usize));
            });

            let tx_quit = tx.clone();
            weak_window.upgrade().unwrap().on_quit_all(move || {
                let _ = tx_quit.send(AppMessage::Quit);
            });

            let inst_clone_rm = instances.clone();
            let window_for_rm = weak_window.clone();
            weak_window.upgrade().unwrap().on_remove_pet(move || {
                let mut lock = inst_clone_rm.lock().unwrap();
                lock.retain(|p| p.pet_id != pet_id);
                if let Some(w) = window_for_rm.upgrade() {
                    w.hide().unwrap();
                }
            });

            let inst_clone_drag = instances.clone();
            weak_window.upgrade().unwrap().on_drag_delta(move |dx, dy| {
                let mut lock = inst_clone_drag.lock().unwrap();
                if let Some(i) = lock.iter_mut().find(|p| p.pet_id == pet_id) {
                    i.movement.x += dx;
                    i.movement.y += dy;
                }
            });

            weak_window.upgrade().unwrap().show().unwrap();
            setup_overlay(&window);
            instances.lock().unwrap().push(inst);
        }
    }
}

fn log_error(msg: &str) {
    use std::io::Write;
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("logs.txt") {
        let _ = writeln!(file, "[ERROR] {}", msg);
    }
}
