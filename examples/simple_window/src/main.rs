#![windows_subsystem = "windows"]

use std::convert::TryFrom;

use bindings::{
    Microsoft,
    Microsoft::UI::Xaml::{
        Application, ApplicationInitializationCallback, Controls::Button, HorizontalAlignment,
        LaunchActivatedEventArgs, RoutedEventHandler, Window,
    },
};

use windows::{implement, initialize_sta, IInspectable, Interface};

#[implement(extend Microsoft::UI::Xaml::Application, override OnLaunched)]
struct App {
    window: Option<Window>,
}

// TODO: Cleanup

// TODO: Resize / center the window before shown on screen to accommodate
// ultra-wide display users and generally be more respectful

#[allow(non_snake_case)]
impl App {
    fn OnLaunched(&mut self, _: &Option<LaunchActivatedEventArgs>) -> windows::Result<()> {
        let window = Window::new()?;
        window.SetTitle("WinUI Desktop (Rust)")?;

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

        let result = window.Activate();
        self.window = Some(window);
        result
    }
}

fn main() -> windows::Result<()> {
    initialize_sta()?;
    project_reunion_rs::initialize()
    .expect("Failed to initialize Project Reunion. Do you have the runtime installed? (https://aka.ms/projectreunion/0.8preview)");

    Application::Start(ApplicationInitializationCallback::new(|_| {
        App { window: None }.new()?;
        Ok(())
    }))
}
