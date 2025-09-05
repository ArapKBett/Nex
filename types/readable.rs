use anyhow::Result;

pub trait Readable {
    fn string_length_size(&self) -> usize;      // only 2 or 4 valid
    fn pid_size(&self) -> usize;                 // only 4 or 8 valid
    fn use_structure_header(&self) -> bool;
    fn remaining(&self) -> u64;
    fn read_remaining(&mut self) -> Result<Vec<u8>>;

    fn read(&mut self, length: u64) -> Result<Vec<u8>>;

    fn read_u8(&mut self) -> Result<u8>;
    fn read_u16_le(&mut self) -> Result<u16>;
    fn read_u32_le(&mut self) -> Result<u32>;
    fn read_u64_le(&mut self) -> Result<u64>;

    fn read_i8(&mut self) -> Result<i8>;
    fn read_i16_le(&mut self) -> Result<i16>;
    fn read_i32_le(&mut self) -> Result<i32>;
    fn read_i64_le(&mut self) -> Result<i64>;

    fn read_f32_le(&mut self) -> Result<f32>;
    fn read_f64_le(&mut self) -> Result<f64>;

    fn read_bool(&mut self) -> Result<bool>;
      }
