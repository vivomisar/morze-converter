use std::time::SystemTime;

use iced::keyboard;
use iced::keyboard::key::Named;
use iced::widget::{button, column, row, text_input};
use iced::{executor, theme, Application, Command, Element, Settings, Subscription};

fn main() -> iced::Result {
    Promt::run(Settings {
        window: iced::window::Settings {
            size: iced::Size {
                width: 400f32,
                height: 400f32,
            },
            position: iced::window::Position::Centered,
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    })
}

struct Promt {
    morze: String,
    timer: SystemTime,
    dot_time: u128,
}

#[derive(Debug, Clone)]
pub enum Msg {
    KeyPressed,
    KeyReleased,
    Clear,
}

impl Application for Promt {
    type Message = Msg;
    type Executor = executor::Default;

    type Theme = theme::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                morze: String::new(),
                timer: SystemTime::now(),
                dot_time: 100,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Morze")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Msg::KeyPressed => {
                self.timer = SystemTime::now();
            }
            Msg::KeyReleased => {
                let low_dot = self.dot_time / 2;
                let hi_dot = self.dot_time * 3 / 2;
                let hi_line = self.dot_time * 9 / 2;
                let time = self.timer.elapsed().unwrap().as_millis();
                if (low_dot..=hi_dot).contains(&time) {
                    self.morze.push('•');
                } else if (hi_dot..=hi_line).contains(&time) {
                    self.morze.push('—');
                }
            }
            Msg::Clear => {
                self.morze.clear();
            }
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column!(
            row!(text_input("", &self.morze)),
            row!(button("clear").on_press(Msg::Clear))
        )
        .padding(10)
        .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let on_press = keyboard::on_key_press(|keycode, _modifiers| match keycode {
            iced::keyboard::Key::Named(Named::Space) => Some(Msg::KeyPressed),
            _ => None,
        });
        let on_release = keyboard::on_key_release(|keycode, _modifiers| match keycode {
            iced::keyboard::Key::Named(Named::Space) => Some(Msg::KeyReleased),
            _ => None,
        });
        Subscription::batch(vec![on_press, on_release])
    }
}
