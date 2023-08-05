use crate::packet::v5::mqtt_packet::Packet;
use crate::packet::v5::packet_type::PacketType;
use crate::packet::v5::pingreq_packet::PingreqPacket;

#[test]
fn test_encode() {
    let mut buffer: [u8; 3] = [0x00, 0x98, 0x45];
    let mut packet = PingreqPacket::new();
    packet.fixed_header = PacketType::Pingreq.into();
    packet.remain_len = 0;
    let res = packet.encode(&mut buffer, 3);
    assert!(res.is_ok());
    assert_eq!(buffer, [0xC0, 0x00, 0x45])
}
