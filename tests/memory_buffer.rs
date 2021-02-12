use std::io::{self, Cursor, Read, Seek, Write};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryByteBuffer {
    data: Cursor<Vec<u8>>,
}

impl Seek for MemoryByteBuffer {
    fn seek(&mut self, pos: io::SeekFrom) -> Result<u64, io::Error> {
        self.data.seek(pos)
    }
}

impl Read for MemoryByteBuffer {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        self.data.read(buf)
    }
}

impl Write for MemoryByteBuffer {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.data.write(buf)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.data.flush()
    }
}

impl MemoryByteBuffer {
    pub fn new() -> Self {
        Self {
            data: Cursor::new(Vec::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let mut buffer = MemoryByteBuffer::new();
        let mut bytes: Vec<u8> = Vec::new();
        buffer.read(&mut bytes).unwrap();
        assert_eq!(buffer.data.position(), 0);
        let empty: Vec<u8> = vec![];
        assert_eq!(bytes, empty);
    }

    #[test]
    fn write_should_write_1_byte() {
        let mut buffer = MemoryByteBuffer::new();
        let n = buffer.write(&[0]).unwrap();
        assert_eq!(buffer.data.position(), 1);
        assert_eq!(n, 1);
    }

    #[test]
    fn flush_should_not_clear_state() {
        let mut buffer = MemoryByteBuffer::new();
        buffer.write(&[0, 1, 2]).unwrap();
        let clone = buffer.clone();
        buffer.flush().unwrap();
        assert_eq!(buffer, clone);
    }

    #[test]
    fn read_should_read_0_bytes() {
        let mut buffer = MemoryByteBuffer::new();
        let mut vec = Vec::new();
        let n = buffer.read(&mut vec).unwrap();
        assert_eq!(buffer.data.position(), 0);
        assert_eq!(n, 0);
    }
    #[test]
    fn read_should_read_4_bytes() {
        let mut buffer = MemoryByteBuffer::new();
        buffer.write(&[0, 1, 2, 3]).unwrap();
        buffer.seek(io::SeekFrom::Start(0)).unwrap();
        let mut vec = vec![0; 15];
        let n = buffer.read(&mut vec).unwrap();
        assert_eq!(n, 4);
        assert_eq!(buffer.data.position(), 4);
    }
}
