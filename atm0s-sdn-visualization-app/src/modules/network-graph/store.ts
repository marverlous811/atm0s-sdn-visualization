import { create } from 'zustand'
import {
  ConnectionDirection,
  ConnectionStatus,
  NetworkNodeTransport,
  getNetworkNodes,
} from './service'
import { randomIntFromInterval } from '../../util'

type NetworkNode = {
  id: number
  label: string
  size: number
  x: number
  y: number
}

type NetworkEdge = {
  from: number
  to: number
  color: string
  label: string
}

export type NetworkGraphData = {
  nodeMap: Map<number, NetworkNode>
  edgeMap: Map<string, NetworkEdge>
}

export interface INetworkGraphAction {
  upsertNode(node: NetworkNodeTransport): void
  fetch(): Promise<void>
}

export const useNetworkdataStore = create<
  NetworkGraphData & INetworkGraphAction
>((set, get) => ({
  nodeMap: new Map(),
  edgeMap: new Map(),
  upsertNode: (node: NetworkNodeTransport) => {
    const { nodeMap, edgeMap } = get()
    if (nodeMap.has(node.id)) return
    nodeMap.set(node.id, {
      id: node.id,
      label: node.addr,
      x: randomIntFromInterval(300, 700),
      y: randomIntFromInterval(300, 700),
      size: 15,
    })

    for (const conn of node.connections) {
      if (conn.direction != ConnectionDirection.INCOMING) continue
      let edge = edgeMap.get(`${node.id}-${conn.id}`)
      if (!edge) {
        edge = {
          from: node.id,
          to: conn.id,
          label:
            conn.status !== ConnectionStatus.CONNECTED
              ? ''
              : `ping: ${conn.metric.latency}ms - spd: ${conn.metric.bandwidth}kbps - loss: ${conn.metric.loss_percent}%`,
          color: conn.status === ConnectionStatus.CONNECTED ? 'green' : 'red',
        }
      } else {
        edge = {
          ...edge,
          label:
            conn.status !== ConnectionStatus.CONNECTED
              ? ''
              : `ping: ${conn.metric.latency}ms - spd: ${conn.metric.bandwidth}kbps - loss: ${conn.metric.loss_percent}%`,
          color: conn.status === ConnectionStatus.CONNECTED ? 'green' : 'red',
        }
      }
      edgeMap.set(`${node.id}-${conn.id}`, edge)
    }

    set({
      nodeMap: nodeMap,
      edgeMap: edgeMap,
    })
  },
  fetch: async () => {
    const { upsertNode } = get()
    const networkNodes = await getNetworkNodes()
    for (let node of networkNodes) {
      for (let trans of node.transports) {
        upsertNode(trans)
      }
    }
  },
}))
