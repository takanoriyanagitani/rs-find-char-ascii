use std::process::ExitCode;

use std::io;

use std::io::BufWriter;
use std::io::Write;

use rs_find_char_ascii::bind;
use rs_find_char_ascii::lift;

#[cfg(not(target_os = "wasi"))]
use rs_find_char_ascii::find::simple::FindConfig;

fn env_val_by_key(key: &'static str) -> impl FnMut() -> Result<String, io::Error> {
    move || {
        std::env::var(key).map_err(|e| {
            io::Error::other(format!(
                "can not get an environment variable (key={key}) : {e}"
            ))
        })
    }
}

fn string2char(i: String) -> Result<u8, io::Error> {
    let s: &[u8] = i.as_bytes();
    s.first()
        .copied()
        .ok_or_else(|| io::Error::other("empty string got"))
}

fn ch_needle() -> Result<u8, io::Error> {
    bind!(
        env_val_by_key("ENV_CHAR_TO_FIND_NEEDLE"),
        lift!(string2char)
    )()
}

#[cfg(not(target_os = "wasi"))]
fn find_cfg() -> Result<FindConfig, io::Error> {
    bind!(ch_needle, lift!(|ch: u8| Ok(FindConfig { ch })))()
}

#[cfg(target_os = "wasi")]
use rs_find_char_ascii::find::wasm::simd;

#[cfg(target_os = "wasi")]
fn find_cfg() -> Result<simd::FindConfig, io::Error> {
    bind!(ch_needle, lift!(|ch: u8| Ok(simd::FindConfig { ch })))()
}

fn filenames_from_arg() -> impl Iterator<Item = String> {
    std::env::args().skip(1)
}

#[cfg(not(target_os = "wasi"))]
fn filenames2names_with_char_std<I, W>(names: I, w: W) -> Result<(), io::Error>
where
    I: Iterator<Item = String>,
    W: FnMut(&str) -> Result<(), io::Error>,
{
    let cfg: FindConfig = find_cfg()?;
    cfg.write_filenames_with_char(names, w)
}

#[cfg(target_os = "wasi")]
fn filenames2names_with_char_wasi_simd<I, W>(names: I, w: W) -> Result<(), io::Error>
where
    I: Iterator<Item = String>,
    W: FnMut(&str) -> Result<(), io::Error>,
{
    let cfg: simd::FindConfig = find_cfg()?;
    cfg.write_filenames_with_char(names, w)
}

#[cfg(not(target_os = "wasi"))]
fn names2found2writer_std<I, W>(names: I, mut wtr: W) -> Result<(), io::Error>
where
    I: Iterator<Item = String>,
    W: Write,
{
    let w = |name_with_ch: &str| writeln!(&mut wtr, "{name_with_ch}");
    filenames2names_with_char_std(names, w)?;
    wtr.flush()
}

#[cfg(target_os = "wasi")]
fn names2found2writer_std<I, W>(names: I, mut wtr: W) -> Result<(), io::Error>
where
    I: Iterator<Item = String>,
    W: Write,
{
    let w = |name_with_ch: &str| writeln!(&mut wtr, "{name_with_ch}");
    filenames2names_with_char_wasi_simd(names, w)?;
    wtr.flush()
}

fn args2names2found2stdout_std() -> Result<(), io::Error> {
    let filenames = filenames_from_arg();

    let o = io::stdout();
    let mut ol = o.lock();

    let bw = BufWriter::new(&mut ol);
    names2found2writer_std(filenames, bw)?;

    ol.flush()
}

fn args2names2found2stdout() -> Result<(), io::Error> {
    args2names2found2stdout_std()
}

fn main() -> ExitCode {
    args2names2found2stdout()
        .map(|_| ExitCode::SUCCESS)
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            ExitCode::FAILURE
        })
}
