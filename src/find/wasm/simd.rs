use std::arch::wasm32;

use std::arch::wasm32::v128;

use std::io;

use std::io::ErrorKind;

use std::io::Read;

use std::fs::File;

pub fn contains_char_ascii_nosimd(ch: u8, s: &[u8]) -> bool {
    s.contains(&ch)
}

pub fn contains_char_ascii_simd(ch: u8, s: &[u8], ch_vec: v128) -> bool {
    let mut chunks = s.chunks_exact(16);
    let mut buf: [u8; 16] = [0; 16];
    for chnk in chunks.by_ref() {
        let sz: usize = chnk.len();
        if 16 != sz {
            eprintln!("unexpected size: {sz}");
            return false;
        }
        buf.copy_from_slice(chnk);

        let u: u128 = u128::from_be_bytes(buf);
        let hi: u64 = (u >> 64) as u64;
        let lo: u64 = (u & 0xffff_ffff_ffff_ffff) as u64;
        let v: v128 = wasm32::u64x2(hi, lo);

        // e.g, 00ff-0000 0000-0000 0000-0000 0000-0000
        let bools: v128 = wasm32::u8x16_eq(v, ch_vec);

        // e.g, ff00-ffff ffff-ffff ffff-ffff ffff-ffff
        let not: v128 = wasm32::v128_not(bools);

        // e.g, false
        let all_true: bool = wasm32::u8x16_all_true(not);

        // e.g, true
        let found: bool = !all_true;

        if found {
            return true;
        }
    }

    let remainder: &[u8] = chunks.remainder();
    contains_char_ascii_nosimd(ch, remainder)
}

pub fn contains_char_ascii(ch: u8, s: &[u8], ch_vec: v128) -> bool {
    contains_char_ascii_simd(ch, s, ch_vec)
}

pub fn read_contains_ascii<R>(ch_needle: u8, mut rdr: R) -> Result<bool, io::Error>
where
    R: Read,
{
    let mut sbuf: [u8; 65536] = [0; 65536];
    let buflen: usize = sbuf.len();

    let ch_vec: v128 = wasm32::u8x16_splat(ch_needle);

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
        let found: bool = contains_char_ascii(ch_needle, sread, ch_vec);
        if found {
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
