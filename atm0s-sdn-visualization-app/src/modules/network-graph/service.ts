import ky from 'ky'
import { Config } from '../../common'

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
  protocol: number
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

export type NetworkNodeData = {
  id: number
  addr: string
  last_ping_ts: number
  conns: Array<NetworkNodeConnection>
}

export async function getNetworkNodes(): Promise<NetworkNodeData[]> {
  try {
    const res = await ky
      .get(`${Config.API_ENDPOINT}/nodes`, {
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
