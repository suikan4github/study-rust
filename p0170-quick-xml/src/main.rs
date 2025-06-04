use quick_xml::events::Event;
use quick_xml::reader::Reader;

fn get_xml() -> String {
    let config_xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <radiru_config>

            <!-- お知らせ -->
            <info><![CDATA[/radio/include/oshirase.txt]]></info>
            
            <!-- 各地域のストリームURL -->
            <stream_url>
                <data>
                    <areajp>札幌</areajp>
                    <area>sapporo</area>
                    <apikey>700</apikey>
                    <areakey>010</areakey>
                    
                    <r1hls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023545/nhkradiruikr1/master.m3u8]]></r1hls>
                    <r2hls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023501/nhkradiruakr2/master.m3u8]]></r2hls>
                    <fmhls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023546/nhkradiruikfm/master.m3u8]]></fmhls>
                    
                </data>
                <data>
                    <areajp>仙台</areajp>
                    <area>sendai</area>
                    <apikey>600</apikey>
                    <areakey>040</areakey>
                    
                    <r1hls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023543/nhkradiruhkr1/master.m3u8]]></r1hls>
                    <r2hls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023501/nhkradiruakr2/master.m3u8]]></r2hls>
                    <fmhls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023544/nhkradiruhkfm/master.m3u8]]></fmhls>
                    
                </data>
                <data>
                    <areajp>東京</areajp>
                    <area>tokyo</area>
                    <apikey>001</apikey>
                    <areakey>130</areakey>

                    <r1hls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023229/nhkradiruakr1/master.m3u8]]></r1hls>
                    <r2hls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023501/nhkradiruakr2/master.m3u8]]></r2hls>
                    <fmhls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023507/nhkradiruakfm/master.m3u8]]></fmhls>
                </data>
                <data>
                    <areajp>名古屋</areajp>
                    <area>nagoya</area>
                    <apikey>300</apikey>
                    <areakey>230</areakey>
                    
                    <r1hls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023510/nhkradiruckr1/master.m3u8]]></r1hls>
                    <r2hls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023501/nhkradiruakr2/master.m3u8]]></r2hls>
                    <fmhls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023511/nhkradiruckfm/master.m3u8]]></fmhls>
                    
                </data>
                <data>
                    <areajp>大阪</areajp>
                    <area>osaka</area>
                    <apikey>200</apikey>
                    <areakey>270</areakey>
                    
                    <r1hls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023508/nhkradirubkr1/master.m3u8]]></r1hls>
                    <r2hls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023501/nhkradiruakr2/master.m3u8]]></r2hls>
                    <fmhls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023509/nhkradirubkfm/master.m3u8]]></fmhls>
                    
                </data>
                <data>
                    <areajp>広島</areajp>
                    <area>hiroshima</area>
                    <apikey>400</apikey>
                    <areakey>340</areakey>
                    
                    <r1hls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023512/nhkradirufkr1/master.m3u8]]></r1hls>
                    <r2hls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023501/nhkradiruakr2/master.m3u8]]></r2hls>
                    <fmhls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023513/nhkradirufkfm/master.m3u8]]></fmhls>
                    
                </data>
                <data>
                    <areajp>松山</areajp>
                    <area>matsuyama</area>
                    <apikey>800</apikey>
                    <areakey>380</areakey>
                    
                    <r1hls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023547/nhkradiruzkr1/master.m3u8]]></r1hls>
                    <r2hls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023501/nhkradiruakr2/master.m3u8]]></r2hls>
                    <fmhls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023548/nhkradiruzkfm/master.m3u8]]></fmhls>
                    
                </data>
                <data>
                    <areajp>福岡</areajp>
                    <area>fukuoka</area>
                    <apikey>501</apikey>
                    <areakey>400</areakey>
                    
                    <r1hls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023541/nhkradirulkr1/master.m3u8]]></r1hls>
                    <r2hls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023501/nhkradiruakr2/master.m3u8]]></r2hls>
                    <fmhls><![CDATA[https://radio-stream.nhk.jp/hls/live/2023542/nhkradirulkfm/master.m3u8]]></fmhls>
                    
                </data>
            </stream_url>
            
            <!-- noa api -->
            <url_program_noa><![CDATA[//api.nhk.jp/r7/pg/now/radio/{area}/now.json]]></url_program_noa>

            <!-- program detail api -->
            <url_program_day><![CDATA[//api.nhk.jp/r7/pg/date/{service}/{area}/[YYYY-MM-DD].json]]></url_program_day>

            <!-- program info api -->
            <url_program_detail><![CDATA[//api.nhk.jp/r7/t/broadcastevent/be/{broadcastEventId}.json]]></url_program_detail>

            <!-- tweet cgi @radiru -->
            <radiru_twitter_timeline><![CDATA[//cgi4.nhk.or.jp/tweet/api/tweet.cgi?twname=nhk_radiru]]></radiru_twitter_timeline>

        </radiru_config>
    "#;

    config_xml.to_string()
}

fn main() {
    let mut context_stack = Vec::new();
    let xml_string = get_xml();
    let mut reader = Reader::from_str(&xml_string);
    let mut buf = Vec::new();

    let mut context = "".to_string();
    let mut r1_url = "".to_string();
    let mut r2_url = "".to_string();
    let mut fm_url = "".to_string();
    let mut area = "".to_string();
    let mut result = "".to_string();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                context_stack.push(context.clone());
                context = String::from_utf8_lossy(e.name().as_ref()).to_string();
            }

            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap().to_string();
                if context == "areajp" {
                    println!("地域: {}", text);
                } else if context == "area" {
                    area = text;
                }
            }
            Ok(Event::CData(e)) => {
                let text = String::from_utf8_lossy(&e).to_ascii_lowercase().to_string();
                if context == "r1hls" {
                    r1_url = text;
                } else if context == "r2hls" {
                    r2_url = text;
                } else if context == "fmhls" {
                    fm_url = text;
                }
            }
            Ok(Event::End(_)) => {
                if context == "data" {
                    if area == "tokyo" {
                        result = r2_url.clone();
                    }
                    r1_url.clear();
                    r2_url.clear();
                    fm_url.clear();
                    area.clear();
                }
                context = context_stack.pop().unwrap_or_default();
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    println!("Result is  {}", result);
}
