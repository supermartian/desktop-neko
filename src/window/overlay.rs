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
                        let _ = ShowWindow(hwnd, SW_HIDE);
                        let mut style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                        style |= WS_EX_TOOLWINDOW.0 as i32;
                        style &= !(WS_EX_APPWINDOW.0 as i32);
                        SetWindowLongW(hwnd, GWL_EXSTYLE, style);
                        let _ = SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
                        let _ = ShowWindow(hwnd, SW_SHOWNA);
                    }
                }
            }
        });
    }
}
