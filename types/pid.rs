use std::fmt;
use anyhow::Result;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PID(pub u64);

impl PID {
    pub fn new(input: u64) -> Self {
        PID(input)
    }
}

impl RVType for PID {
    fn write_to(&self, writable: &mut dyn Writable) -> Result<()> {
        if writable.pid_size() == 8 {
            writable.write_u64_le(self.0);
        } else {
            writable.write_u32_le(self.0 as u32);
        }
        Ok(())
    }

    fn extract_from(&mut self, readable: &mut dyn Readable) -> Result<()> {
        if readable.pid_size() == 8 {
            self.0 = readable.read_u64_le()?;
        } else {
            self.0 = readable.read_u32_le()? as u64;
        }
        Ok(())
    }

    fn copy(&self) -> Box<dyn RVType> {
        Box::new(*self)
    }

    fn equals(&self, other: &dyn RVType) -> bool {
        other.as_any().downcast_ref::<PID>().map_or(false, |o| self == o)
    }

    fn copy_ref(&self) -> Box<dyn RVType> {
        self.copy()
    }

    fn deref(&self) -> Box<dyn RVType> {
        self.copy()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for PID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PID{{\n\tpid: {}\n}}", self.0)
    }
  }
