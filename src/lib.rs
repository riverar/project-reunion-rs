use bindings::Windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK},
};

mod tests;
pub mod workarounds;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PackageVersion {
    pub revision: u8,
    pub build: u8,
    pub minor: u8,
    pub major: u8,
}

impl PackageVersion {
    pub fn new(major: u8, minor: u8, build: u8, revision: u8) -> Self { Self { revision, build, minor, major } }

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

/// Locates a Project Reunion framework package compatible with the specified versioning criteria
/// and loads it into the current process. If multiple packages meet the criteria the best
/// candidate is selected.
pub fn initialize(minimum_version: PackageVersion) -> windows::Result<()> {
    match initialize_without_dialog(minimum_version) {
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

pub fn initialize_without_dialog(minimum_version: PackageVersion) -> windows::Result<()> {
    let version_tag: Vec<u16> = "preview".encode_utf16().collect();
    unsafe {
        // We use the provided version info in both the requested and minimum
        // version parameters to effectively request an exact match. This is
        // to work around what appears to be a bug in Mdd
        //
        // https://github.com/microsoft/ProjectReunion/issues/949

        MddBootstrapInitialize(
            minimum_version.to_major_minor(),
            version_tag.as_ptr(),
            minimum_version,
        )
        .ok()
    }
}

/// Undo the changes made by `initialize()`
pub fn uninitialize() -> windows::Result<()> {
    unsafe { MddBootstrapShutdown().ok() }
}
