use crate::chat;
use chat::{create_bot, get_bot_response};
use iced::alignment::Vertical;
use iced::widget::{container, pick_list, scrollable, text, text_input, Column, Container, Image, Row, Scrollable, TextInput};
use iced::{Fill, FillPortion, Size, Task, Theme};
use openai::chat::ChatCompletionMessage;

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
}

#[derive(Debug, Clone)]
enum Message {
    TextChanged(String),
    TextAdded,
    UserChanged(String),
    UserAdded,
    ThemeChanged(Theme),
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
        
        let side_bar: Container<'_, Message> = container(
            Column::new().push(icon).push(theme_list)
            )
            .center_x(50)
            .height(Fill)
            .width(FillPortion(20))
            .into();

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
            .push(text("Welcome to Your AI Companion! Enter Your Name to Chat!"))
            .padding(20)
            .spacing(5)
            .push(Column::from_iter(self.logs.iter().map(|value| text(value).into())));

        let out_field: Scrollable<'_, Message> = scrollable(scrollable_content)
            .width(Fill)
            .height(Fill);

        let in_out_field: Column<'_, Message> = Column::new()
            .push(out_field)
            .push(in_field)
            .into();

        let main_area: Container<'_, Message> = container(in_out_field)
            .height(Fill)
            .width(FillPortion(80))
            .align_y(Vertical::Bottom)
            .style(container::rounded_box)
            .padding(20)
            .into();

        let area: Row<'_, Message> = Row::new()
            .push(side_bar)
            .push(main_area)
            .padding(20)
            .into();

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
                    let user_text = self.user_text.clone();
                    let content = self.content.clone();
                    let bot = self.bot.clone();

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
        }
    }
    #[tokio::main]
    async fn fetch_bot_response(mut bot: Vec<ChatCompletionMessage>, content: String, user_text: String) -> String {
        get_bot_response(&mut bot, content, &user_text)
            .await
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}