use crate::packet::v5::connack_packet::ConnackPacket;
use crate::packet::v5::mqtt_packet::Packet;
use crate::packet::v5::property::Property;
use crate::packet::v5::reason_codes::ReasonCode;
use crate::utils::buffer_reader::BuffReader;

#[test]
fn test_encode() {
    let mut buffer: [u8; 100] = [0; 100];
    let mut connack = ConnackPacket::<2>::new();
    connack.property_len = 3;
    let prop = Property::ReceiveMaximum(21);
    connack.properties.push(prop);
    connack.connect_reason_code = ReasonCode::ServerMoved.into();
    connack.ack_flags = 0x45;

    let res = connack.encode(&mut buffer, 100);
    assert!(res.is_ok());
    assert_eq!(
        buffer[0..res.unwrap()],
        [
            0x20,
            0x06,
            0x45,
            ReasonCode::ServerMoved.into(),
            0x03,
            0x21,
            0x00,
            0x15
        ]
    )
}

#[test]
fn test_decode() {
    let mut buffer: [u8; 8] = [
        0x20,
        0x06,
        0x45,
        ReasonCode::ServerMoved.into(),
        0x03,
        0x21,
        0x00,
        0x15,
    ];
    let mut connack_res = ConnackPacket::<2>::new();
    let res = connack_res.decode(&mut BuffReader::new(&buffer, 8));

    assert!(res.is_ok());
    assert_eq!(connack_res.property_len, 3);
    assert_eq!(connack_res.ack_flags, 0x45);
    assert_eq!(
        connack_res.connect_reason_code,
        ReasonCode::ServerMoved.into()
    );
    assert_eq!(connack_res.property_len, 3);
    let prop = connack_res.properties.get(0).unwrap();
    assert_eq!(<&Property as Into<u8>>::into(prop), 0x21);
    if let Property::ReceiveMaximum(u) = *prop {
        assert_eq!(u, 21);
    }
}
