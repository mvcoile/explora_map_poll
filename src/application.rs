use std::time::Duration;

use iced::widget::{button, checkbox, column, container, row, scrollable, space, text, tooltip};
use iced::{clipboard, Task};
use iced::{Alignment, Element, Length};

const MAP_SIZES: [u32; 4] = [3500, 3750, 4000, 4250];
const MAX_SEED: u32 = 2_147_483_645;

#[derive(Debug, Clone)]
pub enum Message {
    RefreshPressed,
    CopyPressed,
    StagingToggled(bool),
}

#[derive(Default)]
pub struct App {
    poll: String,
    staging: bool,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self::default();
        app.refresh_seed();
        app.staging = false;
        app
    }

    pub fn refresh_seed(&mut self) {
        let mut temp = String::from("/poll topic::world_map: Next map vote! :world_map:");
        for (i, size) in MAP_SIZES.into_iter().enumerate() {
            temp.push_str(
                format!(
                    "\noption{index}:Map {size}: https://rustmaps.com/map/{size}_{seed}",
                    index = i + 1,
                    seed = fastrand::u32(..MAX_SEED)
                )
                .as_str(),
            );
            if self.staging {
                temp.push_str("/staging");
            }
        }
        self.poll = temp;
    }

    pub fn update(app: &mut App, message: Message) -> Task<Message> {
        match message {
            Message::RefreshPressed => {
                app.refresh_seed();
                Task::none()
            }
            Message::CopyPressed => clipboard::write(app.poll.clone()),
            Message::StagingToggled(value) => {
                app.staging = value;
                app.refresh_seed();
                Task::none()
            }
        }
    }

    pub fn view(app: &App) -> Element<'_, Message> {
        let text = text(app.poll.to_string());
        let refresh_button = button("Refresh")
            .style(button::secondary)
            .on_press(Message::RefreshPressed);
        let copy_button = button("Copy")
            .style(button::primary)
            .on_press(Message::CopyPressed);
        let staging_check = checkbox(app.staging)
            .label("Staging Maps")
            .on_toggle(Message::StagingToggled);

        // To turn on debug grid, wrap the container (before the .into()) with the following:
        // iced::Element::new(your_widget).explain(iced::Color::BLACK)
        container(
            column![
                container(scrollable(
                    text.size(16).height(Length::Fill).width(Length::Fill)
                ))
                .style(container::bordered_box)
                .padding(10)
                .height(Length::Fill)
                .width(Length::Fill),
                row![
                    tooltip(
                        staging_check,
                        container("Check this box to enable staging maps.")
                            .padding(10)
                            .style(container::rounded_box)
                            .style(container::warning),
                        tooltip::Position::Top,
                    )
                    .delay(Duration::from_secs(1)),
                    space().width(Length::Fill),
                    tooltip(
                        refresh_button,
                        container("Get new random seeds for the 4 maps.")
                            .padding(10)
                            .style(container::rounded_box)
                            .style(container::warning),
                        tooltip::Position::Top,
                    )
                    .delay(Duration::from_secs(1)),
                    tooltip(
                        copy_button,
                        container("Copy the /poll text onto your clipboard.")
                            .padding(10)
                            .style(container::rounded_box)
                            .style(container::warning),
                        tooltip::Position::Top,
                    )
                    .delay(Duration::from_secs(1)),
                ]
                .spacing(18)
                .align_y(Alignment::Center),
            ]
            .align_x(Alignment::End)
            .spacing(20),
        )
        .padding(20)
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }
}
