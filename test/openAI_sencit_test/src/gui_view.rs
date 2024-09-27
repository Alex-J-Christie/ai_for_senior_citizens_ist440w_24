use iced::alignment::Vertical;
use iced::widget::{container, scrollable, text, text_input, Column, Container, Image, Row, Scrollable, TextInput};
use iced::{Fill, FillPortion, Size, Theme};

pub fn main() -> iced::Result {
    iced::application(Chat::title, Chat::update, Chat::view)
        .theme(|_| Theme::GruvboxDark)
        .window_size(Size { width: 960.0, height: 960.0 })
        .run()
}

#[derive(Default)]
struct Chat {
    content: String,
    logs: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    TextChanged(String),
    TextAdded,
}

impl Chat {
    fn title(&self) -> String {
        String::from("Senior Citizen AI Chatbot Test")
    }

    fn view(&self) -> Row<'_, Message> {
        let in_field: TextInput<'_, Message> = text_input("Type something here...", &self.content)
            .on_input(Message::TextChanged)
            .on_submit(Message::TextAdded)
            .width(Fill);

        let icon: Image = Image::new("icon.png");

        let scrollable_content: Column<'_, Message> = Column::new()
            .padding(10)
            .spacing(5)
            .push(Column::from_iter(self.logs.iter().map(|value| text(value).into())));

        let out_field: Scrollable<'_, Message> = scrollable(scrollable_content)
            .width(Fill)
            .height(Fill);

        let in_out_field: Column<'_, Message> = Column::new()
            .push(out_field)
            .push(in_field)
            .into();

        let side_bar: Container<'_, Message> = container(icon)
            .center_x(50)
            .height(Fill)
            .width(FillPortion(10))
            .into();

        let main_area: Container<'_, Message> = container(in_out_field)
            .height(Fill)
            .width(FillPortion(90))
            .align_y(Vertical::Bottom)
            .into();

        let area: Row<'_, Message> = Row::new()
            .push(side_bar)
            .push(main_area)
            .padding(10)
            .into();

        area
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TextChanged(content) => {
                self.content = content;
            }
            Message::TextAdded => {
                if !self.content.is_empty() {
                    self.logs.push(self.content.clone());
                    self.content.clear();
                }
            }
        }
    }
}

