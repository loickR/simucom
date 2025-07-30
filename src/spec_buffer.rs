use bytebuffer::ByteBuffer;

#[derive(Debug, Default, Clone)]
pub struct SpecBuffer {
    buf: ByteBuffer
}

impl SpecBuffer {

    pub fn new() -> Self {
        Self { buf: ByteBuffer::new() }
    }

    pub fn from_bytes(data : &[u8]) -> SpecBuffer {
        return SpecBuffer {
            buf : ByteBuffer::from_bytes(data)
        };
    }

    pub fn len(&self) -> usize {
        return self.buf.len();
    }

    pub fn read_u8(&mut self) -> u8 {
        return self.buf.read_u8().unwrap();
    }

    pub fn read_u16(&mut self) -> u16 {
        return self.buf.read_u16().unwrap();
    }

    pub fn read_u32(&mut self) -> u32 {
        return self.buf.read_u32().unwrap();
    }

    pub fn read_u64(&mut self) -> u64 {
        return self.buf.read_u64().unwrap();
    }

    pub fn read_i8(&mut self) -> i8 {
        return self.buf.read_i8().unwrap();
    }

    pub fn read_i16(&mut self) -> i16 {
        return self.buf.read_i16().unwrap();
    }

    pub fn read_i32(&mut self) -> i32 {
        return self.buf.read_i32().unwrap();
    }

    pub fn read_i64(&mut self) -> i64 {
        return self.buf.read_i64().unwrap();
    }

    pub fn read_string(&mut self) -> String {
        return self.buf.read_string().unwrap();
    }

    pub fn write_u8(&mut self, val : u8) {
        self.buf.write_u8(val);
    }

    pub fn write_u16(&mut self, val : u16) {
        self.buf.write_u16(val);
    }

    pub fn write_u32(&mut self, val : u32) {
        self.buf.write_u32(val);
    }

    pub fn write_u64(&mut self, val : u64) {
        self.buf.write_u64(val);
    }

    pub fn write_i8(&mut self, val : i8) {
        self.buf.write_i8(val);
    }

    pub fn write_i16(&mut self, val : i16) {
        self.buf.write_i16(val);
    }

    pub fn write_i32(&mut self, val : i32) {
        self.buf.write_i32(val);
    }

    pub fn write_i64(&mut self, val : i64) {
        self.buf.write_i64(val);
    }

    pub fn write_string(&mut self, val : String) {
        self.buf.write_string(&val);
    }

    pub fn as_bytes(&mut self) -> &[u8] {
        return self.buf.as_bytes();
    }
}
