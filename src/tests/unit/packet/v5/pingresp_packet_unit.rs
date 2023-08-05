use crate::packet::v5::mqtt_packet::Packet;
use crate::packet::v5::packet_type::PacketType;
use crate::packet::v5::pingresp_packet::PingrespPacket;
use crate::utils::buffer_reader::BuffReader;

#[test]
fn test_encode() {
    let mut buffer: [u8; 3] = [0x00, 0x98, 0x45];
    let mut packet = PingrespPacket::new();
    packet.fixed_header = PacketType::Pingresp.into();
    packet.remain_len = 0;
    let res = packet.encode(&mut buffer, 3);
    assert!(res.is_ok());
    assert_eq!(buffer, [0xD0, 0x00, 0x45])
}

#[test]
fn test_decode() {
    let buffer: [u8; 3] = [0xD0, 0x00, 0x51];
    let mut packet = PingrespPacket::new();
    let res = packet.decode(&mut BuffReader::new(&buffer, 3));
    assert!(res.is_ok());
    assert_eq!(packet.fixed_header, PacketType::Pingresp.into());
    assert_eq!(packet.remain_len, 0);
}
