use std::io::{Read, Result, Write};

// Aug 18: QUESTIONS:
// Why exactly are we using phantom data here? Is it becasue we want to 'act' like we have ownership?
// Why exactly is the OS doing reads and writes in chunks? It is to be more efficient right? How does that translate counting bytes?
// How does this 'get between' the reader / writer to log? When and how does that work?
// What is `get_ref` supposed to do?

// Understand how I/O works. Readers and writers. Buffers, why they need to exsit.
// What is a chunked write vs one byte at a time.
// How does pointer allignment happen?

// OTHER: How is this macro doing it's thing? Why isn't a generic being used?

// NEXT:
// Figure out how the chunking works. Why are there two extra reads (or writes?)

pub struct ReadStats<R> {
    wrapped: R,
    reads: usize,
    bytes: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            reads: 0,
            bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    // Number of bytes read
    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    // Read operations
    pub fn reads(&self) -> usize {
        self.reads
    }
}

// What does it mean to implement 'read' for a ' ReadStats<R>? Like a `ReadStats<Vec<u8>>`?

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        self.wrapped.read(buf).map(|bytes| {
            self.bytes += bytes;
            bytes
        })
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    writes: usize,
    bytes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            writes: 0,
            bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        self.wrapped.write(buf).map(|bytes| {
            self.bytes += bytes;
            bytes
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
