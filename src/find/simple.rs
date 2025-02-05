use std::io;

use std::io::ErrorKind;

use std::io::Read;

use std::fs::File;

pub fn contains_char_ascii(ch: u8, s: &[u8]) -> bool {
    s.contains(&ch)
}

pub fn read_contains_ascii<R>(ch_needle: u8, mut rdr: R) -> Result<bool, io::Error>
where
    R: Read,
{
    let mut sbuf: [u8; 65536] = [0; 65536];
    let buflen: usize = sbuf.len();

    loop {
        let mut ms: &mut [u8] = &mut sbuf[..];
        let mut taken = rdr.by_ref().take(buflen as u64);
        let cnt: u64 = io::copy(&mut taken, &mut ms).or_else(|e| match e.kind() {
            ErrorKind::WriteZero => Ok(65536),
            _ => Err(e),
        })?;
        if 0 == cnt {
            return Ok(false);
        }

        let sread: &[u8] = &sbuf[..(cnt as usize)];
        let found: bool = contains_char_ascii(ch_needle, sread);
        if found {
            return Ok(true);
        }
    }
}

pub fn read_contains_ascii_not_worked<R>(
    ch_needle: u8,
    buf: &mut Vec<u8>,
    mut rdr: R,
) -> Result<bool, io::Error>
where
    R: Read,
{
    loop {
        buf.clear();

        let mut taken = rdr.by_ref().take(buf.len() as u64);

        let cnt: usize = taken.read_to_end(buf)?;
        if 0 == cnt {
            return Ok(false);
        }

        let char_found: bool = contains_char_ascii(ch_needle, buf);
        if char_found {
            return Ok(true);
        }
    }
}

pub struct FindConfig {
    pub ch: u8,
}

impl FindConfig {
    pub fn file_contains_ascii(&self, filename: &str) -> Result<bool, io::Error> {
        match filename {
            "-" => read_contains_ascii(self.ch, io::stdin().lock()),
            _ => read_contains_ascii(self.ch, File::open(filename)?),
        }
    }

    pub fn write_filenames_with_char<I, W>(
        &self,
        filenames: I,
        mut writer: W,
    ) -> Result<(), io::Error>
    where
        I: Iterator<Item = String>,
        W: FnMut(&str) -> Result<(), io::Error>,
    {
        for filename in filenames {
            let found: bool = self.file_contains_ascii(&filename)?;
            if found {
                writer(filename.as_str())?;
            }
        }
        Ok(())
    }
}
