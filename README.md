<img src="https://github.com/Alex-J-Christie/ai_for_senior_citizens_ist440w_24/blob/main/test/openAI_sencit_test/icon.png" align="right" width="225" alt="Chatbot Logo" title="Chatbot Logo">

# IST440W AI Chatbot for Senior Citizens Project

An AI Chatbot Designed to be **Comforting**, **Engaging** and **Fun**!

## Advanced Technology

This project uses powerful technology to create a fully featured, comprehensive tool for creating a companion for seniors

* Written in Rust :crab:
* Powered by OpenAI :robot:
* Brought to Life by ChatGPT's Voice API :basecamp:

## Flexible Interface

  The senior citizen chatbot is designed with extensibility in mind.
A robust, database driven backend allows for many potential UI integrations,
including:

- [x] CLI
- [x] GUI (Desktop)
- [ ] TUI
- [ ] GUI (Movil)

## Get Started

~~### Binary Usage~~ (Builds not ready yet!)

### Development Usage

| Requirements  | Documentation |
| ------------- | ------------- |
| Rust       | https://www.rust-lang.org/learn/get-started               |
| OpenAI API | https://platform.openai.com/docs/overview                 |
| OpenSSL |  https://www.openssl.org/  |
| Vulkan |  https://en.wikipedia.org/wiki/Vulkan  |


**Note: During Compilation, Windows Antivirus May to Flag Certain Dependencies as Threats**

**Note: WSL is Not a Supported Option as Vulkan is the Necessary Backend (OpenGL May Be Implemented Later)**

1. Install Rust on your PC [rustup is recommended](https://rustup.rs/) along with other dependencies (OpenSSL, Appropriate Vulkan drivers for your pc)
2. Create an [OpenAI](https://platform.openai.com/docs/overview) Account
   - Get an [OpenAI API Key](https://platform.openai.com/docs/guides/production-best-practices) and Add Credit to It
3. Download the Senior Citizen Chatbot Repository and create the '.env' file in the test/openAI_sencit_test/ folder
   - Example:
```
git clone https://github.com/Alex-J-Christie/ai_for_senior_citizens_ist440w_24
cd ai_for_senior_citizens_ist440w_24/test/openAI_sencit_test/
touch .env
```
4. In a text editor, open the '.env' file and type 'OPENAI_KEY=XX-proj-XX' (replace XX-proj-XX with your project key)
5. From here, you can edit the code in a text editor and either build it or run it with [cargo](https://github.com/rust-lang/cargo)
   - Here is the man page
```
[user@DistroBox openAI_sencit_test]$ cargo run -- -h
   Compiling openAI_sencit_test v0.1.0 (/home/[user]/Projects/ai_for_senior_citizens_ist440w_24/test/openAI_sencit_test)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.79s
     Running `target/debug/openAI_sencit_test -h`
Usage: openAI_sencit_test [OPTIONS]

Options:
  -s, --sign_in <Username>  Use your username to talk to your tailored chatbot
  -t, --test                Test for database functionality
  -g, --gui                 start gui
  -h, --help                Print help
  -V, --version             Print version
```
## Todo! List

- [x] Database Backend
- [x] GUI
- [x] Voice Integration
- [x] Voice Input
- [ ] Portability
