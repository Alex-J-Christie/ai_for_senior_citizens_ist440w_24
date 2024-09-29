//db
use db_man::{get_prompt, add_prompt_user_info};
use std::env;
use std::fmt::Formatter;
use dotenvy::dotenv;
use std::process::Command;
use std::io::{stdin, stdout, Write};
use openai::{set_base_url, set_key};
use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};
use crate::db_man;
use std::fmt::Display;
#[derive(Clone, Debug, PartialEq)]
pub enum Voices {
    Sam,
    Us1,
    Us2,
    Us3
}

impl Voices {
    pub const ALL: &'static [Self] = &[
        Self::Sam,
        Self::Us1,
        Self::Us2,
        Self::Us3
    ];
}

impl Display for Voices {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Voices::Sam => write!(f, "Sam Variant"),
            Voices::Us1 => write!(f, "Us1 Variant"),
            Voices::Us2 => write!(f, "Us2 Variant"),
            Voices::Us3 => write!(f, "Us3 Variant"),
        }
    }
}


pub fn create_bot(user: &String) -> Vec<ChatCompletionMessage> {
    dotenv().unwrap();
    set_key(env::var("OPENAI_KEY").unwrap());
    set_base_url(env::var("OPENAI_BASE_URL").unwrap_or_default());
    let messages: Vec<ChatCompletionMessage> = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some(get_prompt(user)),
        name: None,
        function_call: None,
    }];
    messages
}

pub async fn get_bot_response(messages: &mut Vec<ChatCompletionMessage>, user_message_content: String, user: &str, voice: Voices) -> String {
    let mut voice_choice: String = String::from("");
    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: Some(user_message_content),
        name: None,
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
    let user_answer: String = admin_answer
        .split_off(admin_answer.find("Reply to User: ")
        .unwrap());

    add_prompt_user_info(user.to_owned(), &admin_answer[16..]);

    match voice {
        Voices::Us1 => voice_choice = String::from("mb-us1"),
        Voices::Us2 => voice_choice = String::from("mb-us2"),
        Voices::Us3 => voice_choice = String::from("mb-us3"),
        _ => {}
    }
    messages.push(returned_message);
    Command::new("espeak-ng")
        .arg(&user_answer[15..])
        .arg("-v")
        .arg(voice_choice)
        .spawn()
        .expect("espeak-ng command failed or is not present");

    user_answer
}

#[tokio::main]
pub async fn initiate_chat(user: &String) {
    let mut messages: Vec<ChatCompletionMessage> = create_bot(user);
    loop {
        print!("{}: ", user);
        stdout().flush().unwrap();

        let mut user_message_content: String = String::new();

        stdin().read_line(&mut user_message_content).unwrap();
        let chat_results: String = get_bot_response(&mut messages, user_message_content, user, Voices::Sam).await;

        println!(
            "Assistant: {}",
            &chat_results[15..],
        );

    }
}