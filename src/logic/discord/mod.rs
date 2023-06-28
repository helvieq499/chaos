pub mod dispatch;
pub mod gateway;

#[derive(Debug, serde::Deserialize)]
pub struct Payload {
    #[serde(flatten)]
    pub event: Event,

    #[serde(rename = "s")]
    pub sequence: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Event {
    DispatchEvent(dispatch::Event),
    GatewayEvent(gateway::Event),
    Raw(RawPayload),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RawPayload {
    #[serde(rename = "op")]
    pub opcode: u8,
    #[serde(rename = "d")]
    pub data: serde_json::Value,
    #[serde(rename = "t")]
    pub typ: Option<String>,
    #[serde(rename = "s")]
    pub sequence: Option<i32>,
}

impl RawPayload {
    pub const fn new(opcode: u8, data: serde_json::Value) -> Self {
        Self {
            opcode,
            data,
            typ: None,
            sequence: None,
        }
    }
}
