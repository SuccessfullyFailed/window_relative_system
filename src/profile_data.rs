


pub struct WindowRelativeProfileData(Vec<(String, Vec<u8>)>);
impl WindowRelativeProfileData {
	
	/// Create a new data set.
	pub fn new() -> WindowRelativeProfileData {
		WindowRelativeProfileData(Vec::new())
	}

	/// Write some data.
	pub fn set<T:WindowRelativeProfileDataConvertible>(&mut self, name:&str, data:T) {
		match self.0.iter().position(|(stored_name, _)| stored_name == name) {
			Some(index) => self.0[index].1 = data.to_bytes(),
			None => self.0.push((name.to_string(), data.to_bytes()))
		}
	}

	/// Read some data.
	pub fn get<T:WindowRelativeProfileDataConvertible>(&self, name:&str) -> Option<T> {
		self.0.iter().find(|(stored_name, _)| stored_name == name).map(|(_, bytes)| T::from_bytes(bytes)).flatten()
	}
}



pub trait WindowRelativeProfileDataConvertible:Sized {

	/// Convert the data to bytes.
	fn to_bytes(self) -> Vec<u8>;

	/// Create the data from bytes.
	fn from_bytes(bytes:&[u8]) -> Option<Self>;
}
impl WindowRelativeProfileDataConvertible for bool {
	fn to_bytes(self) -> Vec<u8> {
		vec![if self { 1 } else { 0 }]
	}
	fn from_bytes(bytes:&[u8]) -> Option<Self> {
		if bytes.is_empty() {
			None
		} else {
			Some(bytes[0] != 0)
		}
	}
}
impl WindowRelativeProfileDataConvertible for String {
	fn to_bytes(self) -> Vec<u8> {
		self.as_bytes().to_vec()
	}
	fn from_bytes(bytes:&[u8]) -> Option<Self> {
		match String::from_utf8(bytes.to_vec()) {
			Ok(string) => Some(string),
			Err(_) => None
		}
	}
}
macro_rules! impl_num {
	($t:ty) => {
		impl WindowRelativeProfileDataConvertible for $t {
			fn to_bytes(self) -> Vec<u8> {
				self.to_be_bytes().to_vec()
			}
			fn from_bytes(bytes:&[u8]) -> Option<Self> {
				const SIZE:usize = std::mem::size_of::<$t>();
				if bytes.len() >= SIZE {
					let mut target_bytes:[u8; SIZE] = [0u8; SIZE];
					target_bytes.copy_from_slice(&bytes[..SIZE]);
					Some(<$t>::from_be_bytes(target_bytes))
				} else {
					None
				}
			}
		}
	};
}
impl_num!(u8);
impl_num!(u16);
impl_num!(u32);
impl_num!(u64);
impl_num!(u128);
impl_num!(i8);
impl_num!(i16);
impl_num!(i32);
impl_num!(i64);
impl_num!(i128);
impl_num!(f32);
impl_num!(f64);
impl<T:WindowRelativeProfileDataConvertible> WindowRelativeProfileDataConvertible for Vec<T> {
	fn to_bytes(self) -> Vec<u8> {
		[
			(self.len() as u16).to_be_bytes().to_vec(),
			self.into_iter().map(|data| data.to_bytes()).map(|bytes| vec![(bytes.len() as u16).to_be_bytes().to_vec(), bytes]).flatten().flatten().collect::<Vec<u8>>()
		].into_iter().flatten().collect()
	}
	fn from_bytes(bytes:&[u8]) -> Option<Self> {
		if bytes.len() < 2 {
			return None;
		}
		let amount:usize = u16::from_be_bytes(bytes[..2].try_into().unwrap()) as usize;

		let mut cursor:usize = 2;
		let mut output_list:Vec<T> = Vec::with_capacity(amount);
		for _index in 0..amount {
			let data_size:usize = u16::from_be_bytes(bytes[cursor..cursor + 2].try_into().unwrap()) as usize;
			cursor += 2;
			match T::from_bytes(&bytes[cursor..cursor + data_size]) {
				Some(data) => output_list.push(data),
				None => return None
			}
			cursor += data_size;
		}
		Some(output_list)
	}
}