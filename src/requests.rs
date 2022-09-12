pub trait ApiRequestDescription {
	fn build_request(&self) -> (&str, serde_json::Value);
}

pub enum Request {}
