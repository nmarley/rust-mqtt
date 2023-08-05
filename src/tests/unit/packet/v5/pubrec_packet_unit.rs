use heapless::Vec;

use crate::packet::v5::mqtt_packet::Packet;
use crate::packet::v5::packet_type::PacketType;
use crate::packet::v5::property::Property;
use crate::packet::v5::pubrec_packet::PubrecPacket;
use crate::utils::buffer_reader::BuffReader;
use crate::utils::types::{EncodedString, StringPair};

#[test]
fn test_encode() {
    let mut buffer: [u8; 20] = [0; 20];
    let mut packet = PubrecPacket::<2>::new();
    packet.packet_identifier = 35420;
    packet.reason_code = 0x12;
    let mut name = EncodedString::new();
    name.string = "name1";
    name.len = 5;
    let mut val = EncodedString::new();
    val.string = "val1";
    val.len = 4;
    let mut pair = StringPair::new();
    pair.name = name;
    pair.value = val;
    let mut props = Vec::<Property, 1>::new();
    props.push(Property::UserProperty(pair));
    packet.property_len = packet.add_properties(&props);
    let res = packet.encode(&mut buffer, 20);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 20);
    assert_eq!(
        buffer,
        [
            0x50, 0x12, 0x8A, 0x5C, 0x12, 0x0E, 0x26, 0x00, 0x05, 0x6e, 0x61, 0x6d, 0x65, 0x31,
            0x00, 0x04, 0x76, 0x61, 0x6c, 0x31
        ]
    )
}

#[test]
fn test_decode() {
    let buffer: [u8; 20] = [
        0x50, 0x12, 0x8A, 0x5C, 0x12, 0x0E, 0x26, 0x00, 0x05, 0x6e, 0x61, 0x6d, 0x65, 0x31, 0x00,
        0x04, 0x76, 0x61, 0x6c, 0x31,
    ];
    let mut packet = PubrecPacket::<1>::new();
    let res = packet.decode(&mut BuffReader::new(&buffer, 20));
    assert!(res.is_ok());
    assert_eq!(packet.fixed_header, PacketType::Pubrec.into());
    assert_eq!(packet.remain_len, 18);
    assert_eq!(packet.packet_identifier, 35420);
    assert_eq!(packet.reason_code, 0x12);
    assert_eq!(packet.property_len, 14);
    let prop = packet.properties.get(0);
    assert!(prop.is_some());
    assert_eq!(<&Property as Into<u8>>::into(prop.unwrap()), 0x26);
    if let Property::UserProperty(u) = (*prop.unwrap()).clone() {
        assert_eq!(u.name.len, 5);
        assert_eq!(u.name.string, "name1");
        assert_eq!(u.value.len, 4);
        assert_eq!(u.value.string, "val1");
    }
}
