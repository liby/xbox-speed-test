use clap::Parser;
use std::cmp::Ordering::Equal;
use std::io::{BufRead, BufReader};
use std::process::Command;

static CDN_FILE: &[u8] = include_bytes!("../cdn.list");
static CDN_CN_FILE: &[u8] = include_bytes!("../cdn-cn.list");

/// Xbox SpeedTest
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Use China CDN list
    #[arg(short, long)]
    is_cn: bool,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Output {
    ip: String,
    speed: f32,
    speed_str: String,
}

fn main() {
    let args = Args::parse();

    let cdn_list = if args.is_cn { CDN_CN_FILE } else { CDN_FILE };
    let mut result = Vec::with_capacity(4096);

    let mut cdn_file = BufReader::new(cdn_list);
    let mut line = String::new();
    cdn_file.read_line(&mut line).unwrap();

    println!("===开始测试===");
    for line in cdn_file.lines() {
        let ip = line.unwrap();

        let (url, host, range) = if args.is_cn {
            (format!("http://{}/5/0454fb03-6c81-496a-aee8-96c8e338941e/a124f922-f8fa-412c-84c1-f418ee71632a/1.0.0.21.a3ef4e36-0cfb-47b0-9eda-eab7cb5c076c/EastshadeStudios.Eastshade_1.0.0.21_neutral__dc285gd3x0dar", &ip), "Host: assets1.xboxlive.cn", "0-337437055")
        } else {
            (format!("http://{}/5/795514b6-aad9-4c1c-ac2a-60c1492d7f31/0c57204f-f4f0-4bf6-b119-b7afc231994d/0.0.61375.0.6574fcb5-72f2-4c85-98c1-bd1059c79934/Destiny2_0.0.61375.0_neutral__z7wx9v9k22rmg", &ip), "Host: assets1.xboxlive.com", "33543139328-33752035327")
        };
        let output = Command::new("curl")
            .args([
                "-s",
                "-o",
                "/dev/null",
                "-m",
                "8",
                "-r",
                range,
                "-y",
                "5",
                "--url",
                &url,
                "-H",
                host,
                "-w",
                "%{speed_download}",
            ])
            .output()
            .expect("failed to execute process");

        result.push(if !output.stdout.is_empty() {
            let speed = String::from_utf8_lossy(&output.stdout);
            handle(ip, speed.to_string())
        } else {
            let speed = String::from_utf8_lossy(&output.stderr);
            handle(ip, speed.to_string())
        });
    }

    println!("===下载速度排序===");
    sort_and_print(result);
}

fn handle(ip: String, output: String) -> Output {
    let stdout = output.parse::<f32>().expect("Output is NaN");
    let speed = stdout / 1000000.0;

    let speed_str = format!("{:.2} Mb/s", speed);
    println!("[ip] {:15} [speed] {}", ip, &speed_str);
    Output {
        ip,
        speed: stdout,
        speed_str,
    }
}

fn sort_and_print(mut result: Vec<Output>) {
    result.sort_by(|a, b| b.speed.partial_cmp(&a.speed).unwrap_or(Equal));
    result
        .iter()
        .for_each(|x| println!("[ip] {:15} [speed] {}", x.ip, x.speed_str))
}
