use heapless::Vec;

use crate::encoding::variable_byte_integer::VariableByteIntegerEncoder;
use crate::packet::v5::mqtt_packet::Packet;
use crate::utils::buffer_reader::BuffReader;
use crate::utils::types::BufferError;

use super::packet_type::PacketType;
use super::property::Property;

pub struct SubackPacket<'a, const MAX_REASONS: usize, const MAX_PROPERTIES: usize> {
    pub fixed_header: u8,
    pub remain_len: u32,
    pub packet_identifier: u16,
    pub property_len: u32,
    pub properties: Vec<Property<'a>, MAX_PROPERTIES>,
    pub reason_codes: Vec<u8, MAX_REASONS>,
}

impl<'a, const MAX_REASONS: usize, const MAX_PROPERTIES: usize>
    SubackPacket<'a, MAX_REASONS, MAX_PROPERTIES>
{
    pub fn read_reason_codes(
        &mut self,
        buff_reader: &mut BuffReader<'a>,
    ) -> Result<(), BufferError> {
        let rm_ln_ln = VariableByteIntegerEncoder::len(
            VariableByteIntegerEncoder::encode(self.remain_len).unwrap(),
        );
        let max = self.remain_len as usize + rm_ln_ln + 1;
        if buff_reader.position >= max {
            return Ok(());
        }
        loop {
            self.reason_codes.push(buff_reader.read_u8()?);
            if buff_reader.position == max {
                break;
            }
        }
        Ok(())
    }
}

impl<'a, const MAX_REASONS: usize, const MAX_PROPERTIES: usize> Packet<'a>
    for SubackPacket<'a, MAX_REASONS, MAX_PROPERTIES>
{
    fn new() -> Self {
        Self {
            fixed_header: PacketType::Suback.into(),
            remain_len: 0,
            packet_identifier: 0,
            property_len: 0,
            properties: Vec::<Property<'a>, MAX_PROPERTIES>::new(),
            reason_codes: Vec::<u8, MAX_REASONS>::new(),
        }
    }

    fn encode(&mut self, _buffer: &mut [u8], _buffer_len: usize) -> Result<usize, BufferError> {
        error!("SUBACK packet does not support encoding!");
        Err(BufferError::WrongPacketToEncode)
    }

    fn decode(&mut self, buff_reader: &mut BuffReader<'a>) -> Result<(), BufferError> {
        if self.decode_fixed_header(buff_reader)? != PacketType::Suback {
            error!("Packet you are trying to decode is not SUBACK packet!");
            return Err(BufferError::PacketTypeMismatch);
        }
        self.packet_identifier = buff_reader.read_u16()?;
        self.decode_properties(buff_reader)?;
        self.read_reason_codes(buff_reader)
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
        property.suback_property()
    }

    fn set_fixed_header(&mut self, header: u8) {
        self.fixed_header = header;
    }

    fn set_remaining_len(&mut self, remaining_len: u32) {
        self.remain_len = remaining_len;
    }
}
