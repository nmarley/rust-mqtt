use crate::packet::v5::connect_packet::ConnectPacket;
use crate::packet::v5::mqtt_packet::Packet;

#[test]
fn test_encode() {
    let mut buffer: [u8; 100] = [0; 100];
    let mut connect = ConnectPacket::<1, 0>::clean();
    let res = connect.encode(&mut buffer, 100);

    assert!(res.is_ok());
    assert_eq!(
        buffer[0..res.unwrap()],
        [
            0x10, 0x10, 0x00, 0x04, 0x4d, 0x51, 0x54, 0x54, 0x05, 0x02, 0x00, 0x3c, 0x03, 0x21,
            0x00, 0x14, 0x00, 0x00
        ]
    )
}
