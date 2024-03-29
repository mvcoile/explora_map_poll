use iced::theme::Theme;
use iced::widget::{button, column, container, horizontal_rule, row, text};
use iced::{clipboard, executor, window, Command};
use iced::{Alignment, Application, Element, Length, Settings, Size};

#[derive(Debug, Clone)]
pub enum Message {
    RefreshPressed,
    CopyPressed,
}

#[derive(Default)]
pub struct App {
    poll: String,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self::default();
        app.refresh_seed();
        app
    }

    fn refresh_seed(&mut self) {
        let mut temp = String::from("/poll topic::world_map: Next map vote! :world_map:");
        for (i, size) in [3500, 3750, 4000, 4250].into_iter().enumerate() {
            temp.push_str(
                format!(
                    "\noption{index}:Map {size}: https://rustmaps.com/map/{size}_{seed}",
                    index = i + 1,
                    seed = fastrand::u32(..2147483645)
                )
                .as_str(),
            );
        }
        self.poll = temp;
    }
}

fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        default_text_size: 16.into(),
        window: window::Settings {
            exit_on_close_request: true,
            position: window::Position::Centered,
            size: Size::new(550.0, 250.0),
            min_size: Some(Size::new(220.0, 280.0)),
            max_size: Some(Size::new(800.0, 280.0)),
            ..Default::default()
        },
        ..Default::default()
    })
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn theme(&self) -> Theme {
        match dark_light::detect() {
            dark_light::Mode::Dark => Theme::Dark,
            dark_light::Mode::Light => Theme::Light,
            dark_light::Mode::Default => Theme::Light,
        }
    }

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        (App::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Explora Map Poll")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::RefreshPressed => self.refresh_seed(),
            Message::CopyPressed => return clipboard::write(String::from(&self.poll)),
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let text = text(self.poll.to_string());
        let refresh_button = button("Refresh").on_press(Message::RefreshPressed);
        let copy_button = button("Copy").on_press(Message::CopyPressed);

        container(
            column![
                text.height(120.0).width(Length::Fill),
                horizontal_rule(38),
                row![refresh_button, copy_button,]
                    .spacing(18)
                    .align_items(Alignment::End),
            ]
            .align_items(Alignment::End)
            .spacing(10),
        )
        .padding(20)
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
