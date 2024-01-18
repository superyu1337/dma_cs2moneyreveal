use memflow::{mem::MemoryView, types::Address, os::ModuleInfo};

use crate::pattern::pattern_scan;

const BUF_SIZE: usize = 3;

/// Small helper to display the buffers in clean manner
fn display_buf(buf: &[u8; BUF_SIZE]) -> String {
    let mut string = String::new();

    for b in buf {
        string.push_str(&format!("0x{:0X}/", *b));
    }

    string
}

/// Patches the function
pub fn patch(mem: &mut impl MemoryView, location: Address) -> anyhow::Result<[u8; BUF_SIZE]> {
    let mut original_buf = [0u8; BUF_SIZE];
    mem.read_into(location, &mut original_buf)?;

    let new_buf: [u8; BUF_SIZE] = [
        0xB0, 0x01,     // MOV AL,1
        0xC3            // RET
    ];

    println!("Patching /{} to /{}", display_buf(&original_buf), display_buf(&new_buf));

    mem.write(
        location, 
        &new_buf
    )?;

    Ok(original_buf)
}

/// Restores the function back to the original
pub fn restore(mem: &mut impl MemoryView, location: Address, original: [u8; BUF_SIZE]) -> anyhow::Result<()> {
    println!("\nRestoring to /{}", display_buf(&original));

    mem.write(location, &original)?;
    Ok(())
}

/// Gets the address of the function to patch
/// Tries to scan for the regular pattern first.
/// If that fails, it tries to scan for a fallback that should match if this program didn't restore the function.
pub fn get_function(mem: &mut impl MemoryView, module: &ModuleInfo) -> anyhow::Result<Address> {
    let mut is_hltv = pattern_scan(
        mem,
        module,
        "48 83 EC 28 48 8B 0D ?? ?? ?? ?? 48 8B 01 FF 90 ?? ?? ?? ?? 84 C0 75 0D"
    )?;

    if is_hltv.is_none() {
        is_hltv = pattern_scan(
            mem,
            module,
            "B0 01 C3 28 48 8B 0D ?? ?? ?? ?? 48 8B 01 FF 90 ?? ?? ?? ?? 84 C0 75 0D"
        )?;
    }

    match is_hltv {
        Some(addr) => Ok(addr),
        None => Err(anyhow::anyhow!("Failed to find function")),
    }
}