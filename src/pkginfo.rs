use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use fehler::throws;
use libflate::gzip;
use tar::Archive;
use xz2::read::XzDecoder;

use crate::db::models::Compression;
use crate::error::Error;

#[throws]
pub fn load_pkginfo(compression: Compression, package_file: &str) -> HashMap<String, Vec<String>> {
    let package_path = PathBuf::new()
        .join("packages")
        .join(package_file);
    let compressed_reader = std::fs::File::open(package_path)?;
    let decompressed_reader = decompress(compression, compressed_reader)?;
    extract_pkginfo(decompressed_reader)?
}

#[throws(io::Error)]
fn decompress(compression: Compression, reader: impl Read + 'static) -> Box<dyn Read + 'static> {
    use Compression::*;
    match compression {
        Lzma => Box::new(XzDecoder::new(reader)) as Box<dyn Read>,
        Zstd => Box::new(zstd::Decoder::new(reader)?) as Box<dyn Read>,
        Gzip => Box::new(gzip::Decoder::new(reader)?) as Box<dyn Read>
    }
}

#[throws]
fn extract_pkginfo(reader: impl Read) -> HashMap<String, Vec<String>> {
    let mut archive = Archive::new(reader);
    let pkginfo_entry = archive.entries()?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let path = entry.path().unwrap();
            path.as_os_str() == ".PKGINFO"
        })
        .next()
        .ok_or("Archive does not contain a .PKGINFO file")?;

    let mut contents = String::new();
    pkginfo_entry.take(100_000)
        .read_to_string(&mut contents)?;

    parse_pkginfo(contents)?
}

#[throws]
fn parse_pkginfo(pkginfo: String) -> HashMap<String, Vec<String>> {
    let properties = pkginfo.lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.starts_with("#") {
                None
            } else {
                let components: Vec<_> = line
                    .splitn(2, "=")
                    .collect();
                if components.len() == 2 {
                    Some(Ok((
                        components[0].trim().to_string(),
                        components[1].trim().to_string())))
                } else {
                    Some(Err(format!("Cannot parse line of package info: {}", line)))
                }
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut property_map = HashMap::<String, Vec<String>>::new();
    for (key, value) in properties {
        property_map.entry(key).or_default().push(value);
    }
    property_map
}
