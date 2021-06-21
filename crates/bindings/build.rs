fn main() {
    windows::build! {
        Windows::Win32::Foundation::HWND,
        Windows::Win32::UI::HiDpi::GetDpiForWindow,
        Windows::Win32::UI::WindowsAndMessaging::*,

        Microsoft::Graphics::DirectX::*,
        Microsoft::UI::*,
        Microsoft::UI::Xaml::*,
        Microsoft::UI::Xaml::Controls::*,
        Microsoft::Web::WebView2::Core::*,
        Microsoft::ApplicationModel::Resources::*,
        Microsoft::Foundation::*,
    };
}
