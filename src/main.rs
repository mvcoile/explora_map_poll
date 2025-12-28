use iced::widget::{button, column, container, row, rule, text};
use iced::{clipboard, window, Task};
use iced::{Alignment, Element, Length, Size};

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
    iced::application(App::new, update, view)
        .title("Explora Map Poll")
        // .theme(Theme::Nord)
        .window(window::Settings {
            exit_on_close_request: true,
            position: window::Position::Centered,
            size: Size::new(550.0, 250.0),
            min_size: Some(Size::new(220.0, 280.0)),
            max_size: Some(Size::new(800.0, 280.0)),
            ..Default::default()
        })
        .run()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::RefreshPressed => {
            app.refresh_seed();
            Task::none()
        }
        Message::CopyPressed => clipboard::write(app.poll.clone()),
    }
}

fn view(app: &App) -> Element<'_, Message> {
    let text = text(app.poll.to_string());
    let refresh_button = button("Refresh")
        .style(button::secondary)
        .on_press(Message::RefreshPressed);
    let copy_button = button("Copy")
        .style(button::primary)
        .on_press(Message::CopyPressed);

    container(
        column![
            text.height(120.0).width(Length::Fill),
            rule::horizontal(1),
            row![refresh_button, copy_button,]
                .spacing(18)
                .align_y(Alignment::Center),
        ]
        .align_x(Alignment::End)
        .spacing(10),
    )
    .padding(20)
    .height(Length::Fill)
    .width(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}
