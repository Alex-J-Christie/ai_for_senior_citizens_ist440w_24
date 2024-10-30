use crate::chat;
use crate::chat::bot_voice;
use crate::gui_view::Fonts::{Monospace, Serif};
use crate::sttttts::{get_audio_input, transcribe};
//chat.rs methods
use chat::Voices;
use chat::{create_bot, get_bot_response};
use iced::alignment::Vertical;
use iced::alignment::Vertical::Center;
use iced::font::Family;
use iced::theme::palette::Pair;
use iced::time::Duration as IcedDuration;
use iced::widget::container::Style;
use iced::widget::scrollable::{Rail, Scroller};
use iced::widget::{button, container, horizontal_space, pick_list, scrollable, text, text_input, vertical_space, Column, Container, Image, Row, Scrollable, Slider, Text, TextInput};
use iced::Background::Color as BackgroundColor;
use iced::{border, Color, Element, Fill, FillPortion, Font, Pixels, Renderer, Size, Task, Theme};
use openai::chat::ChatCompletionMessage;
use serde::{Deserialize, Serialize};
//standards and openai
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::io;

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
    settings: Settings,
    side_bar: bool,
    recording_time: u64,
    recording: (bool, IcedDuration),
}
struct Settings {
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
    UserLogOut,
    ThemeChanged(Theme),
    VoiceChanged(Voices),
    TextSizeChanged(Pixels),
    TextFontChanged(Fonts),
    BotResponse(String),
    BotVoice,
    RecordingTimeChanged(u64),
    StartMic,
    Tick,
    SideBarChanged,
    SettingsSaved,
}
impl Default for Chat {
    fn default() -> Self {
        Chat {
            user: None,
            user_text: "".to_string(),
            bot: Vec::new(),
            content: "".to_string(),
            logs: vec![],
            settings: if Path::new("settings.toml").exists() {
                read_settings(Path::new("settings.toml")).expect("Could not read")
            } else {
                Settings {
                    theme: Theme::GruvboxDark,
                    voice: Voices::Alloy,
                    text_size: Pixels(16.0),
                    text_font: Serif,
                    text_family: Family::SansSerif,
                }
            },
            side_bar: true,
            recording_time: 10,
            recording: (false, IcedDuration::from_secs(10)),
        }
    }
}
impl Chat {
    fn title(&self) -> String {
        String::from("Senior Citizen AI Chatbot Test")
    }
    fn view(&self) -> Element<'_, Message> {

        let side_bar: Container<'_, Message> = self.side_bar();

        let main_area: Container<'_, Message> = self.main_area();

        let center_pop_up: Container<'_, Message> = self.login_popup();

        let recording_pop_up: Container<'_, Message> = self.recording_popup();

        let area: Row<'_, Message> = Row::new()
                .push(side_bar)
                .push(main_area)
                .padding(20);

        match self.recording.0 {
            true => {
                recording_pop_up.into()
            }
            false => {
                match self.user {
                    None => {
                        center_pop_up.into()
                    }
                    Some(_) => {
                        area.into()
                    }
                }
            }
        }
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
                    self.content.clear();

                    return Task::perform(async move {
                        Self::fetch_bot_response(bot, content, user_text)
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
                self.logs.push(format!("Welcome to the Chatbot Experience, {}! Say Hi to Your AI Assistant\n (Notice: AI Voices Are Generated and Not Real Humans)", &self.user_text));
                Task::none()
            }
            Message::UserLogOut => {
                self.logs.clear();
                if self.logs.len() % 2 == 0 {
                    self.logs.push(String::from("Goodbye!"));
                }
                self.logs.push(format!("User: {} has logged out", self.user_text));
                self.user = None;
                self.content.clear();
                self.user_text.clear();
                Task::none()
            }
            Message::ThemeChanged(theme) => {
                self.settings.theme = theme;
                Task::none()
            }
            Message::BotResponse(response) => {
                self.recording = (false, IcedDuration::from_secs(10));
                let print_line: String = response.to_string()[15..].parse().unwrap();
                self.logs.push(format!("Assistant: {}\n", print_line));
                let voice: Voices = self.settings.voice.clone();
                self.content.clear();
                Task::perform(async move {
                    Self::play_bot_voice(print_line, voice)
                }, |()| {
                    Message::BotVoice
                })
            }
            Message::VoiceChanged(voice) => {
                self.settings.voice = voice;
                Task::none()
            }
            Message::TextSizeChanged(size) => {
                self.settings.text_size = size;
                Task::none()
            }
            Message::TextFontChanged(font) => {
                self.settings.text_font = font.clone();
                self.settings.text_family = font.convert_to_family();
                Task::none()
            }
            Message::SideBarChanged => {
                self.side_bar = !self.side_bar;
                Task::none()
            }
            Message::BotVoice => {
                Task::none()
            }
            Message::RecordingTimeChanged(time) => {
                self.recording_time = time;
                Task::none()
            }
            Message::StartMic => {
                //todo! get timer popup working - right now a visual response in the chat screen should be enough
                let user_warning: String = String::from("Starting Mic");
                let bot_warning: String = format!("Assistant: Now Recording - You Have {} seconds to speak", self.recording_time);
                self.logs.push(user_warning);
                self.logs.push(bot_warning);

                Task::perform(async move {
                    //I have never 'if it works it works'ed more than I have now
                    Self::add_warning()
                }, |()| {
                    Message::Tick
                })
            }
            Message::Tick => {
                get_audio_input(self.recording_time).expect("Could not find usable mic or sample rate issue");
                let response = transcribe(PathBuf::from("output.wav"));
                if Path::new("output.wav").exists() {
                    self.logs.push(format!("{}: {}\n", self.user.clone().unwrap(), response));
                    let user_text: String = self.user_text.clone();
                    let bot: Vec<ChatCompletionMessage> = self.bot.clone();
                    self.content.clear();
                    return Task::perform(async move {
                        Self::fetch_bot_response(bot, response, user_text)
                    }, |response| {
                        Message::BotResponse(response)
                    });
                }
                Task::none()
            }
            Message::SettingsSaved => {
                save_settings(&self.settings, Path::new("settings.toml")).expect("Cannot write to file");
                Task::none()
            }
        }
    }
    #[tokio::main]
    async fn fetch_bot_response(mut bot: Vec<ChatCompletionMessage>, content: String, user_text: String) -> String {
        get_bot_response(&mut bot, content.clone(), &user_text).await
    }
    #[tokio::main]
    async fn play_bot_voice(input_text: String, voice: Voices) {
        bot_voice(input_text, voice).await
    }
    #[tokio::main]
    async fn add_warning() {
        println!("Warning: {}", String::from("This is a load bearing print"));
    }
    fn theme(&self) -> Theme {
        self.settings.theme.clone()
    }
    fn side_bar(&self) -> Container<'_, Message> {
        let icon: Image = Image::new("icon.png");

        let button_theme = move |theme: &Theme, _status| {
            let palette = theme.extended_palette();
            let background: Pair = palette.background.base;

            button::Style {
                text_color: background.text,
                background: Some(BackgroundColor(palette.background.weak.color)),
                border: Default::default(),
                shadow: Default::default(),
            }
        };

        let bar_button = button(
            text(match self.side_bar {
                true => {"O"}
                false => {"X"}
                    }
                )
            )
            .on_press(Message::SideBarChanged)
            .padding(10)
            .style(button_theme.clone());

        let time_list: Column<'_, Message> = Column::new()
            .push(text(format!("Choose Recording Time: {}", self.recording_time)))
            .push(Slider::new(5..=30, self.recording_time as u16, |value| Message::RecordingTimeChanged(value as u64))
                .step(5u16)
            )
            .padding(20);

        let theme_list: Column<'_, Message> = Column::new()
            .push(text("Choose Theme"))
            .push(pick_list(Theme::ALL, Some(&self.settings.theme), Message::ThemeChanged)
                .width(Fill))
            .padding(20);

        let voice_list: Column<'_, Message> = Column::new()
            .push(text("Choose a Voice"))
            .push(pick_list(Voices::ALL, Some(&self.settings.voice), Message::VoiceChanged))
            .width(Fill)
            .padding(20);

        let text_size_slider: Column<'_, Message> = Column::new()
            .push(text(format!("Set Font Size: {}", self.settings.text_size.0)))
            .push(Slider::new(8..=32, self.settings.text_size.0 as u16, |value| Message::TextSizeChanged(Pixels(value as f32 ))))
            .width(Fill)
            .padding(20);

        let fonts_list: Column<'_, Message> = Column::new()
            .push(text("Set Font Type:"))
            .push(pick_list(Fonts::ALL, Some(&self.settings.text_font), Message::TextFontChanged))
            .width(Fill)
            .padding(20);

        let settings_save_button: Row<'_, Message> = Row::new()
            .push(horizontal_space())
            .push(
                button(text("Save Settings"))
                .on_press(Message::SettingsSaved)
                .padding(10)
                .style(button_theme.clone()),
            )
            .push(horizontal_space())
            .padding(10);

        let logout_button: Row<'_, Message> = Row::new()
            .push(horizontal_space())
            .push(
                button(text("log out"))
                .on_press(Message::UserLogOut)
                .padding(10)
                .style(button_theme.clone()),
            )
            .push(horizontal_space())
            .padding(10);

        let side_bar: Container<'_, Message> = match self.side_bar {
            true => {
                container(
                    Column::new()
                        .push(icon)
                        .push(
                            Column::new()
                                .push(vertical_space())
                                .push(bar_button)
                        )
                )
                    .center_x(50)
                    .height(Fill)
                    .width(FillPortion(5))
            }
            false => {
                container(
                    Column::new()
                        .push(icon)
                        .push(theme_list)
                        .push(voice_list)
                        .push(text_size_slider)
                        .push(time_list)
                        .push(fonts_list)
                        .push(settings_save_button)
                        .push(logout_button)
                        .push(text(self.recording.1.as_secs().to_string()))
                        .push(
                            Column::new()
                                .push(vertical_space())
                                .push(bar_button)
                        )
                )
                    .center_x(50)
                    .height(Fill)
                    .width(FillPortion(20))

            }
        };
        side_bar
    }
    fn main_area(&self) -> Container<'_, Message> {
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
            .push(border_background(-1, text("Welcome to Your AI Companion! Enter Your Name to Chat!")
                .size(Pixels::from(self.settings.text_size.0))
                .font(Font {
                    family: self.settings.text_family,
                    weight: Default::default(),
                    stretch: Default::default(),
                    style: Default::default(),
                })))
            .push(Column::from_iter(self.logs.iter().enumerate().map(|(pos, value)|
                border_background(pos as i32, text(value)
                    .size(Pixels::from(self.settings.text_size.0))
                    .font(Font {
                        family: self.settings.text_family,
                        weight: Default::default(),
                        stretch: Default::default(),
                        style: Default::default(),
                    })
                ).into()
            )));

        let out_field: Scrollable<'_, Message> = scrollable(scrollable_content)
            .width(Fill)
            .height(Fill)
            .style(move |_theme: &Theme, _status| {
                let rail: Rail = Rail {
                    background: None,
                    border: Default::default(),
                    scroller: Scroller {
                        color: Color::TRANSPARENT,
                        border: Default::default()
                    },
                };
                scrollable::Style {
                    container: Default::default(),
                    vertical_rail: Rail {
                        background: None,
                        border: Default::default(),
                        scroller: Scroller { color: Default::default(), border: Default::default() },
                    },
                    horizontal_rail: rail,
                    gap: None,
                }
            })
            .anchor_bottom();

        let in_out_field: Column<'_, Message> = match self.user {
            Some(_) => {
                Column::new()
                    .push(out_field)
                    .push(Row::new()
                        .push(in_field)
                        .push(button(text("Speak"))
                            .padding(20)
                            .style(move |theme: &Theme, _status| {
                                let palette = theme.extended_palette();
                                let background: Pair = palette.background.base;

                                button::Style {
                                    text_color: background.text,
                                    background: Some(BackgroundColor(palette.background.base.color)),
                                    border: Default::default(),
                                    shadow: Default::default(),
                                }
                            })
                            .on_press(Message::StartMic))
                    )
            }
            None => {
                Column::new()
                    .push(out_field)
                    .push(in_field)
            }
        };

        let main_area: Container<'_, Message> = container(in_out_field)
            .height(Fill)
            .width(FillPortion(80))
            .align_y(Vertical::Bottom)
            .style(container::rounded_box)
            .padding(20);
        main_area
    }
    fn login_popup(&self) -> Container<'_, Message> {
        let input_field_container = container(
            text_input("Enter your name here....", &self.user_text)
                .on_input(Message::UserChanged)
                .on_submit(Message::UserAdded)
                .padding(10)
                .width(Fill)
        )
            .width(FillPortion(2))
            .padding(10);

        let input_box: Row<'_, Message> = self.vertical_width_pad(input_field_container.into(), 1);

        let login_box: Container<'_, Message> = container(Column::new()
            .push(container(text("Please Login"))
                    .center_x(Fill)
                    .padding(10))
            .push(input_box))
            .align_y(Center)
            .center_y(FillPortion(1))
            .center_x(FillPortion(2))
            .style(move |theme: &Theme| {
                self.button_theme(theme)
            });

        let output_row: Row<'_, Message> = self.vertical_width_pad(login_box.into(), 1);

        container(self.horizontal_height_pad(output_row.into(), 3))
    }
    fn recording_popup(&self) -> Container<'_, Message> {

        let signpost_container: Container<'_, Message> = container(text(format!("Recording: {} Seconds Remaining", self.recording.1.as_secs())))
            .center_x(Fill)
            .padding(10);

        let blank_space: Row<'_, Message> = Row::new()
            .push(horizontal_space().width(FillPortion(1)))
            .push(horizontal_space().width(FillPortion(1))
            );

        let recording_container: Container<'_, Message> = container(Column::new()
            .push(signpost_container)
            .push(blank_space))
            .align_y(Center)
            .center_y(FillPortion(1))
            .center_x(FillPortion(2))
            .style(move |theme: &Theme| {
                self.button_theme(theme)
            });

        let recording_row: Row<'_, Message> = self.vertical_width_pad(recording_container.into(), 1);

        container(self.horizontal_height_pad(recording_row.into(), 3))
    }
    fn button_theme(&self, theme: &Theme) -> Style {
            let palette = theme.extended_palette();
            let background: Pair = palette.background.strong;
            Style {
                text_color: Some(background.text.into()),
                background: Some(background.color.into()),
                border: border::rounded(border::radius(20)),
                shadow: Default::default(),
            }
    }
    //the height variable is temporary - this should NOT be determined by a bool - **FIX LATER**
    fn vertical_width_pad<'a>(&self, container: Element<'a, Message>, fill: u16) -> Row<'a, Message> {
            Row::new()
                .push(horizontal_space().width(FillPortion(fill)))
                .push(container)
                .push(horizontal_space().width(FillPortion(fill)))
    }
    fn horizontal_height_pad<'a>(&self, container: Element<'a, Message>, fill: u16) -> Column<'a, Message> {
        Column::new()
            .push(vertical_space().height(FillPortion(fill)))
            .push(container)
            .push(vertical_space().height(FillPortion(fill)))
    }
}
//iced widget helpers
fn border_background(pos: i32, text_input: Text<Theme, Renderer>) -> Container<Message> {
    let user_assignment: bool = pos % 2 == 0;
    match pos {
        -1 => {
            container(text_input)
                .padding(10)
                .center_x(800)
                .width(Fill)
                .style(container::bordered_box)
                .into()
        }
        _ => {
            let bubble = container(text_input)
                .style(move |theme: &Theme| {
                    let palette = theme.extended_palette();
                    let (background, radius) =
                        match user_assignment {
                            true => (palette.background.base, border::radius(20).top_left(0)),
                            false => (palette.background.strong, border::radius(20).top_right(0))
                        };
                    Style {
                        background: Some(background.color.into()),
                        text_color: Some(background.text),
                        border: border::rounded(radius),
                        ..Style::default()
                    }
                }
                )
                .padding(10);
            match user_assignment {
                true => {container(bubble)
                    .padding(10)
                    .align_left(Fill)}
                false => {container(bubble)
                    .padding(10)
                    .align_right(Fill)}
            }

        }
    }
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

//these are terrible and i feel bad that i wrote them - but i need to sleep at some point
#[derive(Serialize, Deserialize)]
struct TempSettings {
    theme: String,
    voice: String,
    text_size: f32,
    text_font: String,
    text_family: String,
}
fn save_settings(data: &Settings, file_path: &Path) -> io::Result<()> {
    let out_data: TempSettings = TempSettings {
        theme: match data.theme {
            Theme::Light => {String::from("Light")}
            Theme::Dark => {String::from("Dark")}
            Theme::Dracula => {String::from("Dracula")}
            Theme::Nord => {String::from("Nord")}
            Theme::SolarizedLight => {String::from("SolarizedLight")}
            Theme::SolarizedDark => {String::from("SolarizedDark")}
            Theme::GruvboxLight => {String::from("GruvboxLight")}
            Theme::GruvboxDark => {String::from("GruvboxDark")}
            Theme::CatppuccinLatte => {String::from("CatppuccinLatte")}
            Theme::CatppuccinFrappe => {String::from("CatppuccinFrappe")}
            Theme::CatppuccinMacchiato => {String::from("CatppuccinMacchiato")}
            Theme::CatppuccinMocha => {String::from("CatppuccinMocha")}
            Theme::TokyoNight => {String::from("TokyoNight")}
            Theme::TokyoNightStorm => {String::from("TokyoNightStorm")}
            Theme::TokyoNightLight => {String::from("TokyoNightLight")}
            Theme::KanagawaWave => {String::from("KanagawaWave")}
            Theme::KanagawaDragon => {String::from("KanagawaDragon")}
            Theme::KanagawaLotus => {String::from("KanagawaLotus")}
            Theme::Moonfly => {String::from("Moonfly")}
            Theme::Nightfly => {String::from("Nightfly")}
            Theme::Oxocarbon => {String::from("Oxocarbon")}
            Theme::Ferra => {String::from("Ferra")}
            Theme::Custom(_) => {String::from("Custom")}
        },
        voice: match data.voice {
            Voices::Alloy => {String::from("Alloy")}
            Voices::Echo => {String::from("Echo")}
            Voices::Fable => {String::from("Fable")}
            Voices::Onyx => {String::from("Onyx")}
            Voices::Nova => {String::from("Nova")}
            Voices::Shimmer => {String::from("Shimmer")}
            Voices::None => {String::from("None")}
        },
        text_size: data.text_size.0,
        text_font: match data.text_font {
            Serif => {String::from("Serif")}
            Monospace => {String::from("Monospace")}
        },
        text_family: match data.text_family {
            Family::Serif => {String::from("Serif")}
            Family::SansSerif => {String::from("SansSerif")}
            Family::Cursive => {String::from("Cursive")}
            Family::Fantasy => {String::from("Fantasy")}
            Family::Monospace => {String::from("Monospace")}
            _ => {String::from("None")}
        },
    };
    let toml: String = toml::to_string(&out_data).unwrap();
    let mut file = File::create(file_path).unwrap();
    file.write_all(toml.as_bytes()).unwrap();
    Ok(())
}
fn read_settings(file_path: &Path) -> io::Result<Settings> {
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let out_settings: TempSettings = toml::from_str(&content).unwrap();
    let settings: Settings = Settings {
        theme: match out_settings.theme.as_str() {
            "Light" => {Theme::Light}
            "Dark" => {Theme::Dark}
            "Dracula" => {Theme::Dracula}
            "Nord" => {Theme::Nord}
            "SolarizedLight" => {Theme::SolarizedLight}
            "SolarizedDark" => {Theme::SolarizedDark}
            "GruvboxLight" => {Theme::GruvboxLight}
            "GruvboxDark" => {Theme::GruvboxDark}
            "CatppuccinLatte" => {Theme::CatppuccinLatte}
            "CatppuccinFrappe" => {Theme::CatppuccinFrappe}
            "CatppuccinMacchiato" => {Theme::CatppuccinMacchiato}
            "CatppuccinMocha" => {Theme::CatppuccinMocha}
            "TokyoNight" => {Theme::TokyoNight}
            "TokyoNightStorm" => {Theme::TokyoNightStorm}
            "TokyoNightLight" => {Theme::TokyoNightLight}
            "KanagawaWave" => {Theme::KanagawaWave}
            "KanagawaDragon" => {Theme::KanagawaDragon}
            "KanagawaLotus" => {Theme::KanagawaLotus}
            "Moonfly" => {Theme::Moonfly}
            "Nightfly" => {Theme::Nightfly}
            "Oxocarbon" => {Theme::Oxocarbon}
            "Ferra" => {Theme::Ferra}
            _ => {Theme::GruvboxDark}
        },
        voice: match out_settings.voice.as_str() {
            "Alloy" => {Voices::Alloy}
            "Echo" => {Voices::Echo}
            "Fable" => {Voices::Fable}
            "Onyx" => {Voices::Onyx}
            "Nova" => {Voices::Nova}
            "Shimmer" => {Voices::Shimmer}
            "None" => {Voices::None}
            &_ => {todo!()}
        },
        text_size: Pixels(out_settings.text_size),
        text_font: match out_settings.text_font.as_str() {
            "Serif" => {Serif}
            "Monospace" => {Monospace}
            &_ => {todo!()}
        },
        text_family: match out_settings.text_family.as_str() {
            "Serif" => {Family::Serif}
            "SansSerif" => {Family::SansSerif}
            "Cursive" => {Family::Cursive}
            "Fantasy" => {Family::Fantasy}
            "Monospace" => {Family::Monospace}
            _ => {Family::Serif}
        },
    };

    Ok(settings)
}