#![allow(dead_code)]
use serde_derive::Deserialize;
use std::env;


#[derive(Deserialize, Debug)]
struct SignalValueNames
{
    name: String,
    value: i32
}

#[derive(Deserialize, Debug)]
struct CanSignal
{
    name: String,
    description: String,
    start_byte: i32,
    start_bit: i32,
    bit_length: i32,
    signal_value_names: Vec<SignalValueNames>
}

#[derive(Deserialize, Debug)]
struct CanFrame
{
    id: String,
    protocol: String,
    length: i32,
    signals: Vec<CanSignal>
}

#[derive(Deserialize, Debug)]
struct CanCommunicationMatrix
{
    can_frames: Vec<CanFrame>
}


static COM_MATRIX_FILE_NAME: &str = "k_matrix.json";  

fn load_communication_matrix() ->  Result<CanCommunicationMatrix, std::io::Error>
{
    let mut file_path = env::current_exe()?;
    file_path.pop();
    file_path.push(COM_MATRIX_FILE_NAME);

    let kmatrix_contents = std::fs::read_to_string(&file_path)?;
    let com_matrix = serde_json::from_str::<CanCommunicationMatrix>(&kmatrix_contents).unwrap();
    
    Ok(com_matrix)
}

fn get_higher_payload(payload: &u8, bits_to_take: i32, shift_by: i32, ) -> u32
{
    let mut mask: u8 = 0;
    for i in 0..bits_to_take {
        mask = mask + (1 << i);
    }

    ((payload & mask) as u32) << shift_by
}

fn get_lower_payload(payload: &u8, start_bit: i32, bit_len: i32, ) -> u8
{
    let mut bit_end = start_bit + bit_len;
    if bit_end > 8 { bit_end = 8; }

    let shift_by = start_bit as u8;

    let mut mask: u8 = 0;

    for i in start_bit..bit_end
    {
        mask = mask + (1 << i);
    }

    (payload & mask) >> shift_by
}

// TODO there are signal of integer type (those are not supported currently)
fn get_signal_val(payload: &Vec<u8>, s: &CanSignal) -> u32
{
    let mut value = get_lower_payload(&payload[(s.start_byte-1) as usize], s.start_bit, s.bit_length) as u32;

    let mut bits_left_to_take = s.bit_length - (8 - s.start_bit);
    let mut shift_by = 8 - s.start_bit;
    let mut i = s.start_byte as usize;

    while  bits_left_to_take > 0 
    {
        let mut bits_to_take = bits_left_to_take;

        // in case the signal is spread through more than two bytes:
        if bits_to_take > 8  { bits_to_take = 8;  }

        value = value + get_higher_payload(&payload[i], bits_to_take, shift_by) as u32;

        i=i+1;
        shift_by = shift_by + 8;
        bits_left_to_take = bits_left_to_take - bits_to_take;
    }

    value
}

fn get_named_signal(sig_val: u32, sig_val_names: &Vec<SignalValueNames>) -> String
{
    let l = sig_val_names.iter().find(|&x| x.value == sig_val as i32);
        
    match l {
        Some(sig_val_name) =>  sig_val_name.name.clone(),
        _ => sig_val.to_string()
    }
}

// example command with input args
// cargo run -- "0x12DD5570 8 0x00 0x07 0x00 0x00 0xC8 0xE0 0x01 0x00"
fn main() -> Result<(), std::io::Error>
{
    let input_argument = std::env::args().nth(1).expect("Missing a input argument, provide can frame in form \"ID DLC B1 B2 B3 B4 B5 B6 B7 B8\" !!!");

    let value_collection = input_argument.split(" ").collect::<Vec<&str>>();
    let can_id_str = value_collection[0];
    let dlc = value_collection[1].parse::<usize>().unwrap();

    let mut payload: Vec<u8> = Vec::new();

    for id in 0..(dlc) {
        let byte_str = value_collection[id+2];
        let without_prefix = byte_str.trim_start_matches("0x");
        let z = u8::from_str_radix(without_prefix, 16).unwrap();
        payload.push(z);
    }

    let can_frame_collection =  load_communication_matrix()?;

    let can_frame = can_frame_collection.can_frames.iter().find(|&x| x.id == can_id_str)
                                  .expect("Can Frame could not be found, please check can.id correctnes or add defnition to k_matrix file");
    
    println!("decypring frame with ID: {}", can_frame.id);
    
    assert!(can_frame.length ==  dlc as i32, "The provided Frame length (DLC) does not match the one from the k_matrix.");

    for s in &can_frame.signals 
    {
        let value = get_signal_val(&payload, &s);
        
        let sig_val_str = get_named_signal(value, &s.signal_value_names);
        
        println!("{} : {}", s.name, sig_val_str);
    }

    Ok(())
}
