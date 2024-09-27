    use clap::{Arg, arg, ArgMatches, command};
    use crate::chat::initiate_chat;
    use crate::db_man::test;
    use crate::gui_view;

    pub fn run_args(args: ArgMatches) {
        if let Some(a) = args.get_one::<String>("sign_in") {
            initiate_chat(a);
        }
        if let Some(b) = args.get_one::<bool>("test") {
            if *b {
                test();
            }
        }
        if let Some(c) = args.get_one::<bool>("gui") {
            if *c {
                gui_view::main().expect("TODO: panic message");
            }
        }
    }

    pub fn return_arg_array() -> ArgMatches {
        let sign_in: Arg = arg!(-s --sign_in <Username> "Use your username to talk to your tailored chatbot");
        let test: Arg = arg!(-t --test "Test for database functionality");
        let gui: Arg = arg!(-g --gui "start gui");
        command!().args([
            sign_in,
            test,
            gui
        ]).get_matches()
    }