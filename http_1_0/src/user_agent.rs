use crate::Request;
use crate::Response;

/// オリジンサーバーにリクエストを送る存在。
/// 実際はブラウザだったり、Web探索ロボットだったり、その他のエンドユーザーの使うツールだったりする。
#[derive(Debug)]
pub struct UserAgent;

/// ユーザーエージェントがリクエストを送って、そのレスポンスが帰ってくるまでに何らかの不具合があり
/// 一連のアクションが達成できなかったときに発生するエラー
#[derive(thiserror::Error, Debug)]
pub enum RequestError {
    // とりあえず仮置きのエラー
    #[error("something happens")]
    SomthingHappens,
}

impl UserAgent {
    pub fn new() -> Self {
        Self
    }

    pub fn send_request(&self, _: Request) -> Result<Response, RequestError> {
        Ok(Response::new())
    }
}
