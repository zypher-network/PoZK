pub use ethers::types::Address;
use std::str::FromStr;

const POZK_PLAYER_BINARY: [u8; 8] = [112, 111, 122, 107, 58, 0, 6, 58]; // "pozk:06:"

pub enum TextMessage {
    Started,
    Over,
    ConnectPlayer(Address),
    ConnectViewer(Address),
    ClosePlayer(Address),
    CloseViewer(Address),
    Player(Address, String),
    Broadcast(String),
}

pub enum BinaryMessage {
    Player(Address, Vec<u8>),
    Broadcast(Vec<u8>),
}

impl TextMessage {
    pub fn encode(self) -> String {
        match self {
            TextMessage::Started => "pozk:00:".to_owned(),
            TextMessage::Over => "pozk:01:".to_owned(),
            TextMessage::ConnectPlayer(peer) => format!("pozk:02:{:?}", peer),
            TextMessage::ConnectViewer(peer) => format!("pozk:03:{:?}", peer),
            TextMessage::ClosePlayer(peer) => format!("pozk:04:{:?}", peer),
            TextMessage::CloseViewer(peer) => format!("pozk:05:{:?}", peer),
            TextMessage::Player(peer, text) => format!("pozk:06:{:?}:{}", peer, text),
            TextMessage::Broadcast(text) => text,
        }
    }

    pub fn decode(mut text: String) -> TextMessage {
        if text.len() < 8 {
            return TextMessage::Broadcast(text);
        }
        let dp = Address::zero();
        let real = text.split_off(8);
        match text.as_str() {
            "pozk:00:" => TextMessage::Started,
            "pozk:01:" => TextMessage::Over,
            "pozk:02:" => TextMessage::ConnectPlayer(Address::from_str(&real).unwrap_or(dp)),
            "pozk:03:" => TextMessage::ConnectViewer(Address::from_str(&real).unwrap_or(dp)),
            "pozk:04:" => TextMessage::ClosePlayer(Address::from_str(&real).unwrap_or(dp)),
            "pozk:05:" => TextMessage::CloseViewer(Address::from_str(&real).unwrap_or(dp)),
            "pozk:06:" => {
                if real.len() < 43 {
                    TextMessage::Broadcast(text + &real)
                } else {
                    let peer = Address::from_str(&real[0..42]).unwrap_or(dp);
                    TextMessage::Player(peer, real[43..].to_owned())
                }
            }
            _ => TextMessage::Broadcast(text + &real),
        }
    }
}

impl BinaryMessage {
    pub fn encode(self) -> Vec<u8> {
        match self {
            BinaryMessage::Player(peer, data) => {
                let mut new_data = POZK_PLAYER_BINARY.to_vec(); // 8
                new_data.extend(peer.to_fixed_bytes()); // 20
                new_data.push(58); // ":" 1
                new_data.extend(data);
                new_data
            }
            BinaryMessage::Broadcast(data) => data,
        }
    }

    pub fn decode(mut data: Vec<u8>) -> BinaryMessage {
        if data.len() > 28 && &data[0..8] == &POZK_PLAYER_BINARY {
            let peer = Address::from_slice(&data[8..28]);
            let real = data.split_off(29);
            BinaryMessage::Player(peer, real)
        } else {
            BinaryMessage::Broadcast(data)
        }
    }
}
