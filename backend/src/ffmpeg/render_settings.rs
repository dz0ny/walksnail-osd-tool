use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::ffmpeg::{Codec, Encoder};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Container {
    MOV,
    MKV,
    MP4,
}
impl Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Container::MOV => write!(f, ".mov"),
            Container::MP4 => write!(f, ".mp4"),
            Container::MKV => write!(f, ".mkv"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RenderSettings {
    pub encoder: Encoder,
    pub selected_encoder_idx: usize,
    pub show_undetected_encoders: bool,
    pub bitrate_mbps: u32,
    pub upscale: bool,
    pub use_chroma_key: bool,
    pub chroma_key: [f32; 3],
    pub fps30: bool,
    pub container: Container,
    pub selected_container_idx: usize,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            encoder: Encoder {
                name: "libx264".to_string(),
                codec: Codec::H264,
                hardware: false,
                detected: false,
            },
            selected_encoder_idx: 0,
            show_undetected_encoders: false,
            bitrate_mbps: 40,
            upscale: false,
            use_chroma_key: false,
            chroma_key: [1.0 / 255.0, 177.0 / 255.0, 64.0 / 255.0],
            fps30: false,
            container: Container::MOV,
            selected_container_idx: 0,
        }
    }
