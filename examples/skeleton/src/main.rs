#![windows_subsystem = "console"]

fn main() -> windows::Result<()> {
    project_reunion_rs::initialize()
    .expect("Failed to initialize Project Reunion. Do you have the runtime installed? (https://aka.ms/projectreunion/0.8preview)");
    Ok(())
}
