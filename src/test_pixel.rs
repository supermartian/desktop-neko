use windows::Win32::UI::WindowsAndMessaging::{GetDC, ReleaseDC};
use windows::Win32::Graphics::Gdi::GetPixel;
use windows::Win32::Foundation::HWND;

pub fn test() {
    unsafe {
        let hdc = GetDC(HWND(0));
        let color = GetPixel(hdc, 0, 0);
        ReleaseDC(HWND(0), hdc);
    }
}
