use std::io::Write;
use std::io;

struct BufferWriter {
    bytes: Vec<u8>
}

impl BufferWriter {
    fn new() -> BufferWriter {
        BufferWriter {
            bytes: Vec::new()
        }
    }
}

impl Write for BufferWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        let len = buf.len();
        self.bytes.extend_from_slice(buf);
        Ok(len)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

impl From<BufferWriter> for Vec<u8> {
    fn from(writer: BufferWriter) -> Vec<u8> {
        writer.bytes
    }
}

fn main() {
    let mut writer = BufferWriter::new();
    println!("Write result: {:?}", writer.write(&b"Hello"[..]));
    let buf = Vec::from(writer);
    println!("The bytes are: {:?}", buf);
}
