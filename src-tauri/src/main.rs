use chrono::DateTime;
use chrono::Utc;
use reqwest::Response;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, ACCEPT};

const URL: &str = "https://api-v2.solscan.io/v2/account?address=HeMtKjiwCohxM8dnMABCVooJREEPc2vY1YWU4nH8pump&view_as=token";

struct ReqClient {
    client: reqwest::Client
}

impl ReqClient {
    fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        let client = reqwest::Client::builder().default_headers(headers).build().unwrap();
        Self { client: client }
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