pub trait Writable {
    fn string_length_size(&self) -> usize;      // only 2 or 4 valid
    fn pid_size(&self) -> usize;                 // only 4 or 8 valid
    fn use_structure_header(&self) -> bool;
    fn copy_new(&self) -> Box<dyn Writable>;

    fn write(&mut self, data: &[u8]);
    fn write_u8(&mut self, value: u8);
    fn write_u16_le(&mut self, value: u16);
    fn write_u32_le(&mut self, value: u32);
    fn write_u64_le(&mut self, value: u64);

    fn write_i8(&mut self, value: i8);
    fn write_i16_le(&mut self, value: i16);
    fn write_i32_le(&mut self, value: i32);
    fn write_i64_le(&mut self, value: i64);

    fn write_f32_le(&mut self, value: f32);
    fn write_f64_le(&mut self, value: f64);

    fn write_bool(&mut self, value: bool);

    fn bytes(&self) -> &[u8];
}
