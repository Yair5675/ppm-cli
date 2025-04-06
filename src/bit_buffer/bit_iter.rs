use crate::bit_buffer::BitBuffer;

/// An iterator over bits. Can be derived from BitBuffer or a slice of bytes.
pub struct BitIterator<'a> {
    full_bytes_iter: Box<dyn Iterator<Item = u8> + 'a>,
    current_byte: Option<u8>,
    current_idx: usize,

    // In case there is an incomplete byte, hold it and the number of bits in it:
    incomplete_byte: Option<(u8, usize)>,
}

impl<'a> Iterator for BitIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        // First try the current byte:
        if let Some(byte) = self.current_byte.take() {
            // Get current bit:
            let bit = ((byte >> (7 - self.current_idx)) & 1) == 1;
            self.current_idx += 1;

            // Restore the current byte if not all bits are consumed, otherwise try to put a new one
            // there:
            if self.current_idx < 8 {
                let _ = self.current_byte.insert(byte);
            } else {
                self.current_idx = 0;
                self.current_byte = self.full_bytes_iter.next();
            }
            return Some(bit);
        }

        // Now try the incomplete byte:
        if let Some((byte, num_bits)) = self.incomplete_byte.take() {
            // Get current bit:
            let bit = ((byte >> (7 - self.current_idx)) & 1) == 1;
            self.current_idx += 1;

            // Restore byte or remove incomplete one:
            if self.current_idx < num_bits {
                let _ = self.incomplete_byte.insert((byte, num_bits));
            }
            Some(bit)
        } else {
            None
        }
    }
}

impl<'a> From<BitBuffer> for BitIterator<'a> {
    fn from(mut buffer: BitBuffer) -> Self {
        let mut full_bytes_iter = Box::new(buffer.get_complete_bytes());
        let current_idx = 0;
        let current_byte = full_bytes_iter.next();

        let incomplete_byte = if buffer.current_idx > 0 {
            Some((buffer.current_byte, buffer.current_idx))
        } else {
            None
        };

        Self {
            full_bytes_iter,
            current_idx,
            current_byte,
            incomplete_byte,
        }
    }
}

impl<'a, I: IntoIterator<Item = u8> + 'a> From<I> for BitIterator<'a> {
    fn from(value: I) -> Self {
        // There are only complete bytes here:
        let mut full_bytes_iter = Box::new(value.into_iter());
        let current_byte = full_bytes_iter.next();
        let current_idx = 0;
        let incomplete_byte = None;

        Self {
            full_bytes_iter,
            current_byte,
            current_idx,
            incomplete_byte,
        }
    }
}
