pub fn encode_rle(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut encoded = Vec::new();
    let mut prev_byte = data[0];
    let mut count = 1;

    for &byte in data.iter().skip(1) {
        if count != 255 && byte == prev_byte {
            count += 1;
        } else {
            encoded.push(prev_byte);
            encoded.push(count);
            prev_byte = byte;
            count = 1;
        }
    }
    encoded.push(prev_byte);
    encoded.push(count);

    encoded
}

pub fn decode_rle(data: &[u8]) -> Vec<u8> {
    let mut decoded = Vec::new();

    let mut iter = data.iter();
    while let Some(&byte) = iter.next() {
        if let Some(&count) = iter.next() {
            decoded.extend(std::iter::repeat(byte).take(count as usize));
        }
    }

    decoded
}
