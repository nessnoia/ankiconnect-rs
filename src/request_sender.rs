use crate::anki_connect_client::{AnkiConnectResponse, AnkiConnectRequest};
use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::error::AnkiRequestError;

pub trait AnkiConnectRequestSender {
    fn send_request<ReqParam: Serialize, ResType: DeserializeOwned>(
        &self,
        request: AnkiConnectRequest<ReqParam>,
    ) -> Result<AnkiConnectResponse<ResType>, AnkiRequestError>;
}

pub struct HttpAnkiConnectRequestSender {
    port: u16,
    url: String,
}

impl HttpAnkiConnectRequestSender {
    pub fn new(url: &str, port: u16) -> Self {
        Self {
            port,
            url: url.to_string(),
        }
    }

    pub fn get_url_with_port(&self) -> String {
        format!("http://{}:{}", self.url, self.port)
    }
}

impl AnkiConnectRequestSender for HttpAnkiConnectRequestSender {
    fn send_request<ReqParam: Serialize, ResType: DeserializeOwned>(
        &self,
        request: AnkiConnectRequest<ReqParam>,
    ) -> Result<AnkiConnectResponse<ResType>, AnkiRequestError> {
        ureq::post(self.get_url_with_port())
            .send_json(&request)?
            .body_mut()
            .read_json()
            .map_err(|e| AnkiRequestError::HttpError(e))
    }
}
