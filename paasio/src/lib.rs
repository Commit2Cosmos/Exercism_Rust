use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

                    // bytes_read, read_ops
pub struct ReadStats<R>(R, usize, usize);

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats(wrapped, Default::default(), Default::default())
    }

    pub fn get_ref(&self) -> &R {
        &self.0
    }

    pub fn bytes_through(&self) -> usize {
        self.1
    }

    pub fn reads(&self) -> usize {
        self.2
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = self.0.read(buf)?;
        self.1 += bytes;
        self.2 += 1;
        Ok(bytes)
    }
}

pub struct WriteStats<W>(W, usize, usize);

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats(wrapped, Default::default(), Default::default())
    }

    pub fn get_ref(&self) -> &W {
        &self.0
    }

    pub fn bytes_through(&self) -> usize {
        self.1
    }

    pub fn writes(&self) -> usize {
        self.2
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes = self.0.write(buf)?;
        self.1 += bytes;
        self.2 += 1;
        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}
