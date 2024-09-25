use std::env;
use std::io::{stdin, stdout, Write};
use rusqlite::{Connection, params, Rows, Statement};
use std::path::{PathBuf};
use std::process::Command;
use directories::UserDirs;
use dotenvy::dotenv;
use openai::{set_base_url, set_key};
use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};

fn instance_conn() -> Connection {
    let path_source: UserDirs = UserDirs::new().expect("Could not find Docs");
    let mut path = PathBuf::from(path_source.document_dir()
        .unwrap()
        .to_str()
        .unwrap().to_owned() + "/openAI_sencit_test");
    if !path.exists() {
        std::fs::create_dir(&path).expect("No write Access");
    }
    path.push("openAI_sencit_test.db");
    let connection : Connection = Connection::open(path).expect("Can't access, failed at let in instance_conn()");
    connection
}

fn check_base_tables() {
    instance_conn().execute("create table if not exists Users (\
        User_ID integer primary key unique,\
        User_Name text not null unique,\
        User_Prompt text default 'You are a chatbot designed to help older people deal with the isolation that comes from loneliness, difficulties handling the troubles of aging and various neurological disorders like dementia by being a warm, helpful companion. In order to maintain persistence between sessions you will give two answer to each response. The first answer will tell the backend admin what information you believe to be relevant for ongoing sessions. If there is no such information, or the information is already within the original prompt, do NOT include or provide any text. The second answer will be a response to a specific user following the overall guidelines. Be kind, gentle, and helpful to them. Today you are helping: {User: a new user, Info: no info yet}. Format your responses like below, Important Info: {first response goes here}, Reply to User: {Second response goes here}')", ()).
        expect("What the hell did you type in wrong");
    instance_conn().execute("insert or ignore into Users (User_Name) values('Test')", ())
        .expect("Failed to insert default user");
}

pub fn add_user(user_name: String) {
    check_base_tables();

    instance_conn().execute("insert or ignore into Users (User_Name) values(?1)",
        params![user_name])
        .expect("**failed to add new user in add_user()**\n");
}
fn check_for_user(user_name: String) -> bool {
    check_base_tables();
    let conn: Connection = instance_conn();
    let mut names: Vec<String> = Vec::new();
    let mut stmt: Statement = conn.prepare("select User_Name from Users where User_Name = ?1").unwrap();
    let mut rows: Rows = stmt.query(params![user_name]).unwrap();

    while let Some(row) = rows.next().unwrap() {
        names.push(row.get(0).unwrap());
    }
    names.contains(&user_name)
}
fn get_prompts(user_name: &String) -> Vec<String> {
    check_base_tables();
    let conn: Connection = instance_conn();
    let mut user_info: Vec<String> = Vec::new();
    let mut stmt: Statement = conn.prepare("select User_Prompt from Users where User_Name = ?1").unwrap();
    let mut rows: Rows = stmt.query(params![user_name]).unwrap();

    while let Some(row) = rows.next().unwrap() {
        user_info.push(row.get(0).unwrap());
    }
    user_info
}
fn add_prompt_user_info(user_name: &String, new_info: &String) {
    check_base_tables();
    let prompt_info: &String = get_prompts(user_name)
                                .get(0)
                                .unwrap();
    let endpoints = ["{User:","Info:", "}"];
    let prompt_points =[prompt_info.clone().find(endpoints[0]).unwrap(),
                                        prompt_info.find(endpoints[1]).unwrap(),
                                        prompt_info.find(endpoints[2]).unwrap()];

    let prompt_addition: String = format!("{} {}", &prompt_info[prompt_points[0]..prompt_points[1]], &new_info[16..]);

    prompt_info.replace_range(prompt_points[0]..prompt_points[1],
                               &*prompt_addition);

    prompt_info.replace_range(prompt_points[1]..prompt_points[2],
                                             user_name);
    instance_conn().execute(&prompt_info, ()).expect("add_prompt_user_info");
}

pub fn test(){
    println!("Test 1: Adding 'Adams' user");
    add_user(String::from("Adams"));

    println!("\nTest 2: Checking that 'Adams' exists");
    print!("Check for 'Adams': {}", check_for_user(String::from("Adams")));

    println!("\nTest 3: prompt info output results");
    println!("get prompts results: {:?}", get_prompts(&String::from("Adams")));

    println!("\nTest 4: added prompt info test");
    add_prompt_user_info(&String::from("Adams"), &String::from("Important Info: Her grandkids are named James and Charles"));
    println!("get prompts results: {:?}", get_prompts(&String::from("Adams")));

}

#[tokio::main]
pub async fn initiate_chat(user: String) {
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
            function_call: None,//pipe function in to pass content results into database for persistence. Consider parsing manually to save space.
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