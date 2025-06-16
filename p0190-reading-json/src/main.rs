// series.jsonを読み込んで、各シリーズのプログラムタイトルと、ストリームのURLを表示する。
use serde::{Deserialize, Serialize};
use std::fs;

// エピソード構造体を定義する
#[derive(Serialize, Deserialize, Debug)]
//#[allow(non_snake_case)]
struct Episode {
    id: u32,
    program_title: String,
    onair_date: String,
    stream_url: String,
    program_sub_title: String,
}

// シリーズ構造体を定義する
#[derive(Serialize, Deserialize, Debug)]
//#[allow(non_snake_case)]
struct Series {
    id: u32,
    title: String,
    episodes: Vec<Episode>,
}
fn main() {
    // series.jsonを読み込む
    let file_path = "series.json";
    let file_content = fs::read_to_string(file_path).expect("Unable to read file");

    // JSONをデシリアライズする
    let series: Series = serde_json::from_str(&file_content).expect("JSON was not well-formatted");

    // 各シリーズのタイトルとエピソードのストリームURLを表示する
    println!("Series Title: {}", series.title);
    for e in series.episodes {
        println!(
            r#"{{"program_title": "{}","stream_url": "{}"}},"#,
            e.program_title, e.stream_url
        );
    }
}
