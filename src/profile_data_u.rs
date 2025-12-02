#[cfg(test)]
mod tests {
	use crate::{ WindowRelativeProfileData, WindowRelativeProfileDataConvertible };

	

	struct RawBytes(Vec<u8>);
	impl WindowRelativeProfileDataConvertible for RawBytes {
		fn to_bytes(self) -> Vec<u8> {
			self.0
		}
		fn from_bytes(bytes:&[u8]) -> Option<Self> {
			Some(RawBytes(bytes.to_vec()))
		}
	}

	struct EmptyBytes;
	impl WindowRelativeProfileDataConvertible for EmptyBytes {
		fn to_bytes(self) -> Vec<u8> { Vec::new() }
		fn from_bytes(_bytes:&[u8]) -> Option<Self> { Some(EmptyBytes) }
	}



	#[test]
	fn set_and_get_bool_roundtrip() {
		let mut data:WindowRelativeProfileData = WindowRelativeProfileData::new();
		data.set("flag", true);
		assert_eq!(data.get("flag"), Some(true));
		data.set("flag", false);
		assert_eq!(data.get("flag"), Some(false));
	}

	#[test]
	fn missing_key_returns_none() {
		let mut data:WindowRelativeProfileData = WindowRelativeProfileData::new();
		data.set("present", 123u32);
		assert!(data.get::<u32>("not-present").is_none());
	}

	#[test]
	fn string_conversion() {
		let mut data:WindowRelativeProfileData = WindowRelativeProfileData::new();
		data.set("greeting", String::from("hello world"));
		assert_eq!(data.get("greeting"), Some(String::from("hello world")));

		let invalid:RawBytes = RawBytes(vec![0xff, 0xfe, 0xfd]);
		data.set("bad_utf8", invalid);
		assert!(data.get::<String>("bad_utf8").is_none());
	}

	#[test]
	fn numeric_conversion() {
		let mut data:WindowRelativeProfileData = WindowRelativeProfileData::new();
		data.set("u8", 0x12u8);
		data.set("u16", 0x1234u16);
		data.set("u32", 0x12345678u32);
		data.set("i64", -42i64);
		data.set("f64", 3.14159f64);
		assert_eq!(data.get("u8"), Some(0x12u8));
		assert_eq!(data.get("u16"), Some(0x1234u16));
		assert_eq!(data.get("u32"), Some(0x12345678u32));
		assert_eq!(data.get("i64"), Some(-42i64));
		assert_eq!(data.get("f64"), Some(3.14159f64));
	}

	#[test]
	fn vec_conversion() {
		let mut data:WindowRelativeProfileData = WindowRelativeProfileData::new();
		let v:Vec<String> = vec!["a".to_string(), "bb".to_string(), "ccc".to_string()];
		data.set("strings", v);

		assert_eq!(data.get::<Vec<String>>("strings"), Some(vec!["a".to_string(), "bb".to_string(), "ccc".to_string()]));
	}

	#[test]
	fn raw_bytes_conversion() {
		let mut data:WindowRelativeProfileData = WindowRelativeProfileData::new();
		let payload:Vec<u8> = vec![0xde, 0xad, 0xbe, 0xef];
		data.set("payload", RawBytes(payload.clone()));
		assert_eq!(data.get::<RawBytes>("payload").map(|raw| raw.0), Some(payload));
	}

	#[test]
	fn setting_overwrites() {
		let mut data:WindowRelativeProfileData = WindowRelativeProfileData::new();
		data.set("one", 1i32);
		data.set("two", 2i32);
		data.set("three", 3i32);
		data.set("two", 22i32);

		assert_eq!(data.get::<i32>("one"), Some(1i32));
		assert_eq!(data.get::<i32>("three"), Some(3i32));
		assert_eq!(data.get::<i32>("two"), Some(22i32));
	}
}
