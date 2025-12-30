mod application;

use application::App;
use iced::window;
use iced::Size;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Rust Map Poll for Discord")
        // .theme(Theme::Nord)
        .window(window::Settings {
            exit_on_close_request: true,
            position: window::Position::Centered,
            size: Size::new(650.0, 250.0),
            min_size: Some(Size::new(380.0, 200.0)),
            max_size: Some(Size::new(850.0, 350.0)),
            ..Default::default()
        })
        .run()
}
