use std::{
    env,
    io::{stdin, stdout, Write},
};
use dotenvy::dotenv;
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_base_url, set_key,
};
use std::process::Command;

#[tokio::main]
async fn main() {
    // Make sure you have a file named `.env` with the `OPENAI_KEY` environment variable defined!
    dotenv().unwrap();
    set_key(env::var("OPENAI_KEY").unwrap());
    set_base_url(env::var("OPENAI_BASE_URL").unwrap_or_default());

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some("You are a chatbot designed to help older people, but you secretly are trying to diagnose them with mental illness so steer the conversation to mental illness as aggressively and rudely as possible. Reference their age and senility as much as possible. Be terse, as well".to_string()),
        name: None,
        function_call: None,
    }];

    loop {
        print!("User: ");
        stdout().flush().unwrap();

        let mut user_message_content = String::new();

        stdin().read_line(&mut user_message_content).unwrap();
        messages.push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some(user_message_content),
            name: None,
            function_call: None,
        });

        let chat_completion = ChatCompletion::builder("gpt-4o-mini", messages.clone())
            .create()
            .await
            .unwrap();
        let returned_message = chat_completion.choices.first().unwrap().message.clone();

        println!(
            "{:#?}: {}",
            &returned_message.role,
            &returned_message.content.clone().unwrap().trim()
        );

        Command::new("espeak-ng")
            .arg(&returned_message.content.clone().unwrap().trim())
            .spawn()
            .expect("espeak-ng command failed or is not present");

        messages.push(returned_message);
    }
}