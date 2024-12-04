<div style="text-align:center;">
  <img src="https://github.com/Alex-J-Christie/ai_for_senior_citizens_ist440w_24/blob/main/test/openAI_sencit_test/icon.png" width="225" alt="Chatbot Logo" title="Chatbot Logo">
</div>

# KiYa - AI for Seniors
## IST440W AI Chatbot for Senior Citizens Project

An AI Chatbot Designed to be **Comforting**, **Engaging** and **Fun**!

## Advanced Technology

KiYa (Kidemli Yardımcı) AI uses powerful technology to create a fully featured, comprehensive tool for creating a companion for seniors

* Written in Rust :crab:
* Powered by OpenAI :robot:
* Brought to Life by ChatGPT's Voice API :basecamp:

## Flexible Interface

  KiYa is designed with extensibility in mind.
A robust, database driven backend allows for many potential UI integrations,
including:

- [x] CLI
- [x] GUI (Desktop)
- [ ] TUI
- [ ] GUI (Movil)

## Get Started

### Binary Usage 

| Requirements  | Documentation |
| ------------- | ------------- |
| OpenAI API | https://platform.openai.com/docs/overview                 |
| OpenSSL |  https://www.openssl.org/  |
| Vulkan |  https://en.wikipedia.org/wiki/Vulkan  |
| Linux/Windows |    |

**Note: During Compilation, Windows Antivirus May to Flag Certain Dependencies as Threats**

**Note: MacOS, BSD Builds Not Available**

**Note: WSL is Not a Supported Option as Vulkan is the Necessary Backend (OpenGL May Be Implemented Later)**

1. Download the binary files in the Releases section at [Github Releases](https://github.com/Alex-J-Christie/ai_for_senior_citizens_ist440w_24/releases)
2. Install related dependencies (OpenSSL, Appropriate Vulkan drivers for your pc)
3. Create an [OpenAI](https://platform.openai.com/docs/overview) Account
   - Get an [OpenAI API Key](https://platform.openai.com/docs/guides/production-best-practices) and Add Credit to It
4. In the same directory as your selected KiYa binary, create a text file name '.env.'
5. In a text editor, open the '.env' file and type 'OPENAI_KEY=XX-proj-XX' (replace XX-proj-XX with your project key)
6. Create a application shortcut that points to the binary
    - Alternatively, run binary directly in terminal ('start KiYa.exe -h' for windows, './KiYa -h' or 'sh KiYa -h' on linux (may need to chmod +x first))
8. add '-g' as a launch parameter
9. run the shortcut

### Development Usage

| Requirements  | Documentation |
| ------------- | ------------- |
| Rust       | https://www.rust-lang.org/learn/get-started               |
| OpenAI API | https://platform.openai.com/docs/overview                 |
| OpenSSL |  https://www.openssl.org/  |
| Vulkan |  https://en.wikipedia.org/wiki/Vulkan  |
| Linux/Windows/MacOS/BSD |    |

**Note: During Compilation, Windows Antivirus May to Flag Certain Dependencies as Threats**

**Note: WSL is Not a Supported Option as Vulkan is the Necessary Backend (OpenGL May Be Implemented Later)**

1. Install Rust on your PC [rustup is recommended](https://rustup.rs/) along with other dependencies (OpenSSL, Appropriate Vulkan drivers for your pc)
2. Create an [OpenAI](https://platform.openai.com/docs/overview) Account
   - Get an [OpenAI API Key](https://platform.openai.com/docs/guides/production-best-practices) and Add Credit to It
3. Download the KiYa Repository and create the '.env' file in the test/openAI_sencit_test/ folder
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
