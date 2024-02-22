use atm0s_sdn_identity::{ConnDirection, NodeId};
use serde::{Deserialize, Serialize};

pub fn generate_connection_id(protocol: u8, direction: ConnDirection, node_id: NodeId) -> u64 {
    return (node_id as u64) << 16 | (protocol as u64) << 8 | (direction.to_byte() as u64);
}

pub fn get_direction(id: u64) -> ConnDirection {
    match id as u8 {
        0 => ConnDirection::Outgoing,
        _ => ConnDirection::Incoming,
    }
}

pub fn get_protocol(id: u64) -> u8 {
    (id >> 8) as u8
}

pub fn get_node_id(id: u64) -> NodeId {
    (id >> 16) as NodeId
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_connection_id() {
        let protocol: u8 = 1;
        let direction = ConnDirection::Outgoing;
        let node_id: NodeId = 12345;

        let result = generate_connection_id(protocol, direction, node_id);

        assert_eq!(get_direction(result), ConnDirection::Outgoing);
        assert_eq!(get_protocol(result), protocol);
        assert_eq!(get_node_id(result), node_id);
    }
}
