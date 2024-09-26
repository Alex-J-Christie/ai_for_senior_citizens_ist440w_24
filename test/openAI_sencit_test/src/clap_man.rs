    use clap::{Arg, arg, ArgMatches, command};
    use crate::chat::initiate_chat;
    use crate::db_man::test;

    pub fn run_args(args: ArgMatches) {
        if let Some(a) = args.get_one::<String>("sign_in") {
            initiate_chat(a);
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