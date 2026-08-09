// gif_convert.rs
use std::env;
use std::process::Command;
use std::path::Path;
use std::fs;

fn check_ffmpeg() {
    if Command::new("ffmpeg").arg("-version").output().is_err() {
        eprintln!("Ошибка: ffmpeg не найден. Установите FFmpeg и добавьте в PATH.");
        std::process::exit(1);
    }
}

fn build_cmd(input: &str, output: &str, fps: u32, scale: Option<String>, start: f64, duration: f64,
             quality: u32, filter: Option<String>, reverse: bool) -> Vec<String> {
    let mut cmd = vec!["ffmpeg".to_string(), "-i".to_string(), input.to_string(), "-y".to_string()];
    if start > 0.0 {
        cmd.push("-ss".to_string());
        cmd.push(start.to_string());
    }
    if duration > 0.0 {
        cmd.push("-t".to_string());
        cmd.push(duration.to_string());
    }
    let mut filters = Vec::new();
    if fps > 0 {
        filters.push(format!("fps={}", fps));
    }
    if let Some(sc) = scale {
        filters.push(format!("scale={}", sc));
    }
    if let Some(f) = filter {
        match f.as_str() {
            "grayscale" => filters.push("hue=s=0".to_string()),
            "sepia" => filters.push("colorchannelmixer=.393:.769:.189:.349:.686:.168:.272:.534:.131".to_string()),
            "negate" => filters.push("negate".to_string()),
            _ => {}
        }
    }
    if reverse {
        filters.push("reverse".to_string());
    }
    if !filters.is_empty() {
        cmd.push("-vf".to_string());
        cmd.push(filters.join(","));
    }
    let q = if quality < 1 { 1 } else if quality > 31 { 31 } else { quality };
    cmd.push("-q:v".to_string());
    cmd.push(q.to_string());
    cmd.push("-c:v".to_string());
    cmd.push("gif".to_string());
    cmd.push(output.to_string());
    cmd
}

fn convert(input: &str, output: &str, fps: u32, scale: Option<String>, start: f64, duration: f64,
           quality: u32, filter: Option<String>, reverse: bool) {
    check_ffmpeg();
    let cmd_args = build_cmd(input, output, fps, scale, start, duration, quality, filter, reverse);
    println!("Выполняется команда: {}", cmd_args.join(" "));
    let status = Command::new(&cmd_args[0])
        .args(&cmd_args[1..])
        .status()
        .expect("Не удалось запустить ffmpeg");
    if !status.success() {
        eprintln!("Ошибка конвертации");
        std::process::exit(1);
    }
    println!("✅ GIF создан: {}", output);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Использование: {} <input> [--output file] [--fps N] [--scale WxH] [--start S] [--duration D] [--quality Q] [--filter F] [--reverse]", args[0]);
        std::process::exit(1);
    }
    let mut input = String::new();
    let mut output = "output.gif".to_string();
    let mut fps = 10;
    let mut scale = None;
    let mut start = 0.0;
    let mut duration = 0.0;
    let mut quality = 10;
    let mut filter = None;
    let mut reverse = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--output" => {
                if i+1 < args.len() {
                    output = args[i+1].clone();
                    i += 2;
                } else { i += 1; }
            }
            "--fps" => {
                if i+1 < args.len() {
                    fps = args[i+1].parse().unwrap_or(10);
                    i += 2;
                } else { i += 1; }
            }
            "--scale" => {
                if i+1 < args.len() {
                    scale = Some(args[i+1].clone());
                    i += 2;
                } else { i += 1; }
            }
            "--start" => {
                if i+1 < args.len() {
                    start = args[i+1].parse().unwrap_or(0.0);
                    i += 2;
                } else { i += 1; }
            }
            "--duration" => {
                if i+1 < args.len() {
                    duration = args[i+1].parse().unwrap_or(0.0);
                    i += 2;
                } else { i += 1; }
            }
            "--quality" => {
                if i+1 < args.len() {
                    quality = args[i+1].parse().unwrap_or(10);
                    i += 2;
                } else { i += 1; }
            }
            "--filter" => {
                if i+1 < args.len() {
                    filter = Some(args[i+1].clone());
                    i += 2;
                } else { i += 1; }
            }
            "--reverse" => {
                reverse = true;
                i += 1;
            }
            _ => {
                if input.is_empty() {
                    input = args[i].clone();
                } else {
                    eprintln!("Неизвестный аргумент: {}", args[i]);
                }
                i += 1;
            }
        }
    }

    if input.is_empty() {
        eprintln!("Не указан входной файл.");
        std::process::exit(1);
    }
    if !Path::new(&input).exists() {
        eprintln!("Файл {} не найден.", input);
        std::process::exit(1);
    }

    convert(&input, &output, fps, scale, start, duration, quality, filter, reverse);
}
