use slint::ComponentHandle;
use crate::PetWindow;

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HWND;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, GetWindowLongW, SetWindowLongW, GWL_EXSTYLE, WS_EX_TOOLWINDOW, WS_EX_APPWINDOW, ShowWindow, SW_HIDE, SW_SHOWNA};

pub fn setup_overlay(window: &PetWindow) {
    #[cfg(target_os = "windows")]
    {
        // Use the re-exported winit to avoid version conflicts
        use i_slint_backend_winit::WinitWindowAccessor;
        
        let _ = window.window().with_winit_window(|w| {
            // Here 'w' will have the correct type inferred from the trait
            use i_slint_backend_winit::winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
            
            if let Ok(handle) = w.window_handle() {
                if let RawWindowHandle::Win32(h) = handle.as_raw() {
                    let hwnd = HWND(h.hwnd.get() as _);
                    unsafe {
                        ShowWindow(hwnd, SW_HIDE);
                        let mut style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                        style |= WS_EX_TOOLWINDOW.0 as i32;
                        style &= !(WS_EX_APPWINDOW.0 as i32);
                        SetWindowLongW(hwnd, GWL_EXSTYLE, style);
                        let _ = SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
                        ShowWindow(hwnd, SW_SHOWNA);
                    }
                }
            }
        });
    }
}
