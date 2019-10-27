pub struct Website {
	pub url: String,
	pub hash: String,
	pub status: String 
}

impl Website {
	pub fn getUrl(self) -> String {
		"hello".to_string()
	}
}