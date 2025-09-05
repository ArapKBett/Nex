use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use anyhow::{Result, anyhow};

// Assumes RVType trait as defined previously with methods:
// write_to, extract_from, copy, equals, copy_ref, etc.

#[derive(Clone, Debug)]
pub struct Map<K, V> {
    inner: HashMap<K, V>,
}

impl<K, V> Map<K, V>
where
    K: Eq + Hash + Clone + fmt::Display,
    V: RVType + Clone,
{
    pub fn new() -> Self {
        Map {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.inner.insert(k, v);
    }
    
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<K, V> RVType for Map<K, V>
where
    K: RVType + Eq + Hash + Clone + fmt::Display + 'static,
    V: RVType + Clone + 'static,
{
    fn write_to(&self, writable: &mut dyn Writable) -> Result<()> {
        writable.write_u32_le(self.inner.len() as u32);

        for (key, value) in &self.inner {
            key.write_to(writable)?;
            value.write_to(writable)?;
        }

        Ok(())
    }

    fn extract_from(&mut self, readable: &mut dyn Readable) -> Result<()> {
        let length = readable.read_u32_le()? as usize;
        let mut extracted = HashMap::with_capacity(length);

        for _ in 0..length {
            // Create empty keys and values. This requires K and V to implement Default
            let mut key: K = Default::default();
            let mut value: V = Default::default();

            key.extract_from(readable)?;
            value.extract_from(readable)?;

            extracted.insert(key, value);
        }

        self.inner = extracted;
        Ok(())
    }

    fn copy(&self) -> Box<dyn RVType> {
        Box::new(self.clone())
    }

    fn equals(&self, other: &dyn RVType) -> bool {
        other.as_any().downcast_ref::<Map<K, V>>()
            .map_or(false, |o| {
                if self.inner.len() != o.inner.len() {
                    return false;
                }
                for (k, v) in &self.inner {
                    match o.inner.get(k) {
                        Some(ov) if v.equals(ov.as_ref()) => continue,
                        _ => return false,
                    }
                }
                true
            })
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

impl<K, V> fmt::Display for Map<K, V>
where
    K: fmt::Display + Eq + Hash,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.inner.is_empty() {
            write!(f, "{{}}")
        } else {
            write!(f, "{{\n")?;
            for (k, v) in &self.inner {
                write!(f, "\t{}: {},\n", k, v)?;
            }
            write!(f, "}}")
        }
    }
}

// Require Default for extract_from key/value instantiation
impl Default for String {
    fn default() -> Self {
        "".to_string()
    }
}
impl Default for u16 {
    fn default() -> Self {
        0u16
    }
  }
      
