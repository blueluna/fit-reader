

const CRC_TABLE: [u16; 16] = [
    0x0000, 0xCC01, 0xD801, 0x1400, 0xF001, 0x3C00, 0x2800, 0xE401,
    0xA001, 0x6C00, 0x7800, 0xB401, 0x5000, 0x9C01, 0x8801, 0x4400];

pub fn crc_16(crc: u16, byte: &u8) -> u16 {
    // compute checksum of lower four bits of byte
    let mut tmp = CRC_TABLE[(crc & 0x0Fu16) as usize];
    let mut out = (crc >> 4) & 0x0FFFu16;
    out = out ^ tmp ^ CRC_TABLE[(byte & 0x0Fu8) as usize];
    // now compute checksum of upper four bits of byte
    tmp = CRC_TABLE[(out & 0x0Fu16) as usize];
    out = (out >> 4) & 0x0FFFu16;
    out = out ^ tmp ^ CRC_TABLE[((byte >> 4) & 0x0Fu8) as usize];
    return out;
}