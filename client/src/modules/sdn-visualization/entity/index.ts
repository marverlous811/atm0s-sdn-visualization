export enum ConnectionStatus {
  DISCONNECTED = 'DISCONNECTED',
  CONNECTED = 'CONNECTED',
}

export enum ConnectionDirection {
  OUTGOING = 0,
  INCOMING = 1,
}

export type ConnectionMetric = {
  latency: number // in milisec
  bandwidth: number // kps
  loss_percent: number
}

export type NetworkNode = {
  id: number
  addr: string
  last_ping_ts: string
  conns: Array<NetworkNodeConnection>
}

export type NetworkNodeConnection = {
  id: number
  node_id: number
  protocol: number
  addr: String
  metric: ConnectionMetric
  status: ConnectionStatus
  last_updated_at: number
  direction: ConnectionDirection
}
