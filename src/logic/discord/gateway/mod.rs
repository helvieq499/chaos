pub mod hello;

#[derive(Debug, serde::Deserialize)]
// #[serde(tag = "op", content = "d")]
#[serde(untagged)]
pub enum Event {
    // #[serde(rename = "1")]
    Heartbeat {
        op: OpCode<1>,
        #[serde(rename = "d")]
        data: serde_json::Value,
    },

    // #[serde(rename = "7")]
    Reconnect {
        op: OpCode<7>,
        #[serde(rename = "d")]
        data: serde_json::Value,
    },

    // #[serde(rename = "9")]
    InvalidSession {
        op: OpCode<9>,
        #[serde(rename = "d")]
        data: serde_json::Value,
    },

    // #[serde(rename = "10")]
    Hello {
        op: OpCode<10>,
        #[serde(rename = "d")]
        data: hello::HelloData,
    },

    // #[serde(rename = "11")]
    HeartbeatAck {
        op: OpCode<11>,
        #[serde(rename = "d")]
        data: serde_json::Value,
    },
}

// https://github.com/serde-rs/serde/issues/745#issuecomment-1450072069
#[derive(Debug)]
pub struct OpCode<const V: u8>;

impl<'de, const V: u8> serde::Deserialize<'de> for OpCode<V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        if value == V {
            Ok(Self)
        } else {
            Err(serde::de::Error::custom("Invalid Version"))
        }
    }
}
