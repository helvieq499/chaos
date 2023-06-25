#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RecvEvent {
    #[serde(rename = "op")]
    pub opcode: i32,
    #[serde(rename = "d")]
    pub data: serde_json::Value,
    #[serde(default)]
    #[serde(rename = "s")]
    pub sequence: Option<i32>,
    #[serde(default)]
    #[serde(rename = "t")]
    pub typ: Option<String>,
}

impl RecvEvent {
    pub const fn new(opcode: i32, data: serde_json::Value) -> Self {
        Self {
            opcode,
            data,
            sequence: None,
            typ: None,
        }
    }
}
