use std::{io::Write, net::TcpStream};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Message {
    pub stage: usize,
    pub cpu_x: f32,
    pub cpu_y: f32,
    pub cpu_damage: f32,
    pub opp_x: f32,
    pub opp_y: f32,
    pub opp_damage: f32,
}

impl Message {
    pub fn send_state(&mut self, stream: &mut TcpStream) -> std::io::Result<()> {
        let bytes = serde_json::to_vec(self).unwrap();
        stream.write_all(&bytes)?;
        Ok(())
    }
}
