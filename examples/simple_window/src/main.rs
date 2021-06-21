#![windows_subsystem = "windows"]

use std::convert::TryFrom;

use project_reunion_rs::{workarounds, PackageVersion};

use bindings::{
    Microsoft,
    Microsoft::UI::Xaml::{
        Application, ApplicationInitializationCallback, Controls::Button, HorizontalAlignment,
        LaunchActivatedEventArgs, RoutedEventHandler, Window,
    },
};

use windows::{implement, IInspectable, Interface};

#[implement(extend Microsoft::UI::Xaml::Application, override OnLaunched)]
struct App {
    _window: Option<Window>,
}

#[allow(non_snake_case)]
impl App {
    fn OnLaunched(&mut self, _: &Option<LaunchActivatedEventArgs>) -> windows::Result<()> {
        let window = Window::new().unwrap();
        window.SetTitle("WinUI Desktop, Unpackaged (Rust)")?;

        let button = Button::new()?;
        button.SetContent(IInspectable::try_from("Click Me")?)?;
        button.SetHorizontalAlignment(HorizontalAlignment::Center)?;
        button.Click(RoutedEventHandler::new(|sender, _args| {
            if let Some(button) = sender {
                button
                    .cast::<Button>()?
                    .SetContent(IInspectable::try_from("Clicked!")?)?;
            }
            Ok(())
        }))?;

        window.SetContent(&button)?;
        workarounds::resize_window(&window, 500, 500);
        workarounds::center_window(&window);

        let result = window.Activate();
        self._window = Some(window);
        result
    }
}

fn main() -> windows::Result<()> {
    project_reunion_rs::initialize(PackageVersion::new(0, 8, 0, 0)).and_then(|_| {
        Application::Start(ApplicationInitializationCallback::new(|_| {
            App { _window: None }.new()?;
            Ok(())
        }))
    })
}
