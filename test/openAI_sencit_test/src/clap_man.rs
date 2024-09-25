    use clap::{Arg, arg, ArgMatches, command};
    use crate::chat::{initiate_chat, test};

    pub fn run_args(args: ArgMatches) {
        if let Some(_a) = args.get_one::<String>("sign_in") {
            initiate_chat(String::from("test"));
        }
        if let Some(_b) = args.get_one::<bool>("test") {
            test();
        }
    }

    pub fn return_arg_array() -> ArgMatches {
        let sign_in: Arg = arg!(-s --sign_in <Username> "Use your username to talk to your tailored chatbot");
        let test: Arg = arg!(-t --test "Test for database functionality");
        command!().args([
            sign_in,
            test
        ]).get_matches()
    }