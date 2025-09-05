use std::collections::HashMap;
use std::fmt;
use anyhow::{Result, anyhow};
use std::sync::Mutex;

// Traits RVType, Writable, Readable are assumed defined as above (or similarly)

pub trait HoldableObject: RVType {
    fn object_id(&self) -> &dyn RVType;
    fn copy_ref(&self) -> Box<dyn HoldableObject>;
    fn extract_from(&mut self, readable: &mut dyn Readable) -> Result<()>;
    fn deref(&self) -> &dyn HoldableObject;
}

// Global registry of HoldableObjects keyed by their object id string representation
lazy_static::lazy_static! {
    static ref ANY_OBJECT_HOLDER_OBJECTS: Mutex<HashMap<String, Box<dyn HoldableObject>>> = Mutex::new(HashMap::new());
}

pub fn register_object_holder_type(obj: Box<dyn HoldableObject>) {
    let mut map = ANY_OBJECT_HOLDER_OBJECTS.lock().unwrap();
    map.insert(obj.object_id().to_string(), obj);
}

pub struct AnyObjectHolder {
    pub object: Box<dyn HoldableObject>,
}

impl AnyObjectHolder {
    pub fn new() -> Self {
        // Placeholder: You must replace DummyHoldableObject with your real default
        Self { object: Box::new(DummyHoldableObject::new()) }
    }

    pub fn write_to(&self, writable: &mut dyn Writable) -> Result<()> {
        let mut content_writable = writable.copy_new();

        self.object.write_to(&mut *content_writable)?;

        let object_buffer = content_writable.bytes();
        let object_buffer_length = (object_buffer.len() + 4) as u32;

        self.object.object_id().write_to(writable)?;
        writable.write_u32_le(object_buffer_length);
        writable.write(object_buffer);

        Ok(())
    }

    pub fn extract_from(&mut self, readable: &mut dyn Readable) -> Result<()> {
        // Needs a function to read a string identifier from readable
        let identifier = read_string_from_readable(readable)?; // Implement this appropriately

        let _length = readable.read_u32_le()?; // Length field

        let _buffer_length = readable.read_u32_le()?; // Buffer length, can use for validation if desired

        let map = ANY_OBJECT_HOLDER_OBJECTS.lock().unwrap();

        if let Some(proto) = map.get(&identifier) {
            let mut obj = proto.copy_ref();
            obj.extract_from(readable)?;

            self.object = obj;
            Ok(())
        } else {
            Err(anyhow!("Unknown AnyObjectHolder identifier: {}", identifier))
        }
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
        write!(f, "AnyObjectHolder {{ Identifier: {}, Object: {} }}",
            self.object.object_id(),
            self.object)
    }
}

// DummyHoldableObject for placeholder use
struct DummyHoldableObject { }

impl DummyHoldableObject {
    fn new() -> Self { DummyHoldableObject {} }
}

impl RVType for DummyHoldableObject {
    fn write_to(&self, _writable: &mut dyn Writable) -> Result<()> { Ok(()) }
    fn extract_from(&mut self, _readable: &mut dyn Readable) -> Result<()> { Ok(()) }
    fn copy(&self) -> Box<dyn RVType> { Box::new(Self::new()) }
    fn equals(&self, _other: &dyn RVType) -> bool { true }
    fn copy_ref(&self) -> Box<dyn RVType> { self.copy() }
    fn deref(&self) -> Box<dyn RVType> { self.copy() }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

impl HoldableObject for DummyHoldableObject {
    fn object_id(&self) -> &dyn RVType { self }
    fn copy_ref(&self) -> Box<dyn HoldableObject> { Box::new(DummyHoldableObject::new()) }
    fn extract_from(&mut self, _readable: &mut dyn Readable) -> Result<()> { Ok(()) }
    fn deref(&self) -> &dyn HoldableObject { self }
}

impl fmt::Display for DummyHoldableObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DummyHoldableObject")
    }
}
impl fmt::Debug for DummyHoldableObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DummyHoldableObject")
    }
}

// Helper function you need to implement to extract a string identifier from a Readable
fn read_string_from_readable(_readable: &mut dyn Readable) -> Result<String> {
    // Example: read length prefix, then read UTF-8 string bytes and convert
    unimplemented!("Implement string reading from Readable trait");
		}
															 
