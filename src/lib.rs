// We implement PackageVersion ourselves to avoid creating a WinRT dependency.

#[repr(C)]
#[derive(Clone, Copy)]
struct PackageVersion {
    major: u8,
    minor: u8,
    build: u8,
    revision: u8,
}

impl From<PackageVersion> for u32 {
    fn from(src: PackageVersion) -> u32 {
        ((src.major as u32) << 8) | src.minor as u32
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
    let package_version = PackageVersion {
        major: 0,
        minor: 8,
        revision: 0,
        build: 0,
    };
    let version_tag: Vec<u16> = "preview".encode_utf16().collect();
    unsafe {
        MddBootstrapInitialize(
            package_version.into(),
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
