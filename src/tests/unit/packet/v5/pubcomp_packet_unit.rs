use heapless::Vec;

use crate::packet::v5::mqtt_packet::Packet;
use crate::packet::v5::packet_type::PacketType;
use crate::packet::v5::property::Property;
use crate::packet::v5::pubcomp_packet::PubcompPacket;
use crate::utils::buffer_reader::BuffReader;
use crate::utils::types::EncodedString;

#[test]
fn test_encode() {
    let mut buffer: [u8; 14] = [0; 14];
    let mut packet = PubcompPacket::<1>::new();
    packet.fixed_header = PacketType::Pubcomp.into();
    packet.packet_identifier = 35420;
    packet.reason_code = 0x00;
    let mut str = EncodedString::new();
    str.string = "Wheel";
    str.len = 5;
    let mut props = Vec::<Property, 1>::new();
    props.push(Property::ReasonString(str));
    packet.property_len = packet.add_properties(&props);
    let res = packet.encode(&mut buffer, 14);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 14);
    assert_eq!(
        buffer,
        [0x70, 0x0C, 0x8A, 0x5C, 0x00, 0x08, 0x1F, 0x00, 0x05, 0x57, 0x68, 0x65, 0x65, 0x6c]
    )
}

#[test]
fn test_decode() {
    let buffer: [u8; 14] = [
        0x70, 0x0C, 0x8A, 0x5C, 0x00, 0x08, 0x1F, 0x00, 0x05, 0x57, 0x68, 0x65, 0x65, 0x6c,
    ];
    let mut packet = PubcompPacket::<1>::new();
    let res = packet.decode(&mut BuffReader::new(&buffer, 14));
    assert!(res.is_ok());
    assert_eq!(packet.fixed_header, PacketType::Pubcomp.into());
    assert_eq!(packet.packet_identifier, 35420);
    assert_eq!(packet.reason_code, 0x00);
    assert_eq!(packet.property_len, 8);
    let prop = packet.properties.get(0);
    assert!(prop.is_some());
    assert_eq!(<&Property as Into<u8>>::into(prop.unwrap()), 0x1F);
    if let Property::ReasonString(u) = (*prop.unwrap()).clone() {
        assert_eq!(u.string, "Wheel");
    }
}
