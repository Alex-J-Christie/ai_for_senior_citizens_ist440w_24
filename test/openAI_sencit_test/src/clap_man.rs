use crate::chat::initiate_chat;
use crate::db_man::test;
use crate::gui_view;
use crate::sttttts::transcribe;
use clap::{arg, command, Arg, ArgMatches};
use std::path::PathBuf;

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
        if let Some(d) = args.get_one::<bool>("test_tts") {
            if *d {
                println!("{}", "Test TTS");
                // let audio: PathBuf = PathBuf::from("test-mic.wav");
                // sttttts::transcribe(audio);
                // sttttts::generate_audio(String::from("Whoa, that's really good Jason!"), String::from("alloy"));
                // get_audio_input().expect("Failed to find mic");
                println!("response: {}", transcribe(PathBuf::from("output.wav")));
            }
        }
    }

    pub fn return_arg_array() -> ArgMatches {
        let sign_in: Arg = arg!(-s --sign_in <Username> "Use your username to talk to your tailored chatbot");
        let test: Arg = arg!(-t --test "Test for database functionality");
        let gui: Arg = arg!(-g --gui "start gui");
        let tts_test: Arg = arg!(-c --test_tts "Test with tts functionality");
        command!().args([
            sign_in,
            test,
            gui,
            tts_test,
        ]).get_matches()
    }