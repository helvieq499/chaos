#[derive(Debug)]
pub struct RecvEvent {
    pub opcode: i32,
    pub data: json::JsonValue,
    pub sequence: Option<i32>,
    pub typ: Option<String>
}

impl TryFrom<json::JsonValue> for RecvEvent {
    type Error = ();

    fn try_from(value: json::JsonValue) -> Result<Self, Self::Error> {
        if let Some(opcode) = value["op"].as_i32() {
            let (sequence, typ) = match opcode {
                0 => (value["s"].as_i32(), value["t"].as_str().map(|s| s.to_string())),
                _ => (None, None),
            };

            Ok(Self {
                opcode,
                data: value["data"].clone(),
                sequence,
                typ,
            })
        } else {
            Err(())
        }
    }
}
