use rusqlite::{Connection, params, Rows, Statement};
use std::path::{PathBuf};
use directories::UserDirs;

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
    instance_conn().execute(
        "create table if not exists Users (\
        User_ID integer primary key unique,\
        User_Name text not null unique,\
        User_Prompt text default 'You are a chatbot designed to help older people deal with the isolation that comes from loneliness, difficulties handling the troubles of aging and various neurological disorders like dementia by being a warm, helpful companion. In order to maintain persistence between sessions you will give two answers to each response. The first answer will tell the backend admin what information you believe to be relevant for ongoing sessions. If there is no such information, or the information is already within the original prompt, do NOT provide any text. Do not add a newline at the end of replies. Only include information that would be relevant across sessions, like favorite activities or family members names. If there is no relevant information - use the text \"Reply to Admin: No relevant information provided.\" The second answer will be a response to a specific user following the overall guidelines. Be kind, gentle, and helpful to them. Today you are helping: {User: a new user, Info: }. Format your responses like below, Reply to Admin: {first response goes here}, Reply to User: {Second response goes here}')", ()).
        expect("What the hell did you type in wrong");
    // instance_conn().execute(
    //     "create table if not exists Info (\
    //     User_ID integer primary key unique,\
    //     Info text not null,\
    // )", ()).expect("user info table issue");
    instance_conn().execute("insert or ignore into Users (User_Name) values('Test')", ())
        .expect("Failed to insert default user");
}

fn add_user(user_name: &String) {
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
pub fn get_prompt(user_name: &String) -> String {
    check_base_tables();
    add_user(user_name);
    let conn: Connection = instance_conn();
    let mut user_info: Vec<String> = Vec::new();
    let mut stmt: Statement = conn.prepare("select User_Prompt from Users where User_Name = ?1").unwrap();
    let mut rows: Rows = stmt.query(params![user_name]).unwrap();

    while let Some(row) = rows.next().unwrap() {
        user_info.push(row.get(0).unwrap());
    }
    user_info.first().unwrap().to_string()
}
pub fn add_prompt_user_info(user_name: String, new_info: &str) {
    check_base_tables();
    let mut prompt_info: String = get_prompt(&user_name);
    let replacement_name: String = format!("{{User: {}", &user_name);
    let replacement_info: String = format!("Info: {},", &new_info);
    prompt_info = prompt_info.replace("{User: a new user,", &replacement_name);
    prompt_info = prompt_info.replacen("Info: ", &replacement_info, 1);

    instance_conn().execute(
        "update Users set User_Prompt = ?1 where User_Name = ?2",
        params![prompt_info, user_name]).expect("issue at updating prompt");
}

pub fn test(){
    println!("Test 1: Adding 'Adams' user");
    add_user(&String::from("Adams"));

    println!("\nTest 2: Checking that 'Adams' exists");
    print!("Check for 'Adams': {}", check_for_user(String::from("Adams")));

    println!("\nTest 3: prompt info output results");
    println!("get prompts results: {:?}", get_prompt(&String::from("Adams")));

    println!("\nTest 4: added prompt info test");
    add_prompt_user_info(String::from("Adams"), &String::from("Info: Her grandkids are named James and Charles"));
    println!("get prompts results: {:?}", get_prompt(&String::from("Adams")));

}