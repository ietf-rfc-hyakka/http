//! ここではユーザーエージェントがオリジンサーバーに送るメッセージである
//! 「リクエスト」について規定する

/// ユーザーエージェントがオリジンサーバーに送るメッセージ
#[derive(Debug)]
pub struct Request;

impl Request {
    pub fn new() -> Self {
        Request
    }
}
