use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FrameDim {
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NumpyMetaData {
    total_bytes: i32,
    strides: Option<(i32, Option<i32>)>,
    shape: Option<(i32, Option<i32>)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrdDetectionOutputMeta {
    frame_digest: NumpyMetaData,
}

#[derive(Debug)]
pub struct DetectionServerMeta {
    pub ip: String,
    pub port: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    model_sha256_hash: String,
    std_frame_dim: FrameDim,
    frame_np_size: u32,
    ord_detection_output_meta_total_size: u32,
    ord_detection_output_meta: OrdDetectionOutputMeta,
}

use duct::cmd;
use hex;

pub fn pull_constants_from_settings_file() -> Vec<u8> {
    // load constants from file shared between Rust and Python

    let settings_stdout = cmd!("./settings_script.sh").stdout_capture().run().unwrap();
    // let settings : Settings = serde_json::from_str(&settings_stdout).unwrap();


    settings_stdout.stdout
    
}


pub fn get_detection_server_meta() -> DetectionServerMeta {

    let settings_stdout: Vec<u8> = pull_constants_from_settings_file();

    let length: usize = settings_stdout.len();
    let detection_server_meta = DetectionServerMeta {
        ip: String::from_utf8(settings_stdout[length-18..length-4].to_vec()).unwrap(),
        port: String::from_utf8( settings_stdout[length-4..length].to_vec() ).unwrap()
    };

    detection_server_meta

    
}



// By serializing and deserializing the detection output and frame data into structs, and designing the functions for that we can validate that the data conforms to the necessary format when we're sending or receiving from the detection server and when we're sending and receiving from other nodes


// pub fn serialize_detection_output() {};

// pub fn deserialize_detection_output() {};


// #[derive(Serialize, Deserialize, Debug)]
// pub struct FrameData {
//     detection_output: DetectionOutput,
// }


// pub fn serialize_frame_data() {};

// pub fn deserialize_frame_data() {};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct FramesMerkleTree {
// // includes FrameData. 
// }
