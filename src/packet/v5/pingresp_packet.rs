use crate::packet::v5::mqtt_packet::Packet;
use crate::utils::buffer_reader::BuffReader;
use crate::utils::buffer_writer::BuffWriter;
use crate::utils::types::BufferError;

use super::packet_type::PacketType;
use super::property::Property;

pub struct PingrespPacket {
    pub fixed_header: u8,
    pub remain_len: u32,
}

impl PingrespPacket {}

impl<'a> Packet<'a> for PingrespPacket {
    fn new() -> Self {
        Self {
            fixed_header: PacketType::Pingresp.into(),
            remain_len: 0,
        }
    }

    fn encode(&mut self, buffer: &mut [u8], buffer_len: usize) -> Result<usize, BufferError> {
        let mut buff_writer = BuffWriter::new(buffer, buffer_len);
        buff_writer.write_u8(self.fixed_header)?;
        buff_writer.write_variable_byte_int(self.remain_len)?;
        Ok(buff_writer.position)
    }

    fn decode(&mut self, buff_reader: &mut BuffReader<'a>) -> Result<(), BufferError> {
        let x = self.decode_fixed_header(buff_reader)?;
        if x != PacketType::Pingresp {
            error!("Packet you are trying to decode is not PINGRESP packet!");
            return Err(BufferError::PacketTypeMismatch);
        }
        if self.remain_len != 0 {
            error!("PINGRESP packet does not have 0 lenght!");
            return Err(BufferError::PacketTypeMismatch);
        }
        Ok(())
    }

    fn set_property_len(&mut self, _value: u32) {
        error!("PINGRESP packet does not contain any properties!");
    }

    fn get_property_len(&mut self) -> u32 {
        error!("PINGRESP packet does not contain any properties!");
        0
    }

    fn push_to_properties(&mut self, _property: Property<'a>) {
        error!("PINGRESP packet does not contain any properties!");
    }

    fn property_allowed(&mut self, property: &Property<'a>) -> bool {
        property.pingresp_property()
    }

    fn set_fixed_header(&mut self, header: u8) {
        self.fixed_header = header;
    }

    fn set_remaining_len(&mut self, remaining_len: u32) {
        self.remain_len = remaining_len;
    }
}
