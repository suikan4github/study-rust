use std::fs::File;
use std::io::copy;

// 内部で非同期処理を行う時には、この属性が必要。
#[tokio::main]
// 非同期関数として定義する。
async fn main() {   
    // NHKのラジオストリームURL情報を取得するためのURL。
    let url_str = "https://www.nhk.or.jp/radio/config/config_web.xml";
    let output_file_name = "config_web.xml";

    // URLからデータを取得する。この今回はXML形式のデータとわかっているので
    // レスポンスを文字列として取得する。

    // ネットワーク通信を開始する。
    // get()が返すのは、非同期処理のためのFuture構造体
    let request_future = reqwest::get(url_str);

    // 通信によりヘッダが帰ってくるのを待つ
    // ここで初めてレスポンスを得る。ただし、レスポンス・ボディは未着の可能性がある。 
    let response_struct = request_future
        .await // ヘッダの到着を待つ
        .expect("It could get the response from the URL");
    
    // レスポンスをテキストに変換する。
    // text()が返すのは、非同期処理のためのFuture構造体。
    let text_conversion_future = response_struct.text(); 

    // 変換が終了するのを待つ。
    let response: String = text_conversion_future
        .await  // 文字列変換の完了を待つ
        .expect("It could be converted to text");

    // レスポンスを格納するファイルを作成する。
    let mut file = File::create(output_file_name).expect("It could create the output file");

    // レスポンスの内容をファイルに書き込む。
    copy(&mut response.as_bytes(), &mut file).expect("It could copy the response to the file");
}
