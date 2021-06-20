use bindings::Windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK},
};

mod tests;
pub mod workarounds;

#[repr(C)]
#[derive(Clone, Copy)]
struct PackageVersion {
    revision: u8,
    build: u8,
    minor: u8,
    major: u8,
}

impl PackageVersion {
    fn to_major_minor(&self) -> u32 {
        ((self.major as u32) << 8) | self.minor as u32
    }
}

#[link(name = "Microsoft.ProjectReunion.Bootstrap")]
extern "system" {
    fn MddBootstrapInitialize(
        majorMinorVersion: u32,
        versionTag: *const u16,
        minVersion: PackageVersion,
    ) -> windows::HRESULT;

    fn MddBootstrapShutdown() -> windows::HRESULT;
}

// TODO: Let user pass in versioning criteria

/// Locates a Project Reunion framework package compatible with the specified versioning criteria
/// and loads it into the current process. If multiple packages meet the criteria the best
/// candidate is selected.
pub fn initialize() -> windows::Result<()> {
    match initialize_without_dialog() {
        Err(err) => {
            match unsafe {
                MessageBoxW(
                    HWND::default(),
                    "To run this application, the Project Reunion runtime must be installed.\n\nhttps://aka.ms/projectreunion/0.8preview",
                    "This application could not be started",
                    MB_OK | MB_ICONERROR,
                )
            } {
                _ => Err(err),
            }
        }
        _ => Ok(()),
    }
}

pub fn initialize_without_dialog() -> windows::Result<()> {
    let package_version = PackageVersion {
        major: 0,
        minor: 8,
        revision: 0,
        build: 0,
    };
    let version_tag: Vec<u16> = "preview".encode_utf16().collect();
    unsafe {
        MddBootstrapInitialize(
            package_version.to_major_minor(),
            version_tag.as_ptr(),
            package_version,
        )
        .ok()
    }
}

/// Undo the changes made by `initialize()`
pub fn uninitialize() -> windows::Result<()> {
    unsafe { MddBootstrapShutdown().ok() }
}
