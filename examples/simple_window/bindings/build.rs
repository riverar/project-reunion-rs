fn main() {
    windows::build! {
       Microsoft::UI::Xaml::*, // TODO: Pare this down
       Microsoft::UI::Xaml::Controls::Button,
       Windows::Foundation::*
    };
}
