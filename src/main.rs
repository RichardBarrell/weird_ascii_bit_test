use std::io::{Result, Write};

fn main() {
    let bits = make_bits();
    output_bits_packed(&bits).expect("Error writing packed file");
    println!("Wrote packed file");
    output_bits_ascii_with_newlines(&bits, 8).expect("Error writing ASCII file");
    println!("Wrote ASCII file (1 line per byte)");
    output_bits_ascii_with_newlines(&bits, 72 * 8).expect("Error writing ASCII file");
    println!("Wrote ASCII file (1 line per 72 bytes)");
    output_bits_ascii_no_newlines(&bits).expect("Error writing ASCII file");
    println!("Wrote ASCII file (oops all bytes)");
}

fn make_bits() -> Vec<u8> {
    let mut bits: Vec<u8> = Vec::new();
    for i in 0..1024 {
        let n = (i % 67) as u8;
        for shift_by in 0..7 {
            let bit = (n >> shift_by) & 1;
            bits.push(bit);
        }
    }
    return bits;
}

fn output_bits_packed(bits: &[u8]) -> std::io::Result<()> {
    let mut fd = std::fs::File::create("./output.packed")?;
    let mut byte: u8 = 0;
    let mut bit_index = 0;
    let mut output_bytes: [u8; 4096] = [0; 4096];
    let mut byte_index = 0;
    let mut output_byte = |byte: u8| -> std::io::Result<()> {
        output_bytes[byte_index] = byte;
        byte_index += 1;
        if byte_index == 4096 {
            fd.write(&output_bytes)?;
            byte_index = 0;
        }
        return Result::Ok(());
    };
    for bit in bits {
        byte = (byte << 1) + bit;
        bit_index += 1;
        if bit_index == 8 {
            output_byte(byte)?;
            byte = 0;
            bit_index = 0;
        }
    }
    if bit_index != 0 {
        while bit_index < 8 {
            byte = byte << 1;
            bit_index += 1;
        }
        output_byte(byte)?;
    }
    drop(output_byte);
    if byte_index != 0 {
        fd.write(&output_bytes[0..byte_index])?;
    }
    return Result::Ok(());
}

fn output_bits_ascii_with_newlines(bits: &[u8], bits_per_line: u32) -> std::io::Result<()> {
    let path = format!("./output.ascii_{}", bits_per_line);
    let mut fd = std::fs::File::create(path)?;
    let mut output_bytes: [u8; 4096] = [0; 4096];
    let mut byte_index = 0;
    let mut output_byte = |byte: u8| -> std::io::Result<()> {
        output_bytes[byte_index] = byte;
        byte_index += 1;
        if byte_index == 4096 {
            fd.write(&output_bytes)?;
            byte_index = 0;
        }
        return Result::Ok(());
    };
    let mut line_index = 0;
    let mut output_bit = |bit: u8| -> std::io::Result<()> {
        output_byte(('0' as u8) + bit)?;
        line_index += 1;
        if line_index == bits_per_line {
            output_byte('\n' as u8)?;
            line_index = 0;
        }
        return Result::Ok(());
    };
    for bit in bits {
        output_bit(*bit)?;
    }
    drop(output_bit);
    if line_index > 0 {
        output_byte('\n' as u8)?;
    }
    drop(output_byte);
    if byte_index != 0 {
        fd.write(&output_bytes[0..byte_index])?;
    }
    return Result::Ok(());
}

fn output_bits_ascii_no_newlines(bits: &[u8]) -> std::io::Result<()> {
    let mut fd = std::fs::File::create("./output.ascii")?;
    let mut output_bytes: [u8; 4096] = [0; 4096];
    let mut byte_index = 0;
    let mut output_byte = |byte: u8| -> std::io::Result<()> {
        output_bytes[byte_index] = byte;
        byte_index += 1;
        if byte_index == 4096 {
            fd.write(&output_bytes)?;
            byte_index = 0;
        }
        return Result::Ok(());
    };
    let mut output_bit = |bit: u8| -> std::io::Result<()> {
        output_byte(('0' as u8) + bit)?;
        return Result::Ok(());
    };
    for bit in bits {
        output_bit(*bit)?;
    }
    drop(output_bit);
    drop(output_byte);
    if byte_index != 0 {
        fd.write(&output_bytes[0..byte_index])?;
    }
    return Result::Ok(());
}
