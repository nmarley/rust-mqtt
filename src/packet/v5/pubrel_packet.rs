use heapless::Vec;

use crate::encoding::variable_byte_integer::VariableByteIntegerEncoder;
use crate::packet::v5::mqtt_packet::Packet;
use crate::utils::buffer_reader::BuffReader;
use crate::utils::buffer_writer::BuffWriter;
use crate::utils::types::BufferError;

use super::packet_type::PacketType;
use super::property::Property;

pub struct PubrelPacket<'a, const MAX_PROPERTIES: usize> {
    pub fixed_header: u8,
    pub remain_len: u32,
    pub packet_identifier: u16,
    pub reason_code: u8,
    pub property_len: u32,
    pub properties: Vec<Property<'a>, MAX_PROPERTIES>,
}

impl<'a, const MAX_PROPERTIES: usize> PubrelPacket<'a, MAX_PROPERTIES> {}

impl<'a, const MAX_PROPERTIES: usize> Packet<'a> for PubrelPacket<'a, MAX_PROPERTIES> {
    fn new() -> Self {
        Self {
            fixed_header: 0,
            remain_len: 0,
            packet_identifier: 0,
            reason_code: 0,
            property_len: 0,
            properties: Vec::<Property<'a>, MAX_PROPERTIES>::new(),
        }
    }

    fn encode(&mut self, buffer: &mut [u8], buffer_len: usize) -> Result<usize, BufferError> {
        let mut buff_writer = BuffWriter::new(buffer, buffer_len);

        let mut rm_ln = self.property_len;
        let property_len_enc: [u8; 4] = VariableByteIntegerEncoder::encode(self.property_len)?;
        let property_len_len = VariableByteIntegerEncoder::len(property_len_enc);
        rm_ln = rm_ln + property_len_len as u32 + 3;

        buff_writer.write_u8(self.fixed_header)?;
        buff_writer.write_variable_byte_int(rm_ln)?;
        buff_writer.write_u16(self.packet_identifier)?;
        buff_writer.write_u8(self.reason_code)?;
        buff_writer.write_variable_byte_int(self.property_len)?;
        buff_writer.write_properties::<MAX_PROPERTIES>(&self.properties)?;
        Ok(buff_writer.position)
    }

    fn decode(&mut self, buff_reader: &mut BuffReader<'a>) -> Result<(), BufferError> {
        if self.decode_fixed_header(buff_reader)? != PacketType::Pubrel {
            error!("Packet you are trying to decode is not PUBREL packet!");
            return Err(BufferError::PacketTypeMismatch);
        }
        self.packet_identifier = buff_reader.read_u16()?;
        self.reason_code = buff_reader.read_u8()?;
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
        property.pubrel_property()
    }

    fn set_fixed_header(&mut self, header: u8) {
        self.fixed_header = header;
    }

    fn set_remaining_len(&mut self, remaining_len: u32) {
        self.remain_len = remaining_len;
    }
}
