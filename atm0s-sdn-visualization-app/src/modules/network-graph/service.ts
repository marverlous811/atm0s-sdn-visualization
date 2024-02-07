import ky from 'ky'

export enum ConnectionStatus {
  CONNECTED = 'CONNECTED',
  DISCONNECTED = 'DISCONNECTED',
}

export enum ConnectionDirection {
  OUTGOING = 0,
  INCOMING = 1,
}

export type NetworkNodeConnection = {
  id: number
  node_id: number
  addr: string
  status: ConnectionStatus
  direction: ConnectionDirection
  last_updated_at: number
  metric: {
    latency: number
    bandwidth: number
    loss_percent: number
  }
}

export type NetworkNodeTransport = {
  id: number
  addr: string
  connections: Array<NetworkNodeConnection>
}

export type NetworkNodeData = {
  node_id: number
  transports: Array<NetworkNodeTransport>
}

export async function getNetworkNodes(): Promise<NetworkNodeData[]> {
  try {
    const res = await ky
      .get('http://localhost:8080/nodes', {
        headers: {
          'content-type': 'application/json',
        },
      })
      .json<{ nodes: NetworkNodeData[] }>()

    return res.nodes
  } catch (e) {
    console.error(`error when get network graph data`, e)
    throw e
  }
}
