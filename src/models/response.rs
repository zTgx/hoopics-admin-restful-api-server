#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub code: u32,
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(code: u32, message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            code: code,
            message: message.to_string(),
            data,
        }
    }
}
