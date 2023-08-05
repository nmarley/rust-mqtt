use crate::encoding::variable_byte_integer::{
    VariableByteInteger, VariableByteIntegerDecoder, VariableByteIntegerEncoder,
};
use crate::utils::types::BufferError;

#[test]
fn test_decode() {
    static BUFFER: VariableByteInteger = [0x81, 0x81, 0x81, 0x01];

    let decoded = VariableByteIntegerDecoder::decode(BUFFER);
    assert!(decoded.is_ok());
    assert_eq!(decoded.unwrap(), 2113665);
}

#[test]
fn test_decode_small() {
    static BUFFER: VariableByteInteger = [0x81, 0x81, 0x01, 0x85];

    let decoded = VariableByteIntegerDecoder::decode(BUFFER);
    assert!(decoded.is_ok());
    assert_eq!(decoded.unwrap(), 16_513);
}

#[test]
fn test_encode() {
    let encoded = VariableByteIntegerEncoder::encode(211_366_5);
    assert!(encoded.is_ok());
    let res = encoded.unwrap();
    assert_eq!(res, [0x81, 0x81, 0x81, 0x01]);
    assert_eq!(VariableByteIntegerEncoder::len(res), 4);
}

#[test]
fn test_encode_small() {
    let encoded = VariableByteIntegerEncoder::encode(16_513);
    assert!(encoded.is_ok());
    let res = encoded.unwrap();
    assert_eq!(res, [0x81, 0x81, 0x01, 0x00]);
    assert_eq!(VariableByteIntegerEncoder::len(res), 3);
}

#[test]
fn test_encode_extra_small() {
    let encoded = VariableByteIntegerEncoder::encode(5);
    assert!(encoded.is_ok());
    let res = encoded.unwrap();
    assert_eq!(res, [0x05, 0x00, 0x00, 0x00]);
    assert_eq!(VariableByteIntegerEncoder::len(res), 1);
}

#[test]
fn test_encode_max() {
    let encoded = VariableByteIntegerEncoder::encode(288_435_455);
    assert!(encoded.is_err());
    assert_eq!(encoded.unwrap_err(), BufferError::EncodingError);
}
