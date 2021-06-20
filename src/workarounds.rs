use bindings::{
    Microsoft::UI::Xaml::*,
    Windows::Win32::{
        Foundation::{BOOL, HWND},
        UI::{
            HiDpi::GetDpiForWindow,
            WindowsAndMessaging::{SetWindowPos, SWP_NOMOVE},
        },
    },
};
use windows::{Interface, HRESULT};

#[repr(transparent)]
struct IWindowNative(windows::IUnknown);

#[allow(dead_code)]
impl IWindowNative {
    pub fn handle(&self) -> Option<HWND> {
        unsafe {
            let mut hwnd = HWND(0);
            match self.WindowHandle(&mut hwnd as *mut HWND) {
                HRESULT(0) => Some(hwnd),
                _ => None,
            }
        }
    }

    #[allow(non_snake_case)]
    unsafe fn WindowHandle(&self, hwnd: *mut HWND) -> windows::HRESULT {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), hwnd)
    }
}

#[repr(C)]
pub struct IWindowNative_vtbl(
    // IUnknown::QueryInterface
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::HRESULT,
    // IUnknown::AddRef
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    // IUnknown::Release
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    // HRESULT WindowHandle(HWND* hWnd);
    pub unsafe extern "system" fn(this: ::windows::RawPtr, hwnd: *const HWND) -> ::windows::HRESULT,
);

unsafe impl ::windows::Interface for IWindowNative {
    type Vtable = IWindowNative_vtbl;
    // {eecdbf0e-bae9-4cb6-a68e-9598e1cb57bb}
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        0xeecdbf0e,
        0xbae9,
        0x4cb6,
        [0xa6, 0x8e, 0x95, 0x98, 0xe1, 0xcb, 0x57, 0xbb],
    );
}

pub fn resize_window(window: &Window, width: u32, height: u32) -> BOOL {
    let native_window = window.cast::<IWindowNative>().unwrap();
    if let Some(handle) = native_window.handle() {
        let scale_factor = unsafe { GetDpiForWindow(handle) / 96 };
        let width = width * scale_factor;
        let height = height * scale_factor;
        unsafe {
            return SetWindowPos(
                handle,
                HWND(0),
                0, // x
                0, // y
                width as i32,
                height as i32,
                SWP_NOMOVE,
            );
        }
    }
    BOOL::from(false)
}
