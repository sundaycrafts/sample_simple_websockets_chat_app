#[derive(Debug)]
pub struct ConnectionId(Option<String>);

impl From<Option<String>> for ConnectionId {
    fn from(value: Option<String>) -> Self {
        ConnectionId(value)
    }
}

impl TryInto<(String, String)> for ConnectionId {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> Result<(String, String), Self::Error> {
        let cid = self.0.ok_or("connectionId is not found")?;
        Ok(("connectionId".to_string(), cid))
    }
}
