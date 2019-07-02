use crate::{
    alloc::{Allocator, GlobalAllocator},
    containers::Array,
    hash::Adler32,
    io::{BitReader, Read},
};

#[derive(Debug)]
pub enum InflateError
{
    InvalidStructure,
    UnsupportedMethod,
    InvalidWindowSize,
    DictNotSupported,
    CorruptedHeader,
    CorruptedData,
    CorruptedBlockHeader,
    ReadPastWindowSize,
}

const EXTRA_LENGTHS: [(usize, usize); 29] = [
    (0, 3),
    (0, 4),
    (0, 5),
    (0, 6),
    (0, 7),
    (0, 8),
    (0, 9),
    (0, 10),
    (1, 11),
    (1, 13),
    (1, 15),
    (1, 17),
    (2, 19),
    (2, 23),
    (2, 27),
    (2, 31),
    (3, 35),
    (3, 43),
    (3, 51),
    (3, 59),
    (4, 67),
    (4, 83),
    (4, 99),
    (4, 115),
    (5, 131),
    (5, 163),
    (5, 195),
    (5, 227),
    (0, 258),
];

const EXTRA_DIST: [(usize, usize); 30] = [
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (1, 5),
    (1, 7),
    (2, 9),
    (2, 13),
    (3, 17),
    (3, 25),
    (4, 33),
    (4, 49),
    (5, 65),
    (5, 97),
    (6, 129),
    (6, 193),
    (7, 257),
    (7, 385),
    (8, 513),
    (8, 769),
    (9, 1025),
    (9, 1537),
    (10, 2049),
    (10, 3073),
    (11, 4097),
    (11, 6145),
    (12, 8193),
    (12, 12289),
    (13, 16385),
    (13, 24577),
];

#[inline]
pub fn inflate(input: &[u8]) -> Result<Array<u8>, InflateError>
{
    inflate_with(input, GlobalAllocator)
}

pub fn inflate_with<A: Allocator>(input: &[u8], alloc: A) -> Result<Array<u8, A>, InflateError>
{
    let mut output = Array::new_with(alloc);

    if input.len() < 6
    {
        return Err(InflateError::InvalidStructure);
    }

    let cmf = input[0];
    let flg = input[1];

    let cm = cmf & 0b0000_1111;
    let cinfo = (cmf & 0b1111_0000) >> 4;

    if cm != 8
    {
        return Err(InflateError::UnsupportedMethod);
    }

    if cinfo > 7
    {
        return Err(InflateError::InvalidWindowSize);
    }

    let fdict = (flg & 0b0010_0000) != 0;
    if fdict
    {
        return Err(InflateError::DictNotSupported);
    }

    if (cmf as u16 * 256u16 + flg as u16) % 31 != 0
    {
        return Err(InflateError::CorruptedHeader);
    }

    let checksum_slice = &input[input.len() - 4..];
    let checksum_bytes = [
        checksum_slice[0],
        checksum_slice[1],
        checksum_slice[2],
        checksum_slice[3],
    ];
    let checksum_input = u32::from_be_bytes(checksum_bytes);

    let window_size = 1 << (cinfo as u32 + 8);

    let mut input = &input[2..input.len() - 4];
    let mut codes = Array::<HuffmanCode>::new();
    let mut clen_codes = Array::<HuffmanCode>::new();
    let mut decode_lookup = Array::new();
    let mut decode_lookup_dist = Array::new();
    let mut decode_lookup_clen = Array::new();

    loop
    {
        if input.len() == 0
        {
            return Err(InflateError::CorruptedData);
        }

        let mut bit_reader = BitReader::new(input);
        let bfinal = bit_reader.consume(1) == 1;
        let btype = bit_reader.consume(2);

        if btype == 3
        {
            return Err(InflateError::CorruptedBlockHeader);
        }

        if btype == 0
        {
            input = &input[1..];

            if input.len() < 4
            {
                return Err(InflateError::CorruptedData);
            }

            let len = (input[0] as u16) + (input[1] as u16) * 256;
            let nlen = (input[2] as u16) + (input[3] as u16) * 256;

            if len != !nlen
            {
                return Err(InflateError::CorruptedBlockHeader);
            }

            input = &input[4..];

            if input.len() < len as usize
            {
                return Err(InflateError::CorruptedData);
            }

            output.extend(&input[..(len as usize)]);
            input = &input[(len as usize)..];
        }
        else
        {
            let (nlit, _ndist) = if btype == 2
            {
                let nlit = bit_reader.consume(5) + 257;
                let ndist = bit_reader.consume(5) + 1;
                let nclen = bit_reader.consume(4) + 4;

                const LEN_CODES_ORDER: [usize; 19] = [
                    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
                ];

                clen_codes.clear();
                clen_codes.resize_default(19);

                for i in 0..nclen
                {
                    let index = LEN_CODES_ORDER[i as usize];
                    clen_codes[index].len = bit_reader.consume(3) as u16;
                }

                build_huffman_codes_in_place(&mut clen_codes);
                build_huffman_decode_lookup(&clen_codes, &mut decode_lookup_clen);

                codes.clear();
                codes.reserve((nlit + ndist) as usize);

                let mut code_index = 0;
                while code_index < nlit + ndist
                {
                    let value = decode_huffman(&mut bit_reader, &clen_codes, &decode_lookup_clen);

                    let (value, count) = match value
                    {
                        len @ 0..=15 => (len, 1),
                        16 =>
                        {
                            let count = 3 + bit_reader.consume(2);

                            if codes.len() == 0
                            {
                                return Err(InflateError::CorruptedData);
                            }

                            (codes[codes.len() - 1].len, count)
                        }
                        17 =>
                        {
                            let count = 3 + bit_reader.consume(3);
                            (0, count)
                        }
                        18 =>
                        {
                            let count = 11 + bit_reader.consume(7);
                            (0, count)
                        }
                        _ => unreachable!(),
                    };

                    codes.extend((0..count).map(|_| HuffmanCode {
                        code: 0,
                        len: value,
                    }));

                    code_index += count;
                }

                build_huffman_codes_in_place(&mut codes[0..nlit as usize]);
                build_huffman_codes_in_place(&mut codes[nlit as usize..]);

                unimplemented!();
                // @Todo This is not actually unimplemented, but it's not
                // tested yet so I want to be notified if anything goes through this code path
                // before it has been properly tested.

                (nlit as usize, ndist as usize)
            }
            else
            {
                build_fixed_huffman_codes(&mut codes);
                (288, 32)
            };

            build_huffman_decode_lookup(&codes[0..nlit], &mut decode_lookup);
            build_huffman_decode_lookup(&codes[nlit..], &mut decode_lookup_dist);

            let lit_codes = &codes[0..nlit];
            let dist_codes = &codes[nlit..];

            loop
            {
                let value = decode_huffman(&mut bit_reader, &lit_codes, &decode_lookup);

                if value == 256
                {
                    break;
                }
                else if value <= 255
                {
                    output.push(value as u8);
                }
                else
                {
                    let (extra_length_bit_count, extra_length_base) =
                        EXTRA_LENGTHS[(value - 257) as usize];
                    let extra_length = bit_reader.consume(extra_length_bit_count);
                    let length = extra_length as usize + extra_length_base;

                    let value = decode_huffman(&mut bit_reader, &dist_codes, &decode_lookup_dist);

                    let (extra_dist_bit_count, extra_dist_base) = EXTRA_DIST[value as usize];
                    let extra_dist = bit_reader.consume(extra_dist_bit_count);
                    let dist = extra_dist as usize + extra_dist_base;

                    if dist > window_size || dist as usize > output.len()
                    {
                        return Err(InflateError::ReadPastWindowSize);
                    }

                    let read_index = output.len() - dist;
                    for index in read_index..(read_index + length)
                    {
                        output.push(output[index]);
                    }
                }
            }
        }

        if bfinal
        {
            break;
        }
    }

    let mut checksum = Adler32::new();
    checksum.eat_slice(&output);

    if checksum.finish() != checksum_input
    {
        return Err(InflateError::CorruptedData);
    }

    return Ok(output);
}

const MAX_HUFFMAN_CODE_LEN: usize = 16;

#[derive(Debug, PartialEq, Default)]
struct HuffmanCode
{
    code: u16,
    len: u16,
}

fn decode_huffman<'a, R: Read>(
    bit_reader: &mut BitReader<R>,
    codes: &'a [HuffmanCode],
    decode_lookup: &[u16],
) -> u16
{
    let literal = bit_reader.peek(16);
    let code_index = decode_lookup[literal as usize];
    let code = &codes[code_index as usize];
    bit_reader.consume(code.len as usize);
    return code_index;
}

fn build_huffman_decode_lookup<A: Allocator>(
    codes: &[HuffmanCode],
    decode_lookup: &mut Array<u16, A>,
)
{
    decode_lookup.resize_default(1 << MAX_HUFFMAN_CODE_LEN);

    for (code_index, code) in codes.iter().filter(|c| c.len > 0).enumerate()
    {
        let remaining_bits = MAX_HUFFMAN_CODE_LEN - code.len as usize;
        let fill_count = 1 << remaining_bits;

        for fill in 0..fill_count
        {
            let code = (code.code << remaining_bits) | fill;
            decode_lookup[code.reverse_bits() as usize] = code_index as u16;
        }
    }
}

fn build_fixed_huffman_codes<A: Allocator>(codes: &mut Array<HuffmanCode, A>)
{
    const NLIT: usize = 288;
    const NDIST: usize = 32;
    const LITERAL_LENGTHS: [(u16, u16, usize); 4] =
        [(0, 143, 8), (144, 255, 9), (256, 279, 7), (280, 287, 8)];
    const DIST_LENGTH: usize = 5;

    codes.clear();
    codes.reserve(NLIT + NDIST);

    for (lit_min, lit_max, code_len) in LITERAL_LENGTHS.iter()
    {
        for _ in *lit_min..=*lit_max
        {
            codes.push(HuffmanCode {
                code: 0,
                len: *code_len as u16,
            })
        }
    }

    for _ in 0..NDIST
    {
        codes.push(HuffmanCode {
            code: 0,
            len: DIST_LENGTH as u16,
        })
    }

    build_huffman_codes_in_place(&mut codes[0..NLIT]);
    build_huffman_codes_in_place(&mut codes[NLIT..]);
}

pub(self) fn build_huffman_codes_in_place(codes: &mut [HuffmanCode])
{
    // Count code of each length
    let mut bl_count = [0; MAX_HUFFMAN_CODE_LEN];
    codes
        .iter()
        .for_each(|code| bl_count[code.len as usize] += 1);

    // Compute first code of each bit length
    let mut next_code = [0; MAX_HUFFMAN_CODE_LEN];
    let mut code = 0;
    for bits in 1..=MAX_HUFFMAN_CODE_LEN
    {
        code = (code + bl_count[bits - 1]) << 1;
        next_code[bits - 1] = code;
    }

    // Assign value to each code
    for code in codes.iter_mut()
    {
        if code.len > 0
        {
            code.code = next_code[code.len as usize - 1];
            next_code[code.len as usize - 1] += 1;
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::alloc::{Allocator, GlobalAllocator};

    #[test]
    fn compressed()
    {
        const COMPRESSED: &[u8] = &[
            0x78, 0x9C, 0xF3, 0x48, 0xCD, 0xC9, 0xC9, 0xD7, 0x51, 0x28, 0xCF, 0x2F, 0xCA, 0x49,
            0x51, 0x04, 0x00, 0x20, 0x5E, 0x04, 0x8A,
        ];

        let output = inflate(COMPRESSED).unwrap();
        assert_eq!(core::str::from_utf8(&output).unwrap(), "Hello, world!");
    }

    #[test]
    fn compressed2()
    {
        const COMPRESSED: &[u8] = &[
            0x78, 0x5E, 0xF3, 0x48, 0xCD, 0xC9, 0xC9, 0xD7, 0x51, 0x28, 0xCF, 0x2F, 0xCA, 0x49,
            0x51, 0x04, 0x00, 0x20, 0x5E, 0x04, 0x8A,
        ];

        let output = inflate(COMPRESSED).unwrap();
        assert_eq!(core::str::from_utf8(&output).unwrap(), "Hello, world!");
    }

    #[test]
    fn compressed3()
    {
        const COMPRESSED: &[u8] = &[
            0x78, 0x9C, 0x4B, 0x4C, 0x84, 0x80, 0x24, 0x28, 0x80, 0x72, 0x13, 0x01, 0x72, 0x28,
            0x09, 0x21,
        ];

        let output = inflate(COMPRESSED).unwrap();
        assert_eq!(
            core::str::from_utf8(&output).unwrap(),
            "aaaaaaaabbbbbbbbaaaaaaaa"
        );
    }

    #[test]
    fn compressed4()
    {
        const COMPRESSED: &[u8] = include_bytes!("../text.txt");

        let output = inflate(COMPRESSED).unwrap();
        assert_eq!(
            core::str::from_utf8(&output).unwrap(),
            "aaaaaaaabbbbbbbbaaaaaaaa"
        );
    }

    #[test]
    fn uncompressed()
    {
        const UNCOMPRESSED: &[u8] = &[
            0x78, 0x01, 0x01, 0x0D, 0x00, 0xF2, 0xFF, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20,
            0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21, 0x20, 0x5E, 0x04, 0x8A,
        ];

        let output = inflate(UNCOMPRESSED).unwrap();
        assert_eq!(core::str::from_utf8(&output).unwrap(), "Hello, world!");
    }

    fn build_huffman_codes<A: Allocator>(lengths: &[usize], alloc: A) -> Array<HuffmanCode, A>
    {
        let mut codes = Array::new_with(alloc);
        codes.extend(lengths.iter().map(|len| HuffmanCode {
            code: 0,
            len: *len as u16,
        }));
        build_huffman_codes_in_place(&mut codes);
        return codes;
    }

    #[test]
    fn huffman_codes()
    {
        let lengths = &[3, 3, 3, 3, 3, 2, 4, 4];
        let codes = build_huffman_codes(lengths, GlobalAllocator);

        assert_eq!(codes.len(), lengths.len());
        assert_eq!(codes[0], HuffmanCode { len: 3, code: 2 });
        assert_eq!(codes[1], HuffmanCode { len: 3, code: 3 });
        assert_eq!(codes[2], HuffmanCode { len: 3, code: 4 });
        assert_eq!(codes[3], HuffmanCode { len: 3, code: 5 });
        assert_eq!(codes[4], HuffmanCode { len: 3, code: 6 });
        assert_eq!(codes[5], HuffmanCode { len: 2, code: 0 });
        assert_eq!(codes[6], HuffmanCode { len: 4, code: 14 });
        assert_eq!(codes[7], HuffmanCode { len: 4, code: 15 });
    }
}
