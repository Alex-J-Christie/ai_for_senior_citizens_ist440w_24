//db
use db_man::{get_prompt, add_prompt_user_info};
//chat
use std::env;
use dotenvy::dotenv;
use std::process::Command;
use std::io::{stdin, stdout, Write};
use openai::{set_base_url, set_key};
use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};
use crate::db_man;

#[tokio::main]
pub async fn initiate_chat(user: &String) {
    dotenv().unwrap();
    set_key(env::var("OPENAI_KEY").unwrap());
    set_base_url(env::var("OPENAI_BASE_URL").unwrap_or_default());

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some(get_prompt(&user)),
        name: None,
        function_call: None,
    }];

    loop {
        print!("{}: ", user);
        stdout().flush().unwrap();

        let mut user_message_content: String = String::new();

        stdin().read_line(&mut user_message_content).unwrap();
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

        let mut admin_answer: String = returned_message.content.clone().unwrap();
        let user_answer:      String = admin_answer.split_off(admin_answer.find("Reply to User: ").unwrap());
        // println!("{:?}", &admin_answer[16..]);
        // println!("{:?}", &user_answer[15..]);

        add_prompt_user_info(user.clone(), &admin_answer[16..]);

        println!(
            "{:#?}: {}",
            &returned_message.role,
            &user_answer[15..],
        );

        Command::new("espeak-ng")
            .arg(&user_answer[15..])
            .spawn()
            .expect("espeak-ng command failed or is not present");

        messages.push(returned_message);
    }
}