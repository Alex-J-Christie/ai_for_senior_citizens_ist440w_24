use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, SupportedStreamConfig};
use curl::easy::{Easy, List};
use dotenvy::dotenv;
use mp3_duration;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use serde_json::json;
use std::env;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use std::time::Duration;
use hound::{SampleFormat, WavSpec, WavWriter};
use tokio::task;

//I have tried to get mp3 file duration for 40 minutes, we're just going to assume the response is never longer than like 20 seconds im done
//never mind, doing that ruins everything so we need to get exact audio duration - i hate it here
//i found a crate that just does it for me im a genius
pub async fn play_audio() {
    // let file: File = File::open("output.mp3").unwrap();
    // let decoder: Decoder<File> = Decoder::new_mp3(file).unwrap();
    //
    // let samples = &decoder.convert_samples().collect::<Vec<u32>>();
    // let total_rate = &decoder.sample_rate();
    //
    // let duration = decoder.total_duration().unwrap();
    // println!("{:?}", duration);

    //crappy code - i rely on manually entering file paths more than once on this page. ill fix it when i have time and i want to
    let file: File = File::open("output.mp3").unwrap();
    let source: Decoder<BufReader<File>> = Decoder::new(BufReader::new(file)).expect("failed to decode WAV file");

    let host: Host = cpal::default_host();
    let device: Device = host.default_output_device().expect("failed to get default output device");
    let _config: SupportedStreamConfig = device.default_output_config().expect("failed to get default output config");

    let (_stream, stream_handle): (OutputStream, OutputStreamHandle) =
        OutputStream::try_default().expect("failed to get default output stream");

    task::spawn(async move {
        stream_handle.play_raw(source.convert_samples()).expect("failed to play audio");
        tokio::time::sleep(get_file_duration()).await;
    }).await.unwrap();
    // std::thread::sleep(duration);
}

fn get_file_duration() -> Duration {
    let file: File = File::open("output.mp3").unwrap();
    let duration = mp3_duration::from_file(&file).unwrap();
    duration
}

pub async fn generate_audio(audio_in: String, voice: String) {
    dotenv().unwrap();
    let api_key: String = env::var("OPENAI_KEY").unwrap();
    let mut easy = Easy::new();

    easy.url("https://api.openai.com/v1/audio/speech").unwrap();
    easy.post(true).unwrap();

    let mut headers = List::new();
    headers.append(&format!("Authorization: Bearer {}", api_key)).unwrap();
    headers.append("Content-Type: application/json").unwrap();
    easy.http_headers(headers).unwrap();


    let json_payload = json!({
        "model": "tts-1",
        "input": audio_in,
        "voice": voice
    });
    let json_str = serde_json::to_string(&json_payload).unwrap();

    easy.post_fields_copy(json_str.as_bytes()).unwrap();

    let mut output_file = File::create("output.mp3").unwrap();

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            output_file.write_all(data).unwrap();
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let response = easy.response_code().unwrap();
    match response {
        200 => {
            // println!("Audio generated at output.mp3");
            play_audio().await;
        },
        _ => println!("Error: {}", response)
    }

}

pub fn _transcribe(audio_in: PathBuf) {
    dotenv().unwrap();
    let api_key: String = env::var("OPENAI_KEY").unwrap();
    let test_file: PathBuf = PathBuf::from(audio_in);
    let mut easy = Easy::new();

    easy.url("https://api.openai.com/v1/audio/speech").unwrap();
    easy.post(true).unwrap();

    let mut headers = List::new();
    headers.append(&format!("Authorization: Bearer {}", api_key)).unwrap();
    headers.append("Content-Type: multipart/form-data").unwrap();
    easy.http_headers(headers).unwrap();

    let mut form = curl::easy::Form::new();
    form.part("file")
        .file(&test_file)
        .add().unwrap();
    form.part("model")
        .contents("whisper-1".as_ref())
        .add().unwrap();

    easy.httppost(form).unwrap();

    let mut response = Vec::new();
    {
    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        response.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
    }

let response: String = String::from_utf8(response).unwrap();
    println!("{}", response);
}

//i am dead - low level audio libraries killed me
//its 1 am im just gonna look for a different library tomorrow
pub fn get_audio_input() {

    let _hosts = cpal::available_hosts();
    let host = cpal::default_host();
    // println!("Available hosts: {:?}", hosts);
    // println!("Host: {:?}", host.id());
    let devices = host.input_devices().expect("failed to get input devices");

    let device_names: Vec<Device> = devices.into_iter()
        .filter(|device| device.name().unwrap().contains("sysdefault"))
        .collect::<Vec<Device>>();

    println!("\n\n\n");

    for (ind, device_name) in device_names.clone().iter().enumerate() {
        println!("device {}: {}", ind, device_name.name().unwrap());
    }

    let selected_device = device_names.get(1).unwrap();
    let mut supported_configs = selected_device
        .supported_input_configs()
        .expect("failed to get supported input configs");

    // for config in supported_configs {
    //     println!("Supported config: {:?}", config);
    // }

    let config = supported_configs
        .next().unwrap();

    // println!("config {:?}", config.min_sample_rate().0);

    let sample_format = config.sample_format();
    let channels = config.channels();
    let sample_rate = config.min_sample_rate().0 * 2;

    let spec = WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut writer = WavWriter::create("output.wav", spec)
        .expect("Failed to create WAV file");


    let stream = selected_device
        .build_input_stream(
            &config.with_max_sample_rate().into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                for &sample in data.iter() {
                    let int_sample = (sample * i16::MAX as f32) as i16;
                    writer.write_sample(int_sample)
                        .expect("failed to write to WAV file");
                }
            },
            |err| {
                eprintln!("{}", err);
            },
            None
        )
        .expect("failed to build input stream");

    stream.play().unwrap();

    std::thread::sleep(Duration::from_secs(10));
}
