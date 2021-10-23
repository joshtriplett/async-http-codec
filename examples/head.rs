use async_http_codec::head::decode::RequestHeadDecoder;
use async_http_codec::head::encode::ResponseHeadEncoder;
use futures_lite::future::block_on;
use futures_lite::io::Cursor;
use http::Response;

const REQUEST: &[u8] = b"GET / HTTP/1.1\r\nHost: www.example.com\r\nConnection: close\r\n\r\n";

fn main() {
    block_on(async {
        let reader = Cursor::new(REQUEST);
        let request = RequestHeadDecoder::default().decode(reader).await.unwrap();
        dbg!(&request);

        let writer = Cursor::new(Vec::new());
        ResponseHeadEncoder::default()
            .encode(writer, Response::new(()))
            .await
            .unwrap();
        // let response = String::from_utf8(writer.into_inner()).unwrap();
        // dbg!(response);
    })
}
