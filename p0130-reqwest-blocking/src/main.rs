use std::fs::File;
use std::io::copy;

fn main() {
    // NHKのラジオストリームURL情報を取得するためのURL。
    let url_str = "https://www.nhk.or.jp/radio/config/config_web.xml";
    let output_file_name = "config_web.xml";

    // URLからデータを取得する。この今回はXML形式のデータとわかっているので
    // レスポンスを文字列として取得する。
    let response: String = reqwest::blocking::get(url_str)
        .expect("It could get the response from the URL")
        .text()
        .expect("It could be converted to text");

    // レスポンスを格納するファイルを作成する。
    let mut file = File::create(output_file_name).expect("It could create the output file");

    // レスポンスの内容をファイルに書き込む。
    copy(&mut response.as_bytes(), &mut file).expect("It could copy the response to the file");
}
