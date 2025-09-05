use std::any::Any;
use std::fmt;
use anyhow::Result;

pub trait RVType: fmt::Display + fmt::Debug {
    fn write_to(&self, writable: &mut dyn Writable) -> Result<()>;
    fn extract_from(&mut self, readable: &mut dyn Readable) -> Result<()>;
    fn copy(&self) -> Box<dyn RVType>;
    fn equals(&self, other: &dyn RVType) -> bool;
    fn copy_ref(&self) -> Box<dyn RVType>;
    fn deref(&self) -> Box<dyn RVType>;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bool(pub bool);

impl Bool {
    pub fn new(input: bool) -> Self {
        Self(input)
    }
}

impl RVType for Bool {
    fn write_to(&self, writable: &mut dyn Writable) -> Result<()> {
        writable.write_bool(self.0);
        Ok(())
    }

    fn extract_from(&mut self, readable: &mut dyn Readable) -> Result<()> {
        let value = readable.read_bool()?;
        self.0 = value;
        Ok(())
    }

    fn copy(&self) -> Box<dyn RVType> {
        Box::new(*self)
    }

    fn equals(&self, other: &dyn RVType) -> bool {
        other.as_any().downcast_ref::<Bool>().map_or(false, |o| self == o)
    }

    fn copy_ref(&self) -> Box<dyn RVType> {
        self.copy()
    }

    fn deref(&self) -> Box<dyn RVType> {
        self.copy()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
