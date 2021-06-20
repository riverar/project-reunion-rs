#![windows_subsystem = "console"]

use project_reunion_rs::PackageVersion;

fn main() -> windows::Result<()> {
    project_reunion_rs::initialize(PackageVersion::new(0, 8, 0, 0))
    .expect("Failed to initialize Project Reunion. Do you have the runtime installed? (https://aka.ms/projectreunion/0.8preview)");
    Ok(())
}
