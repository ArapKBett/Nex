use std::collections::HashMap;
use std::fmt;
use anyhow::{Result, anyhow}; // for error handling

// Traits analogous to RVType and Writable/Readable should be defined elsewhere.
pub trait RVType: fmt::Display + fmt::Debug {
	fn write_to(&self, writable: &mut dyn Writable) -> Result<()>;
	fn copy(&self) -> Box<dyn RVType>;
	fn equals(&self, other: &dyn RVType) -> bool;
}

pub trait Writable {
	fn copy_new(&self) -> Box<dyn Writable>;
	fn write_uint32_le(&mut self, val: u32);
	fn write_bytes(&mut self, bytes: &[u8]);
	fn bytes(&self) -> &[u8];
}

pub trait Readable {
	fn read(&mut self, buf: &mut [u8]) -> Result<()>;
	// Add more as needed.
}

// HoldableObject trait, extends RVType and requires object_id method
pub trait HoldableObject: RVType {
	fn object_id(&self) -> &dyn RVType;
	fn copy_ref(&self) -> Box<dyn HoldableObject>;
	fn extract_from(&mut self, readable: &mut dyn Readable) -> Result<()>;
	fn deref(&self) -> &dyn HoldableObject;
}

// To store registered objects globally
lazy_static::lazy_static! {
	static ref ANY_OBJECT_HOLDER_OBJECTS: std::sync::Mutex<HashMap<String, Box<dyn HoldableObject>>> =
		std::sync::Mutex::new(HashMap::new());
}

// Register a HoldableObject by its object_id string representation
pub fn register_object_holder_type(obj: Box<dyn HoldableObject>) {
	let mut map = ANY_OBJECT_HOLDER_OBJECTS.lock().unwrap();
	// Assuming object_id can be converted to string
	let key = obj.object_id().to_string();
	map.insert(key, obj);
}

// AnyObjectHolder: Holds any HoldableObject
pub struct AnyObjectHolder {
	pub object: Box<dyn HoldableObject>,
}

impl AnyObjectHolder {
	pub fn new() -> Self {
		Self { object: Box::new(DummyHoldableObject::new()) } // Dummy placeholder
	}

	pub fn write_to(&self, writable: &mut dyn Writable) -> Result<()> {
		let mut content_writable = writable.copy_new();

		self.object.write_to(&mut *content_writable)?;

		let object_buffer = content_writable.bytes();
		let object_buffer_length = (object_buffer.len() + 4) as u32; // length + 4 bytes length prefix

		self.object.object_id().write_to(writable)?;
		writable.write_uint32_le(object_buffer_length);
		writable.write_bytes(object_buffer);

		Ok(())
	}

	pub fn extract_from(&mut self, readable: &mut dyn Readable) -> Result<()> {
		// Assume identifier is string-like and can be extracted similarly
		let mut identifier = String::new();
		// You must implement reading a String from Readable, placeholder here:
		// identifier = read_string(readable)?;
		// For now let's error out since this requires your string deserialization.

		// Example placeholder:
		return Err(anyhow!("extract_from not fully implemented, needs string extractor"));

		// Once identifier read:
		/*
		let map = ANY_OBJECT_HOLDER_OBJECTS.lock().unwrap();
		if let Some(proto) = map.get(&identifier) {
			let mut obj = proto.copy_ref();

			obj.extract_from(readable)?;

			// Downcast check or something analogous
			self.object = obj; // transfer ownership
			Ok(())
		} else {
			Err(anyhow!("Unknown AnyObjectHolder identifier: {}", identifier))
		}
		*/
	}

	pub fn copy(&self) -> Self {
		Self {
			object: self.object.copy_ref(),
		}
	}

	pub fn equals(&self, other: &AnyObjectHolder) -> bool {
		self.object.equals(&*other.object)
	}
}

impl fmt::Display for AnyObjectHolder {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// You can expand this as needed
		write!(f, "AnyObjectHolder {{ Identifier: {}, Object: {} }}",
			self.object.object_id(),
			self.object)
	}
}

// DummyHoldableObject to satisfy the new() call, replace with real implementation
struct DummyHoldableObject {}
impl DummyHoldableObject {
	fn new() -> Self { DummyHoldableObject {} }
}
impl RVType for DummyHoldableObject {
	fn write_to(&self, _writable: &mut dyn Writable) -> Result<()> { Ok(()) }
	fn copy(&self) -> Box<dyn RVType> { Box::new(DummyHoldableObject::new()) }
	fn equals(&self, _other: &dyn RVType) -> bool { true }
}
impl HoldableObject for DummyHoldableObject {
	fn object_id(&self) -> &dyn RVType { self }
	fn copy_ref(&self) -> Box<dyn HoldableObject> { Box::new(DummyHoldableObject::new()) }
	fn extract_from(&mut self, _readable: &mut dyn Readable) -> Result<()> { Ok(()) }
	fn deref(&self) -> &dyn HoldableObject { self }
}
// Implement fmt for DummyHoldableObject for Display & Debug
impl fmt::Display for DummyHoldableObject {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "DummyHoldableObject") }
}
impl fmt::Debug for DummyHoldableObject {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "DummyHoldableObject") }
                     }
    
