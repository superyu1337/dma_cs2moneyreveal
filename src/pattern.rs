use std::io::Read;

use log::warn;
use memflow::{os::ModuleInfo, mem::MemoryView, types::Address};

fn pattern_to_bytes(pattern: String) -> Vec<Option<u8>> {
    pattern.split(' ')
        .fold(Vec::new(), |mut accum, str| {
            if str == "??" {
                accum.push(None);
            } else {
                let byte = u8::from_str_radix(str, 16).unwrap();
                accum.push(Some(byte));
            }

            accum
        })
}

pub fn pattern_scan(mem: &mut impl MemoryView, module: &ModuleInfo, sig: &str) -> anyhow::Result<Option<Address>> {
    let pattern_bytes = pattern_to_bytes(sig.to_owned());
    let mut cursor = mem.cursor();

    println!("Searching \"{}\" for pattern: \"{}\"", module.name, sig);

    for i in 0..module.size {
        let mut found = true;
        cursor.set_address(module.base + i);

        let mut buf = vec![0u8; pattern_bytes.len()];

        if cursor.read_exact(buf.as_mut_slice()).is_err() {
            warn!("Encountered read error while scanning for pattern");
        }

        for (idx, byte) in buf.iter().enumerate() {
            if let Some(pat_byte) = pattern_bytes[idx] {
                if *byte != pat_byte {
                    found = false;
                    break;
                }
            }
        }

        if found {
            return Ok(Some(module.base + i));
        }
    }

    Ok(None)
}