use clap::{App, Arg};
use failure::{Error, ResultExt};
use std::io::Write;
use std::path::Path;

use failure::err_msg;

mod build;
mod constants;

fn main() -> Result<(), Error> {
    let version = format!(
        "{}.{}.{}{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH"),
        option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")
    );
    let matches = App::new("ontio-wasm-build")
        .about("does awesome things")
        .version(version.as_str())
        .arg(
            Arg::with_name("input")
                .index(1)
                .required(true)
                .help("Wasm file generated by rustc compiler"),
        )
        .arg(Arg::with_name("output").index(2).required(true).help("Output wasm file name"))
        .arg(
            Arg::with_name("keepcustom")
                .long("keep-custom")
                .help("Keep custom section in output wasm file"),
        )
        .get_matches();

    let input = matches.value_of("input").expect("required arg can not be None");
    let output = matches.value_of("output").expect("required arg can not be None");
    let keep_custom = matches.is_present("keepcustom");

    let module =
        parity_wasm::deserialize_file(input).context("could not deserialize input wasm file")?;

    let module = build::build(module, !keep_custom)?;
    let buf = parity_wasm::serialize(module)?;
    if buf.len() > constants::MAX_WASM_SIZE {
        return Err(err_msg("finial wasm file size exceed 512KB"));
    }
    let buf = match Path::new(output).extension() {
        Some(ext) if ext == "wat" || ext == "wast" => {
            let wat = wabt::wasm2wat(buf)?;
            wat.into_bytes()
        }
        _ => buf,
    };
    let mut io = ::std::fs::File::create(output)?;
    io.write_all(&buf)?;

    let output_hex = output.to_string() + ".str";
    let mut io = ::std::fs::File::create(output_hex)?;
    io.write_all(hex::encode(&buf).as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod test;
