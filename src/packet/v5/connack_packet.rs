use heapless::Vec;

use crate::encoding::variable_byte_integer::VariableByteIntegerEncoder;
use crate::packet::v5::mqtt_packet::Packet;
use crate::utils::buffer_reader::BuffReader;
use crate::utils::buffer_writer::BuffWriter;
use crate::utils::types::BufferError;

use super::packet_type::PacketType;
use super::property::Property;

pub struct ConnackPacket<'a, const MAX_PROPERTIES: usize> {
    pub fixed_header: u8,
    pub remain_len: u32,
    pub ack_flags: u8,
    pub connect_reason_code: u8,
    pub property_len: u32,
    pub properties: Vec<Property<'a>, MAX_PROPERTIES>,
}

impl<'a, const MAX_PROPERTIES: usize> ConnackPacket<'a, MAX_PROPERTIES> {}

impl<'a, const MAX_PROPERTIES: usize> Packet<'a> for ConnackPacket<'a, MAX_PROPERTIES> {
    fn new() -> Self {
        Self {
            fixed_header: PacketType::Connack.into(),
            remain_len: 0,
            ack_flags: 0,
            connect_reason_code: 0,
            property_len: 0,
            properties: Vec::<Property<'a>, MAX_PROPERTIES>::new(),
        }
    }

    fn encode(&mut self, buffer: &mut [u8], buffer_len: usize) -> Result<usize, BufferError> {
        let mut buff_writer = BuffWriter::new(buffer, buffer_len);
        buff_writer.write_u8(self.fixed_header)?;
        let property_len_enc = VariableByteIntegerEncoder::encode(self.property_len)?;
        let property_len_len = VariableByteIntegerEncoder::len(property_len_enc);

        let rm_len: u32 = 2 + self.property_len + property_len_len as u32;
        buff_writer.write_variable_byte_int(rm_len)?;
        buff_writer.write_u8(self.ack_flags)?;
        buff_writer.write_u8(self.connect_reason_code)?;
        buff_writer.write_variable_byte_int(self.property_len)?;
        buff_writer.write_properties(&self.properties)?;
        Ok(buff_writer.position)
    }

    fn decode(&mut self, buff_reader: &mut BuffReader<'a>) -> Result<(), BufferError> {
        if self.decode_fixed_header(buff_reader)? != PacketType::Connack {
            error!("Packet you are trying to decode is not CONNACK packet!");
            return Err(BufferError::PacketTypeMismatch);
        }
        self.ack_flags = buff_reader.read_u8()?;
        self.connect_reason_code = buff_reader.read_u8()?;
        self.decode_properties(buff_reader)
    }

    fn set_property_len(&mut self, value: u32) {
        self.property_len = value;
    }

    fn get_property_len(&mut self) -> u32 {
        self.property_len
    }

    fn push_to_properties(&mut self, property: Property<'a>) {
        self.properties.push(property);
    }

    fn property_allowed(&mut self, property: &Property<'a>) -> bool {
        property.connack_property()
    }

    fn set_fixed_header(&mut self, header: u8) {
        self.fixed_header = header;
    }

    fn set_remaining_len(&mut self, remaining_len: u32) {
        self.remain_len = remaining_len;
    }
}
