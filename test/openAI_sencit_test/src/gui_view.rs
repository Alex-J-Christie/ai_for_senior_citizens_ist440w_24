use crate::chat;
//chat.rs methods
use chat::Voices;
use chat::{create_bot, get_bot_response};
use iced::alignment::Vertical;
use iced::font::Family;
use iced::widget::{container, pick_list, scrollable, text, text_input, Column, Container, Image, Row, Scrollable, Slider, Text, TextInput};
use iced::{Fill, FillPortion, Font, Pixels, Renderer, Size, Task, Theme};

use crate::gui_view::Fonts::{Monospace, Serif};
use openai::chat::ChatCompletionMessage;
//standards and openai
use std::fmt::{Display, Formatter};
use iced::widget::container::Style;

pub fn main() -> iced::Result {
    iced::application(Chat::title, Chat::update, Chat::view)
        .theme(Chat::theme)
        .window_size(Size { width: 960.0, height: 960.0 })
        .run()
}

// #[derive(Default)]
struct Chat {
    user: Option<String>,
    user_text: String,
    bot: Vec<ChatCompletionMessage>,
    content: String,
    logs: Vec<String>,
    theme: Theme,
    voice: Voices,
    text_size: Pixels,
    text_font: Fonts,
    text_family: Family,
}

#[derive(Debug, Clone)]
enum Message {
    TextChanged(String),
    TextAdded,
    UserChanged(String),
    UserAdded,
    ThemeChanged(Theme),
    VoiceChanged(Voices),
    TextSizeChanged(Pixels),
    TextFontChanged(Fonts),
    BotResponse(String),
}

impl Default for Chat {
    fn default() -> Self {
        Chat {
            user: None,
            user_text: "".to_string(),
            bot: Vec::new(),
            content: "".to_string(),
            logs: vec![],
            theme: Theme::GruvboxDark,
            voice: Voices::Sam,
            text_size: Pixels(16.0),
            text_font: Serif,
            text_family: Family::SansSerif,
        }
    }
}

impl Chat {
    fn title(&self) -> String {
        String::from("Senior Citizen AI Chatbot Test")
    }
    fn view(&self) -> Row<'_, Message> {

        let icon: Image = Image::new("icon.png");

        let theme_list: Column<'_, Message> = Column::new()
            .push(text("Choose Theme"))
            .push(pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged)
                .width(Fill))
                .padding(20);

        let voice_list: Column<'_, Message> = Column::new()
            .push(text("Choose a Voice"))
            .push(pick_list(Voices::ALL, Some(&self.voice), Message::VoiceChanged))
                .width(Fill)
                .padding(20);

        let text_size_slider: Column<'_, Message> = Column::new()
            .push(text(format!("Set Font Size: {}", self.text_size.0)))
            .push(Slider::new(8..=32, self.text_size.0 as u16, |value| Message::TextSizeChanged(Pixels(value as f32 ))))
                .width(Fill)
                .padding(20);

        let fonts_list: Column<'_, Message> = Column::new()
            .push(text("Set Font Type:"))
            .push(pick_list(Fonts::ALL, Some(&self.text_font), Message::TextFontChanged))
                .width(Fill)
                .padding(20);

        let side_bar: Container<'_, Message> = container(
            Column::new().push(icon).push(theme_list).push(voice_list).push(text_size_slider).push(fonts_list))
                .center_x(50)
                .height(Fill)
                .width(FillPortion(20));

        let in_field: TextInput<'_, Message> = match &self.user {
            Some(_) => {
                text_input("Type something here...", &self.content)
                .on_input(Message::TextChanged)
                .on_submit(Message::TextAdded)
                .width(Fill)
                .padding(20)
            }
            None => {
                text_input("Enter your name here....", &self.user_text)
                .on_input(Message::UserChanged)
                .on_submit(Message::UserAdded)
                .width(Fill)
                .padding(20)
            }
        };

        let scrollable_content: Column<'_, Message> = Column::new()
            .push(border_background(self.theme.clone(), text("Welcome to Your AI Companion! Enter Your Name to Chat!")
                                        .size(Pixels::from(self.text_size.0))
                                        .font(Font {
                                            family: self.text_family,
                                            weight: Default::default(),
                                            stretch: Default::default(),
                                            style: Default::default(),
                                        })))
                                        //self.text_font.convert_to_font())) //self.text_font.convert_to_font())).padding(20)
            .push(Column::from_iter(self.logs.iter().map(|value|
                  text(value)
                 .size(Pixels::from(self.text_size.0))
                 .font(Font {
                     family: self.text_family,
                     weight: Default::default(),
                     stretch: Default::default(),
                     style: Default::default(),
                 })
                 .into()
            )));

        let out_field: Scrollable<'_, Message> = scrollable(scrollable_content)
                .width(Fill)
                .height(Fill);

        let in_out_field: Column<'_, Message> = Column::new()
                .push(out_field)
                .push(in_field);

        let main_area: Container<'_, Message> = container(in_out_field)
                .height(Fill)
                .width(FillPortion(80))
                .align_y(Vertical::Bottom)
                .style(container::rounded_box)
                .padding(20);

        let area: Row<'_, Message> = Row::new()
                .push(side_bar)
                .push(main_area)
                .padding(20);

        area
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TextChanged(content) => {
                self.content = content;
                Task::none()
            }
            Message::TextAdded => {
                if !self.content.is_empty() {
                    self.logs.push(format!("{}: {}\n", self.user.clone().unwrap(), self.content.clone()));
                    let user_text: String = self.user_text.clone();
                    let content: String = self.content.clone();
                    let bot: Vec<ChatCompletionMessage> = self.bot.clone();
                    let voice: Voices = self.voice.clone();

                    return Task::perform(async move {
                        Self::fetch_bot_response(bot, content, user_text, voice)
                    }, |response| {
                        Message::BotResponse(response)
                    });
                }
                self.content.clear();
                Task::none()
            }
            Message::UserChanged(content) => {
                self.user_text = content;
                Task::none()
            }
            Message::UserAdded => {
                self.user = Some(self.user_text.clone());
                self.bot = create_bot(&self.user_text);
                self.logs.push(format!("Welcome to the Chatbot Experience, {}! Say Hi to Your AI Assistant\n", &self.user_text));
                Task::none()
            }
            Message::ThemeChanged(theme) => {
                self.theme = theme;
                Task::none()
            }
            Message::BotResponse(response) => {
                let print_line: String = response.to_string()[15..].parse().unwrap();
                self.logs.push(format!("Assistant: {}\n", print_line));
                self.content.clear();
                Task::none()
            }
            Message::VoiceChanged(voice) => {
                self.voice = voice;
                Task::none()
            }
            Message::TextSizeChanged(size) => {
                self.text_size = size;
                Task::none()
            }
            Message::TextFontChanged(font) => {
                self.text_font = font.clone();
                self.text_family = font.convert_to_family();
                Task::none()
            }
        }
    }
    #[tokio::main]
    async fn fetch_bot_response(mut bot: Vec<ChatCompletionMessage>, content: String, user_text: String, voice: Voices) -> String {
        get_bot_response(&mut bot, content, &user_text, voice)
            .await
    }
    fn theme(&self) -> Theme {
        self.theme.clone()
    }



}

//iced widget helpers
fn border_background(theme: Theme, text_input: Text<Theme, Renderer>) -> Container<Message> {

    let bubble_style: Style = Style {
        text_color: Option::from(theme.palette().text),
        background: None,
        border: Default::default(),
        shadow: Default::default(),
    };

    container(text_input)
        .padding(10)
        .center_x(800)
        .style(container::bordered_box)
        .into()
}

//temp space for fonts
#[derive(Debug, Clone, PartialEq)]
enum Fonts {
    Serif,
    Monospace,
}
    impl Display for Fonts {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.family_to_string())
        }
    }
impl Fonts {
    pub const ALL: [Fonts; 2] = [
        Serif,
        Monospace,
        ];
    pub fn family_to_string(&self) -> String {
        match self {
            Serif => "Serif".to_string(),
            // SansSerif => "SansSerif".to_string(),
            // Cursive => "Cursive".to_string(),
            // Fantasy => "Fantasy".to_string(),
            Monospace => "Monospace".to_string(),
        }
    }
    // #[warn(dead_code)]
    // pub fn convert_to_font(&self) -> Font {
    //     match self {
    //         Serif => Font::with_name("Serif"),
    //         // SansSerif => Font::with_name("SansSerif"),
    //         // Cursive => Font::with_name("Cursive"),
    //         // Fantasy => Font::with_name("Fantasy"),
    //         Monospace => Font::with_name("Monospace"),
    //     }
    // }

    pub fn convert_to_family(&self) -> Family {

        match self {
            Serif => Family::Serif,
            // SansSerif => Family::SansSerif,
            // Cursive => Family::Cursive,
            // Fantasy => Family::Fantasy,
            Monospace => Family::Monospace,
        }
    }
}


// let load_data = match self {
//     Montserrat => read("fonts/Montserrat/static/Montserrat-Medium.ttf"),
//     NotoSans => read("fonts/Noto_Sans/static/NotoSans-Medium.ttf"),
//     NotoSerif => read("fonts/Noto_Serif/static/NotoSerif-Medium.ttf"),
//     OpenSans => read("fonts/Open_Sans/static/OpenSans-Medium.ttf"),
//     Roboto => read("fonts/Roboto/Roboto-Regular.ttf"),
//     CedarvilleCursive => read("fonts/Cedarville_Cursive/CedarvilleCursive-Regular.ttf"),