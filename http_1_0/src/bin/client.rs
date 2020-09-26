fn main() {
    let ua = http_1_0::UserAgent::new();

    let request = http_1_0::Request::new(/* 何らかのパラメーターが入る */);
    println!("リクエストメッセージ:\n{:?}", request);
    match ua.send_request(request) {
        Ok(response) => {
            println!("レスポンスメッセージ:\n{:?}", response);
        }
        Err(err) => {
            panic!("エラーです！！！:\n{:?}", err);
        }
    }
}
