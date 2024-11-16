use std::env;
use rusqlite::{Connection, params, Rows, Statement};
use std::path::{PathBuf};
use directories::UserDirs;

// #[derive(Debug)]
// struct User {
//     username: String,
//     user_id: i32,
//     info: Vec<String>,
//     assistants: Vec<String>,
//     current_assistant: String,
// }
// impl User {
//     fn new(user_name: String) -> User {
//         add_user(&user_name);
//         add_assistant(user_name.clone(), String::from("Assistant"));
//         User {
//             username: user_name.clone(),
//             user_id: get_user_id(user_name.clone()),
//             info: get_prompt_info(&user_name, &"Assistant".to_string()),
//             assistants: get_assistants(user_name.clone()),
//             current_assistant: "Assistant".to_string(),
//         }
//     }
// }

fn instance_conn() -> Connection {
    let path_source: UserDirs = UserDirs::new().expect("Could not find Docs");
    let mut path = match env::consts::OS {
        "windows" => PathBuf::from(path_source.document_dir()
            .unwrap()
            .to_str()
            .unwrap().to_owned() + r"\openAI_sencit_test"),
        _ => PathBuf::from(path_source.document_dir()
            .unwrap()
            .to_str()
            .unwrap().to_owned() + "/openAI_sencit_test")
    };
    if !path.exists() {
        std::fs::create_dir(&path).expect("No write Access");
    }
    path.push("openAI_sencit_test.db");
    let connection : Connection = Connection::open(path).expect("Can't access, failed at let in instance_conn()");
    connection
}
fn check_base_tables() {
    let conn = instance_conn();
    conn.execute(
        "create table if not exists Users (\
                user_id integer primary key unique,\
                user_name text not null unique)", ()).expect("Users Table Error");
    conn.execute(
        "create table if not exists Assistants (\
                assistant_id integer primary key unique,\
                assistant_name text not null default 'Assistant',\
                user_id integer, \
                foreign key (user_id) REFERENCES Users (user_id)\
                UNIQUE(user_id, assistant_name))", ()).expect("Assistants Table Error");
    conn.execute(
        "create table if not exists Info (\
                info_id integer primary key unique,\
                info text not null,\
                assistant_id integer,\
                foreign key (assistant_id) REFERENCES Assistants (assistant_id)\
                UNIQUE(assistant_id, info))", ()).expect("Info Table Error");
}
fn add_user(user_name: &String) {
    check_base_tables();
    let conn = instance_conn();
    let user_id: i32;
    conn.execute("insert or ignore into Users (User_Name) values(?1)",params![user_name])
        .expect("**failed to add new user in add_user()**\n");
    user_id = get_user_id(user_name);
    conn.execute("insert or ignore into Assistants (assistant_name, user_id) values('Assistant', ?1)",params![user_id]).expect("**failed to add new assistant in add_user()**\n");
}
fn check_for_user(user_name: &String) -> bool {
    check_base_tables();
    let conn: Connection = instance_conn();
    let mut names: Vec<String> = Vec::new();
    let mut stmt: Statement = conn.prepare("select user_name from Users where user_name = ?1").unwrap();
    let mut rows: Rows = stmt.query(params![user_name]).unwrap();

    while let Some(row) = rows.next().unwrap() {
        names.push(row.get(0).unwrap());
    }

    names.contains(&user_name)
}
fn get_user_id(user_name: &String) -> i32 {
    check_base_tables();
    let conn: Connection = instance_conn();
    let mut user_id: i32 = 0;
    let mut stmt = conn.prepare("select user_id from Users where user_name = ?1").unwrap();
    let mut rows: Rows = stmt.query(params![user_name]).unwrap();
    while let Some(row) = rows.next().unwrap() {
        user_id = row.get(0).unwrap();
    }
    user_id
}
fn get_assistants(user_name: &String) -> Vec<String> {
    check_base_tables();
    let conn: Connection = instance_conn();
    let mut assistants: Vec<String> = Vec::new();
    let mut stmt = conn.prepare("select assistant_name from Assistants where user_id = ?1").unwrap();
    let mut rows: Rows = stmt.query(params![get_user_id(&user_name)]).unwrap();
    while let Some(row) = rows.next().unwrap() {
        assistants.push(row.get(0).unwrap());
    }
    assistants
}
fn get_assistant_id(user_name: &String, assistant_name: String) -> i32 {
    check_base_tables();
    let conn: Connection = instance_conn();
    let mut assistant_id: i32 = 0;
    let user_id: i32 = get_user_id(user_name);
    let mut stmt = conn.prepare("select assistant_id from Assistants where assistant_name = ?1 and user_id = ?2").unwrap();
    let mut rows: Rows = stmt.query(params![assistant_name, user_id]).unwrap();
    while let Some(row) = rows.next().unwrap() {
        assistant_id = row.get(0).unwrap();
    }
    assistant_id
}
pub fn add_prompt_user_info(user_name: &String, assistant_name: String, new_info: &str) {
    check_base_tables();
    let conn: Connection = instance_conn();
    let assistant_id: i32 = get_assistant_id(user_name, assistant_name);
    match conn.execute("insert into Info (info, assistant_id) values (?1, ?2)",params![new_info, assistant_id]) {
        Ok(_) => print!(""),
        Err(err) => println!("Add Prompt User Info Error: {:?}", err),
    }
}
pub fn get_prompt_info(user_name: &String, assistant_name: &String) -> Vec<String> {
    check_base_tables();
    let conn: Connection = instance_conn();
    let mut user_info: Vec<String> = Vec::new();
    let assistant_id: i32 = get_assistant_id(user_name.into(), assistant_name.to_string());

    let mut stmt: Statement = conn.prepare("select info from Info where assistant_id = ?1").unwrap();
    let mut rows: Rows = stmt.query(params![assistant_id]).unwrap();

    while let Some(row) = rows.next().unwrap() {
        user_info.push(row.get(0).unwrap());
    }
    user_info
}
pub fn get_prompt(user_name: &String, assistant_name: &String) -> String {
    check_base_tables();
    add_user(&user_name);
    let info: String = get_prompt_info(user_name, assistant_name)
        .join(", ");
    format!("You are a chatbot designed to help older people deal with the isolation that comes from loneliness, difficulties handling the troubles of aging and various neurological disorders like dementia by being a warm, helpful companion. In order to maintain persistence between sessions you will give two answers to each response. The first answer will tell the backend admin what information you believe to be relevant for ongoing sessions. If there is no such information, or the information is already within the original prompt, do NOT provide any text. Do not add a newline at the end of replies. Only include information that would be relevant across sessions, like favorite activities or family members names. If there is no relevant information - use the text \"Reply to Admin: No relevant information provided.\" The second answer will be a response to a specific user following the overall guidelines. Be kind, gentle, and helpful to them. Today you are helping: {{User: {}, Info: {}}}. Format your responses like below, Reply to Admin: {{first response goes here}}, Reply to User: {{Second response goes here}}", user_name, info)
}
fn add_assistant(user_name: &String, assistant_name: &String) {
    check_base_tables();
    let conn: Connection = instance_conn();
    match conn.execute("insert into Assistants (assistant_name, user_id) values (?1, ?2)", params![assistant_name, get_user_id(&user_name)]) {
        Ok(updated) => println!("Assistants updated: {}", updated),
        Err(err) => println!("Add Assistant Error: {}", err),
    }

}
pub fn test(){
    let user: String = String::from("dorothy");
    add_user(&user);
    println!("test 1: basic user");
    println!("user exists: {}, user_id: {}", check_for_user(&user), get_user_id(&user));

    println!("test 2: add assistant");
    println!("Assistants: {:?}", get_assistants(&user));
    add_assistant(&user, &String::from("Luna"));
    println!("Assistants: {:?}", get_assistants(&user));

    println!("test 3: add user info");
    println!("Derek Info: {:?}", get_prompt_info(&String::from("Derek"), &String::from("Luna")));
    add_prompt_user_info(&user, String::from("Luna"), "Likes Chocolate");
    add_prompt_user_info(&user, String::from("Luna"), "Likes Chocolate");
    add_prompt_user_info(&user, String::from("Luna"), "Has 2 grandkids");
    add_prompt_user_info(&user, String::from("Luna"), "Can't drive");
    println!("Derek Info: {:?}", get_prompt_info(&user, &String::from("Luna")));

    println!("test 4: get prompt from info");
    println!("prompt: {}", get_prompt(&user, &String::from("Assistants")));
}