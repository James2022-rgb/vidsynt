use std::io::{self, Read, Seek};

use crate::h265::nalu::{NaluValueContext, Nalu};
use crate::h265::nalu_ref::NaluRef;

#[derive(Debug, Clone, Copy)]
pub struct ByteStreamContent<T> {
    pub offset: usize,
    pub value: T,
    pub consumed: usize,
}

pub type LengthPrefixedByteStreamNaluReader<R> =
    LengthPrefixedByteStreamContentReader<R, NaluReader>;
pub type LengthPrefixedByteStreamNaluRefReader<R> =
    LengthPrefixedByteStreamContentReader<R, NaluRefReader>;

impl<R> LengthPrefixedByteStreamNaluReader<R> {
    pub fn with_length_size_minus_one(
        length_size_minus_one: usize,
        inner_reader: R,
        nalu_value_context: NaluValueContext,
    ) -> Self {
        Self {
            length_size_minus_one,
            inner_reader,
            content_reader: NaluReader { nalu_value_context },
        }
    }
}

impl<R> LengthPrefixedByteStreamNaluRefReader<R> {
    pub fn with_length_size_minus_one(
        length_size_minus_one: usize,
        inner_reader: R,
        nalu_value_context: NaluValueContext,
    ) -> Self {
        Self {
            length_size_minus_one,
            inner_reader,
            content_reader: NaluRefReader { nalu_value_context },
        }
    }
}

/// A `ReadContent` that reads `Nalu`s.
#[derive(Debug)]
pub struct NaluReader {
    nalu_value_context: NaluValueContext,
}

impl<R: Read> ReadContent<R, Nalu> for NaluReader {
    fn read_content(
        &mut self,
        reader: &mut R,
        length: usize,
        current_offset: usize,
    ) -> Result<(usize, Nalu), io::Error> {
        let nalu = Nalu::from_reader(reader, length, self.nalu_value_context)?;
        Ok((length, nalu))
    }
}

/// A `ReadContent` that reads `NaluRef`s.
#[derive(Debug)]
pub struct NaluRefReader {
    nalu_value_context: NaluValueContext,
}

impl<R: Read + Seek> ReadContent<R, NaluRef> for NaluRefReader {
    fn read_content(
        &mut self,
        reader: &mut R,
        length: usize,
        current_offset: usize,
    ) -> Result<(usize, NaluRef), io::Error> {
        let (consumed, nalu_ref) = NaluRef::from_reader(reader, current_offset)?;
        Ok((consumed, nalu_ref))
    }
}

pub trait ReadContent<R: Read, T> {
    /// Reads a `T`, returning the number of bytes consumed and the `T` itself.
    fn read_content(
        &mut self,
        reader: &mut R,
        length: usize,
        current_offset: usize,
    ) -> Result<(usize, T), io::Error>;
}

#[derive(Debug)]
pub struct LengthPrefixedByteStreamContentReader<R, CR> {
    length_size_minus_one: usize,
    inner_reader: R,
    content_reader: CR,
}

impl<R: Read + Seek, CR> LengthPrefixedByteStreamContentReader<R, CR> {
    pub fn read_contents_until_eof<T>(&mut self) -> Result<Vec<ByteStreamContent<T>>, io::Error>
    where
        CR: ReadContent<R, T>,
    {
        let mut contents = Vec::new();

        let mut current_offset: usize = 0;

        loop {
            match self.read_content(current_offset) {
                Ok(bytestream_content) => {
                    current_offset += bytestream_content.consumed;
                    contents.push(bytestream_content);
                }
                Err(err) => {
                    if err.kind() == io::ErrorKind::UnexpectedEof {
                        break;
                    } else {
                        return Err(err);
                    }
                }
            }
        }

        Ok(contents)
    }

    pub fn read_content<T>(
        &mut self,
        current_offset: usize,
    ) -> Result<ByteStreamContent<T>, io::Error>
    where
        CR: ReadContent<R, T>,
    {
        // tracing::trace!("read_content: @0x{:#08x}", current_offset);

        // We expect to get `UnexpectedEof` if there is no more content, which we propagate as-is.
        let length = self.read_length()?;
        // tracing::trace!("length: {}", length);

        // The offset of the content itself, not the length.
        let current_offset = current_offset + self.length_size_minus_one + 1;

        let (content_consumed, content) =
            self.content_reader
                .read_content(&mut self.inner_reader, length, current_offset)?;
        // tracing::trace!("@0x{:#08x} content_consumed: {}", current_offset, content_consumed);

        // Consume remaining bytes if any.
        let remaining = length - content_consumed;
        if remaining > 0 {
            tracing::trace!("Consume remaining {} bytes", remaining);
            self.inner_reader
                .seek(io::SeekFrom::Current(remaining.try_into().unwrap()))?;
        }

        let consumed = self.length_size_minus_one + 1 + length;

        Ok(ByteStreamContent {
            offset: current_offset,
            value: content,
            consumed,
        })
    }

    fn read_length(&mut self) -> Result<usize, io::Error> {
        let mut buf = [0; 4];
        self.inner_reader
            .read_exact(&mut buf[..=self.length_size_minus_one])?;
        let length = buf[..=self.length_size_minus_one]
            .iter()
            .fold(0, |acc, &x| acc << 8 | x as usize);
        Ok(length)
    }
}

pub fn parse_nalus_length_prefixed(
    length_prefixed_byte_stream: &[u8],
    length_size_minus_one: usize,
    nalu_value_context: NaluValueContext,
) -> Vec<ByteStreamContent<Nalu>> {
    LengthPrefixedByteStreamNaluReader::with_length_size_minus_one(
        length_size_minus_one,
        // `Seek` is currently a requirement, which we hope to lift.
        io::Cursor::new(&length_prefixed_byte_stream),
        nalu_value_context,
    )
    .read_contents_until_eof()
    .unwrap()
}

/// Returns a tuple of slice segment start code offsets and the converted _Annex B_ byte stream.
pub fn parse_length_prefixed_and_convert_to_annex_b(
    length_prefixed_byte_stream: &[u8],
    length_size_minus_one: usize,
    nalu_value_context: NaluValueContext,
) -> (Vec<u32>, Vec<u8>) {
    let nalu_contents = parse_nalus_length_prefixed(
        length_prefixed_byte_stream,
        length_size_minus_one,
        nalu_value_context,
    );

    convert_length_prefixed_to_annex_b(
        length_prefixed_byte_stream,
        length_size_minus_one,
        &nalu_contents,
    )
}

/// Returns a tuple of slice segment start code offsets and the converted _Annex B_ byte stream.
pub fn convert_length_prefixed_to_annex_b<'a>(
    length_prefixed_byte_stream: &[u8],
    length_size_minus_one: usize,
    nalu_contents: impl IntoIterator<Item = &'a ByteStreamContent<Nalu>>,
) -> (Vec<u32>, Vec<u8>) {
    let mut slice_segment_offsets: Vec<u32> = Default::default();
    let mut annex_b_byte_stream: Vec<u8> = Vec::with_capacity(length_prefixed_byte_stream.len());

    for nalu_content in nalu_contents.into_iter() {
        let extend_vec_with_bytes = |dst: &mut Vec<u8>, bytes: &[u8]| {
            dst.reserve(dst.len() + bytes.len());
            unsafe {
                std::slice::from_raw_parts_mut(dst.as_mut_ptr().add(dst.len()), bytes.len())
                    .copy_from_slice(bytes);
                dst.set_len(dst.len() + bytes.len());
            }
        };

        let start_code_offset = annex_b_byte_stream.len() as u32;
        slice_segment_offsets.push(start_code_offset);

        // `start_code_prefix_one_3bytes`.
        extend_vec_with_bytes(&mut annex_b_byte_stream, &[0, 0, 1]);

        let nal_unit_size = nalu_content.consumed - (length_size_minus_one + 1);
        let nal_unit_bytes =
            &length_prefixed_byte_stream[nalu_content.offset..nalu_content.offset + nal_unit_size];

        // `nal_unit(NumBytesInNalUnit)`: NAL unit header + NAL unit value EBSP.
        extend_vec_with_bytes(&mut annex_b_byte_stream, nal_unit_bytes);
    }

    (slice_segment_offsets, annex_b_byte_stream)
}
