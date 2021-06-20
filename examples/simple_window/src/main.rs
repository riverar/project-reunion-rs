#![windows_subsystem = "windows"]

use std::convert::TryFrom;

use project_reunion_rs::{workarounds::resize_window, PackageVersion};

use bindings::{
    Microsoft,
    Microsoft::UI::Xaml::{
        Application, ApplicationInitializationCallback, Controls::Button, HorizontalAlignment,
        LaunchActivatedEventArgs, RoutedEventHandler, Window,
    },
};

use windows::{implement, IInspectable, Interface, HRESULT};

#[implement(extend Microsoft::UI::Xaml::Application, override OnLaunched)]
struct App {
    window: Option<Window>,
}

#[allow(non_snake_case)]
impl App {
    fn OnLaunched(&mut self, _: &Option<LaunchActivatedEventArgs>) -> windows::Result<()> {
        self.window = Some(Window::new()?);
        if let Some(window) = &self.window {
            window.SetTitle("WinUI Desktop, Unpackaged (Rust)");

            let button = Button::new()?;
            button.SetContent(IInspectable::try_from("Click Me")?);
            button.SetHorizontalAlignment(HorizontalAlignment::Center);
            button.Click(RoutedEventHandler::new(|sender, _args| {
                if let Some(button) = sender {
                    button
                        .cast::<Button>()?
                        .SetContent(IInspectable::try_from("Clicked!")?);
                }
                Ok(())
            }));

            window.SetContent(&button);
            resize_window(window, 500, 500);
            window.Activate()
        } else {
            Err(HRESULT::from_thread().into())
        }
    }
}

fn main() -> windows::Result<()> {
    match project_reunion_rs::initialize(PackageVersion::new(0, 8, 0, 0)) {
        Ok(_) => Application::Start(ApplicationInitializationCallback::new(|_| {
            App { window: None }.new()?;
            Ok(())
        })),
        Err(err) => Err(err),
    }
}
