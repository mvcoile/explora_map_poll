use iced::theme::Theme;
use iced::widget::{button, column, container, row, text, horizontal_rule};
use iced::{window, Command, executor, clipboard};
use iced::{Alignment, Element, Length, Application, Settings};

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
        self.poll = format!(
            "/poll topic::world_map: Next map vote! :world_map:
			option1:Map 3500: https://rustmaps.com/map/3500_{} 
			option2:Map 3750: https://rustmaps.com/map/3750_{} 
			option3:Map 4000: https://rustmaps.com/map/4000_{} 
			option4:Map 4250: https://rustmaps.com/map/4250_{}",
            fastrand::u32(..2147483645),
            fastrand::u32(..2147483645),
            fastrand::u32(..2147483645),
            fastrand::u32(..2147483645)
        );
    }
}

fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        exit_on_close_request: true,
        default_text_size: 18,
        window: window::Settings {
            position: window::Position::Centered,
            size: (550, 250),
            min_size: Some((200, 250)),
            max_size: Some((800, 250)),
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
        (
            App::new(),
            Command::none()
        )
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
    } //clipboard::write(self.poll)

    fn view(&self) -> Element<Message> {
        let text = text(format!("{}", self.poll));
        let refresh_button = button("refresh").on_press(Message::RefreshPressed);
        let copy_button = button("copy").on_press(Message::CopyPressed);

        container(
            column![
                text.height(Length::Units(120)).width(Length::Fill),
                horizontal_rule(38),
                row![
                    refresh_button,
                    copy_button,
                ]
                .spacing(20)
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
