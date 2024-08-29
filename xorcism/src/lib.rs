use std::{borrow::Borrow, io::{Read, Write}};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    //* for state */
    index: usize
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key>(key: &'a Key) -> Xorcism<'a>
    where Key: AsRef<[u8]> + ?Sized
    {
        Self { key: key.as_ref() , index: Default::default()}
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        let key = Xorcism::get_key(self.key, self.index);

        data.iter_mut().zip(key).for_each(|(d, k)| *d ^= k);

        self.index = (self.index + data.len()) % self.key.len()
    }

    fn get_key(key: &[u8], x: usize) -> impl Iterator<Item = u8> + '_ {
        key.iter().copied().cycle().skip(x)
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<'k, Data>(&'k mut self, data: Data) -> impl Iterator<Item = u8> + '_
    where 
        Data: IntoIterator<Item: Borrow<u8>> + 'k,
    {
        let key = Xorcism::get_key(self.key, self.index);

        data.into_iter().zip(key).map(|(d, k)| {
            self.index = (self.index + 1) % self.key.len();
            d.borrow() ^ k
        })
    }

    pub fn reader(self, r: impl Read + 'a) -> impl Read + 'a {
        XorcismIO {
            xor: self,
            io: r,
        }
    }

    pub fn writer(self, w: impl Write + 'a) -> impl Write + 'a {
        XorcismIO {
            xor: self,
            io: w,
        }
    }
}


struct XorcismIO<'a, IO> {
    xor: Xorcism<'a>,
    io: IO
}

impl<R: Read> Read for XorcismIO<'_, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_read = self.io.read(buf)?;
        self.xor.munge_in_place(buf);
        Ok(bytes_read)
    }
}

impl<W: Write> Write for XorcismIO<'_, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut bytes_written = 0;

        for m in self.xor.munge(buf) {
            bytes_written += self.io.write(&[m])?;
        };

        Ok(bytes_written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.io.flush()
    }
}