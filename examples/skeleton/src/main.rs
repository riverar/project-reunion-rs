#![windows_subsystem = "console"]

use project_reunion_rs::PackageVersion;

fn main() -> windows::Result<()> {
    project_reunion_rs::initialize(PackageVersion::new(0, 8, 0, 0))
}
