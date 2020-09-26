//! ここではオリジンサーバーがユーザーエージェントに送り返すメッセージである
//! 「レスポンス」について規定する

/// オリジンサーバーがユーザーエージェントに送り返すメッセージ
#[derive(Debug)]
pub struct Response;

impl Response {
    pub fn new() -> Self {
        Response
    }
}
