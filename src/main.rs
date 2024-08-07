use std::collections::HashMap;
use std::ops::RangeInclusive;
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

struct Promt<'a> {
    morze: String,
    text: String,
    timer: SystemTime,
    dot_time: RangeInclusive<u128>,
    line_time: RangeInclusive<u128>,
    space_time: RangeInclusive<u128>,
    morze_table: HashMap<&'a str, char>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    KeyPressed,
    KeyReleased,
    Clear,
    Decode,
}

impl Application for Promt<'_> {
    type Message = Msg;
    type Executor = executor::Default;

    type Theme = theme::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                morze: String::new(),
                text: String::new(),
                timer: SystemTime::now(),
                dot_time: 100/2..=100*3/2,
                line_time: 300/2..=300*3/2,
                space_time: 300..=300*3,
                morze_table: HashMap::from([
                    ("*—", 'А'),
                    ("—***", 'Б'),
                    ("*——", 'В'),
                    ("——*", 'Г'),
                    ("—**", 'Д'),
                    ("*", 'Е'),
                    ("***—", 'Ж'),
                    ("——**", 'З'),
                    ("**", 'И'),
                    ("*——", 'Й'),
                    ("—*—", 'К'),
                    ("*—**", 'Л'),
                    ("——", 'М'),
                    ("—*", 'Н'),
                    ("———", 'О'),
                    ("*——*", 'П'),
                    ("*—*", 'Р'),
                    ("***", 'С'),
                    ("—", 'Т'),
                    ("**—", 'У'),
                    ("**—*", 'Ф'),
                    ("****", 'Х'),
                    ("—*—*", 'Ц'),
                    ("———*", 'Ч'),
                    ("————", 'Ш'),
                    ("——*—", 'Щ'),
                    ("*——*—*", 'Ъ'),
                    ("—*——", 'Ы'),
                    ("—**—", 'Ь'),
                    ("**—**", 'Э'),
                    ("**——", 'Ю'),
                    ("*—*—", 'Я')
                ]),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Morze")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let time = self.timer.elapsed().unwrap().as_millis();
        match message {
            Msg::KeyPressed => {
                if self.space_time.contains(&time){
                    self.morze.push(' ')
                }
            }
            Msg::KeyReleased => {
                if self.dot_time.contains(&time) {
                    self.morze.push('*');
                } else if self.line_time.contains(&time) {
                    self.morze.push('—');
                }
            }
            Msg::Clear => {
                self.morze.clear();
                self.text.clear();
            }
            Msg::Decode => {
                self.text = self.decode();
            }
        };
        self.timer = SystemTime::now();
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column!(
            row!(text_input("", &self.morze)),
            row!(text_input("", &self.text)),
            row!(button("clear").on_press(Msg::Clear), button("decode").on_press(Msg::Decode))
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

impl Promt<'_>{
    fn decode(&self) -> String{
        let mut ret = String::new();
        for i in self.morze.split(' '){
            ret.push(match self.morze_table.get(i) {
                Some(&ch) => ch,
                None => ' '
            })
        }
        ret
    }
}
