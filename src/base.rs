//
// Exponential-Golomb conding: https://en.wikipedia.org/wiki/Exponential-Golomb_coding
//

use std::io;

use bitstream_io::BitRead;

pub fn ebsp_to_rbsp(ebsp: &[u8]) -> Vec<u8> {
    let mut rbsp: Vec<u8> = Vec::with_capacity(ebsp.len());

    let mut i = 0;
    while i < ebsp.len() {
        if i + 2 < ebsp.len() && ebsp[i] == 0 && ebsp[i + 1] == 0 && ebsp[i + 2] == 3 {
            rbsp.push(0);
            rbsp.push(0);
            i += 3;
        } else {
            rbsp.push(ebsp[i]);
            i += 1;
        }
    }
    rbsp
}

/// Parses an unsigned 0-th order Exp-Golomb code.
///
/// See _9.2 Parsing process for 0-th order Exp-Golomb codes_ in the H.265/HEVC spec.
pub fn read_exp_golomb_ue<R: BitRead>(reader: &mut R) -> Result<u32, io::Error> {
    let leading_zero_count = reader.read_unary1()?;
    Ok((1 << leading_zero_count) - 1 + reader.read::<u32>(leading_zero_count)?)
}

/// Parses an unsigned 0-th order Exp-Golomb code.
///
/// See _9.2 Parsing process for 0-th order Exp-Golomb codes_ in the H.265/HEVC spec.
pub fn read_exp_golomb_ue_count_bits<R: BitRead>(
    reader: &mut R,
    bit_count: &mut u32,
) -> Result<u32, io::Error> {
    let leading_zero_count = reader.read_unary1()?;
    let value = (1 << leading_zero_count) - 1 + reader.read::<u32>(leading_zero_count)?;
    *bit_count += leading_zero_count + 1 + leading_zero_count;
    Ok(value)
}

/// Parses a signed 0-th order Exp-Golomb code.
///
/// See _9.2 Parsing process for 0-th order Exp-Golomb codes_ in the H.265/HEVC spec.
pub fn read_exp_golomb_se<R: BitRead>(reader: &mut R) -> Result<i32, io::Error> {
    let code_num = read_exp_golomb_ue(reader)?;
    if code_num & 0b1 != 0 {
        Ok(((code_num >> 1) + 1) as i32)
    } else {
        Ok(-((code_num >> 1) as i32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use bitstream_io::{BigEndian, BitReader, BitWrite, BitWriter};

    #[test]
    fn exp_golomb_works() {
        assert_eq!(de_exp_golomb_ue(31, 0b1), 0);
        assert_eq!(de_exp_golomb_ue(29, 0b010), 1);
        assert_eq!(de_exp_golomb_ue(29, 0b011), 2);
        assert_eq!(de_exp_golomb_ue(27, 0b00100), 3);
        assert_eq!(de_exp_golomb_ue(27, 0b00101), 4);
        assert_eq!(de_exp_golomb_ue(27, 0b00110), 5);
        assert_eq!(de_exp_golomb_ue(27, 0b00111), 6);
        assert_eq!(de_exp_golomb_ue(25, 0b0001000), 7);
        assert_eq!(de_exp_golomb_ue(25, 0b0001001), 8);
        assert_eq!(de_exp_golomb_ue(25, 0b0001010), 9);
        assert_eq!(de_exp_golomb_ue(19, 0b0000001000100), 67);
        assert_eq!(de_exp_golomb_ue(19, 0b0000001101011), 106);

        assert_eq!(en_exp_golomb_ue(3), (5, 0b00100));
        assert_eq!(en_exp_golomb_ue(4), (5, 0b00101));
        assert_eq!(en_exp_golomb_ue(5), (5, 0b00110));
        assert_eq!(en_exp_golomb_ue(6), (5, 0b00111));
        assert_eq!(en_exp_golomb_ue(67), (13, 0b0000001000100));
        assert_eq!(en_exp_golomb_ue(106), (13, 0b0000001101011));

        assert_eq!(de_exp_golomb_se(31, 0b1), 0);
        assert_eq!(de_exp_golomb_se(29, 0b010), 1);
        assert_eq!(de_exp_golomb_se(29, 0b011), -1);
        assert_eq!(de_exp_golomb_se(27, 0b00100), 2);
        assert_eq!(de_exp_golomb_se(27, 0b00101), -2);
        assert_eq!(de_exp_golomb_se(27, 0b00110), 3);
        assert_eq!(de_exp_golomb_se(27, 0b00111), -3);
    }

    fn de_exp_golomb_ue(skip: u8, seq: u32) -> u32 {
        let mut reader = io::Cursor::new(seq.to_be_bytes());
        let mut bit_reader = BitReader::endian(&mut reader, BigEndian);

        bit_reader
            .seek_bits(io::SeekFrom::Current(skip as i64))
            .unwrap();

        let leading_zero_count = bit_reader.read_unary1().unwrap();
        (1 << leading_zero_count) - 1 + bit_reader.read::<u32>(leading_zero_count).unwrap()
    }

    fn de_exp_golomb_se(skip: u8, seq: u32) -> i32 {
        let code_num = de_exp_golomb_ue(skip, seq);

        if code_num & 0b1 != 0 {
            ((code_num >> 1) + 1) as i32
        } else {
            -((code_num >> 1) as i32)
        }
    }

    fn en_exp_golomb_ue(code_num: u32) -> (u32, u32) {
        let mut bytes: [u8; 4] = [0; 4];
        let mut bit_writer = BitWriter::endian(&mut bytes[..], BigEndian);

        let code_num_plus1 = code_num + 1;
        let num_bits_code_num_plus1 = 32 - code_num_plus1.leading_zeros();
        let leading_zero_count = num_bits_code_num_plus1 - 1;

        let bits_used = leading_zero_count + num_bits_code_num_plus1;

        for _ in 0..32 - bits_used {
            bit_writer.write_bit(false).unwrap();
        }

        for _ in 0..leading_zero_count {
            bit_writer.write_bit(false).unwrap();
        }
        bit_writer
            .write(num_bits_code_num_plus1, code_num_plus1)
            .unwrap();

        let coded = u32::from_be_bytes(bytes);

        (bits_used, coded)
    }
}
