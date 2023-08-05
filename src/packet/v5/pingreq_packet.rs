use crate::packet::v5::mqtt_packet::Packet;
use crate::utils::buffer_reader::BuffReader;
use crate::utils::buffer_writer::BuffWriter;
use crate::utils::types::BufferError;

use super::packet_type::PacketType;
use super::property::Property;

pub struct PingreqPacket {
    pub fixed_header: u8,
    pub remain_len: u32,
}

impl PingreqPacket {}

impl<'a> Packet<'a> for PingreqPacket {
    fn new() -> Self {
        Self {
            fixed_header: PacketType::Pingreq.into(),
            remain_len: 0,
        }
    }

    fn encode(&mut self, buffer: &mut [u8], buffer_len: usize) -> Result<usize, BufferError> {
        let mut buff_writer = BuffWriter::new(buffer, buffer_len);
        buff_writer.write_u8(self.fixed_header)?;
        buff_writer.write_variable_byte_int(0)?;
        Ok(buff_writer.position)
    }

    fn decode(&mut self, _buff_reader: &mut BuffReader<'a>) -> Result<(), BufferError> {
        error!("Pingreq Packet packet does not support decode funtion on client!");
        Err(BufferError::WrongPacketToDecode)
    }

    fn set_property_len(&mut self, _value: u32) {
        error!("PINGREQ packet does not contain any properties!");
    }

    fn get_property_len(&mut self) -> u32 {
        error!("PINGREQ packet does not contain any properties!");
        0
    }

    fn push_to_properties(&mut self, _property: Property<'a>) {
        error!("PINGREQ packet does not contain any properties!");
    }

    fn property_allowed(&mut self, property: &Property<'a>) -> bool {
        property.pingreq_property()
    }

    fn set_fixed_header(&mut self, header: u8) {
        self.fixed_header = header;
    }

    fn set_remaining_len(&mut self, remaining_len: u32) {
        self.remain_len = remaining_len;
    }
}
