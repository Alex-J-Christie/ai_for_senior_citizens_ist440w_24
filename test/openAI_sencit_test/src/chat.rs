use crate::db_man;
use crate::sttttts::generate_audio;
//db
use db_man::{add_prompt_user_info, get_prompt};
use dotenvy::dotenv;
use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};
use openai::{set_base_url, set_key};
use std::env;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io::{stdin, stdout, Write};

#[derive(Clone, Debug, PartialEq)]
pub enum Voices {
    Alloy,
    Echo,
    Fable,
    Onyx,
    Nova,
    Shimmer,
    None
}

impl Voices {
    pub const ALL: &'static [Self] = &[
        Self::Alloy,
        Self::Echo,
        Self::Fable,
        Self::Onyx,
        Self::Nova,
        Self::Shimmer,
        Self::None
    ];
}

impl Display for Voices {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Voices::Alloy => write!(f, "Alloy"),
            Voices::Echo => write!(f, "Echo"),
            Voices::Fable => write!(f, "Fable"),
            Voices::Onyx => write!(f, "Onyx"),
            Voices::Nova => write!(f, "Nova"),
            Voices::Shimmer => write!(f, "Shimmer"),
            Voices::None => write!(f, "None")
        }
    }
}

pub fn create_bot(user: &String, assistant: &String) -> Vec<ChatCompletionMessage> {
    dotenv().unwrap();
    set_key(env::var("OPENAI_KEY").unwrap());
    set_base_url(env::var("OPENAI_BASE_URL").unwrap_or_default());
    let messages: Vec<ChatCompletionMessage> = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some(get_prompt(user, assistant)),
        name: None,
        function_call: None,
    }];
    messages
}
pub async fn get_bot_response(messages: &mut Vec<ChatCompletionMessage>, user_message_content: String, user: &str, assistant: String) -> Vec<ChatCompletionMessage> {
    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: Some(user_message_content),
        name: Some(user.to_string()),
        function_call: None,//pipe function in to pass content results into database for persistence. Consider parsing manually to save space.
    });

    let chat_completion:  ChatCompletion = ChatCompletion::builder("gpt-4o-mini", messages.clone())
        .create()
        .await
        .unwrap();

    let returned_message: ChatCompletionMessage = chat_completion.choices.first().unwrap().message.clone();
    let mut admin_answer: String = returned_message
        .content
        .clone()
        .unwrap();
    let _user_answer: String = admin_answer
        .split_off(admin_answer.find("Reply to User: ")
        .unwrap());

    add_prompt_user_info(&user.to_owned(), assistant, &admin_answer[16..]);
    messages.push(returned_message);

    //for the record, I don't like this change because it makes gui and cli more complex than necessary but im struggling to solve this problem any other way so we're just returning the entire chat
    messages.clone()
}

pub async fn bot_voice(voice_line: String, voice: Voices) {
    generate_audio(voice_line, voice).await;
}

#[tokio::main]
pub async fn initiate_chat(user: &String) {
    let mut messages: Vec<ChatCompletionMessage> = create_bot(user, &String::from("Assistant"));
    loop {
        print!("{}: ", user);
        stdout().flush().unwrap();

        let mut user_message_content: String = String::new();

        stdin().read_line(&mut user_message_content).unwrap();
        let chat_results: Vec<ChatCompletionMessage> = get_bot_response(&mut messages, user_message_content, user, String::from("Assistant")).await;
        let chat_response: String = chat_results.last().unwrap().content.clone().unwrap()[50..].parse().unwrap();

        println!(
            "Assistant: {}",
            &chat_response[15..],
        );
        bot_voice(chat_response[15..].to_string(), Voices::Onyx).await;
    }
}