use chrono::DateTime;
use chrono::Utc;
use reqwest::Response;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GetSigPayload {
    jsonrpc: String,
    id: i32,
    method: String,
    params: (String, Option<GetSigParam>)
}

#[derive(Serialize, Deserialize)]
struct GetSigParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct RpcError {
    code: i64,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct SigObj {
    signature: String,
    slot: u64,
    err: Option<String>,
    memo: Option<String>,
    #[serde(rename = "blockTime")]
    block_time: Option<i64>,
    #[serde(rename = "confirmationStatus")]
    confirmation_status: Option<String>
}

#[derive(Serialize, Deserialize)]
struct SigResp {
    jsonrpc: String,
    id: i32,
    #[serde(default)]
    result: Option<Vec<SigObj>>,
    #[serde(default)]
    error: Option<RpcError>
}

struct SolClient {
    base_url: String,
    client: reqwest::Client
}

struct SigContainer {
    sigs: Vec<SigObj>,
    next_sig: Option<String>
}

impl SolClient {
    fn new() -> Self {
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        let client: reqwest::Client = reqwest::Client::builder().default_headers(headers).build().unwrap();

        Self {
            client: client,
            base_url: "https://api.devnet.solana.com".to_string()
        }
    }

    // takes address and start sig (optional, for pagination) and returns a list of signatures
    async fn get_sigs(self, address: String, start_sig: Option<String>) -> Result<SigContainer, String> {
        let params: GetSigParam = GetSigParam {
            commitment: Some("finalized".to_string()),
            limit: Some(100),
            before: start_sig,
            since: None
        };

        let payload: GetSigPayload = GetSigPayload {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "getSignaturesForAddress".to_string(),
            params: (address, Some(params))
        };

        let response: Response = self.client.post(self.base_url).json(&payload).send().await.unwrap();
        let body: String = response.text().await.unwrap();
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        let result: SigResp = serde_json::from_value(json).unwrap();
        
        Ok(SigContainer { sigs: result, next_sig: next_sig })
    }

    async fn get_creation_time(self, address: String) -> Result<String, String> -> Result<String, String> {
        
    }

}

#[tauri::command]
async fn get_info(info_type: String) -> Result<String, String> {
    eprintln!("[get_info] Called with info_type: {}", info_type);
    if info_type == "timestamp" {
        let client: ReqClient = ReqClient::new();
        let response: Response = client.client.get(URL).send().await.unwrap();
        eprintln!("[get_info] Response status: {}", response.status());
        let body: String = response.text().await.unwrap();
        eprintln!("[get_info] Body: {}", body);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        let timestamp: i64 = json["data"]["tokenInfo"]["created_time"].as_i64().unwrap_or(0);
        let datetime: DateTime<Utc> = DateTime::from_timestamp(timestamp, 0).unwrap();
        Ok(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
    } else {
        Ok("Hello, world!".to_string())
    }
}

fn main() {
    tauri::Builder::default().invoke_handler(tauri::generate_handler![
        get_info
    ]).run(tauri::generate_context!()).expect("error");
}