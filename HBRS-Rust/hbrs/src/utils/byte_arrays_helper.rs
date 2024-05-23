use crate::consts::errors::HbrsError;

pub struct ByteArraysHelper;

impl ByteArraysHelper {
    // Returns the string in the form a vector of bytes restricted to a certain length
    pub fn cp_arr_string_fill(data: String, strict_length: usize) -> Result<Vec<u8>, HbrsError>
    {
        let data2: Vec<u8> = if data.is_empty(){
            Vec::new()
        } else {
            data.as_bytes().to_vec()
        };

       return Self::cp_arr_bytes_fill(&data2, strict_length);
    }

    pub fn cp_arr_bytes_fill(data: &Vec<u8>, strict_length: usize) -> Result<Vec<u8>, HbrsError>
    {
        let mut vector_by: Vec<u8> = vec![0; strict_length];

        if data.len() < strict_length as usize
        {
            return Err(HbrsError::DataLengthExceedsLimit);
        }

        vector_by[..data.len()].copy_from_slice(&data);
        return Ok(vector_by);
    }

    // Combines 2 vectors of bytes into one vector bytes
    pub fn combine_2v(first: &Vec<u8>, second: &Vec<u8>) -> Vec<u8>{
        let mut combined: Vec<u8> = vec![0; (first.len() + second.len()) as usize];

        combined[..first.len()].copy_from_slice(&first);
        combined[first.len()..second.len()].copy_from_slice(&second);

        return combined;
    }
    
    // Combines a vector of bytes and a single byte into one vector of bytes
    pub fn combine_1v_1b(first: &Vec<u8>, second: u8) -> Vec<u8>{
        let mut combined: Vec<u8> = vec![0; (first.len() + 1) as usize];

        combined[..first.len()].copy_from_slice(&first);
        combined[first.len()] = second;

        return combined;
    }

    // Combines 2 single bytes into one vector of bytes
    pub fn combine_2b(first: u8, second: u8) -> Vec<u8>{
        let mut combined: Vec<u8> = vec![0; 2];

        combined[0] = first;
        combined[1] = second;

        return combined;
    }

    // Turn 2 8 bit values into a 16 bit value
    pub fn port16_to_word(b0: u8, b1: u8) -> u16{
        return (b1 as u16) * 256 + b0 as u16;
    }

    // Turn a 16 bit value into 2 8 bit values and return a tuple of both
    pub fn word_to_port16(value: u16) -> (u8, u8){
        let b0 = (value & 0xFF) as u8;
        let b1 = (value >> 8) as u8;

        return (b0, b1);
    }

    //Turn one 32 bit value into 4 8 bit values and return a vector of all 4
    pub fn short_to_port32(value: u32) -> Vec<u8>{
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push((value & 0xFF) as u8);
        bytes.push(((value >> 8) & 0xFF) as u8);
        bytes.push(((value >> 16) & 0xFF) as u8);
        bytes.push((value >> 24) as u8);

        return bytes;
        
    }

    //Turn 4 8 bit values into one 32 bit value
    pub fn port32_to_dword(b0: u8, b1: u8, b2: u8, b3: u8) -> u32{
        return (b3 as u32) << 24 | (b2 as u32) << 16 | (b1 as u32) << 8 | b0 as u32;
    }

    pub fn int16_to_vectorby(value: i16) -> Vec<u8>{
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push((value & 0xFF) as u8);
        bytes.push((value >> 8) as u8);

        return bytes;
    }
}