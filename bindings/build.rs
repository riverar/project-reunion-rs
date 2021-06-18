fn main() {
    windows::build! {
       Microsoft::Graphics::DirectX::*,
       Microsoft::UI::*,
       Microsoft::Web::WebView2::Core::*,
       Microsoft::ApplicationModel::Resources::*,
       Microsoft::Foundation::*,
    };
}
