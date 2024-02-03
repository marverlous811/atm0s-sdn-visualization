pub enum ConnectionStatus {
    DISCONNECTED = 0,
    CONNECTED = 1,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConnectionMetric {
    pub latency: u16,      // in milisec
    pub bandwidth: u32,    // kps
    pub loss_percent: u32, // percentage of package loss
}
