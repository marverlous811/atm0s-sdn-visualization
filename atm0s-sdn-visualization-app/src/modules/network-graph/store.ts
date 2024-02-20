import { create } from 'zustand'
import {
  ConnectionDirection,
  ConnectionStatus,
  NetworkNodeData,
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
  key: string
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
  upsertNode(node: NetworkNodeData): void
  fetch(): Promise<void>
}

export const useNetworkdataStore = create<
  NetworkGraphData & INetworkGraphAction
>((set, get) => ({
  nodeMap: new Map(),
  edgeMap: new Map(),
  upsertNode: (node: NetworkNodeData) => {
    const { nodeMap, edgeMap } = get()
    if (nodeMap.has(node.id)) return
    nodeMap.set(node.id, {
      id: node.id,
      label: node.addr,
      x: randomIntFromInterval(300, 700),
      y: randomIntFromInterval(300, 700),
      size: 15,
    })

    for (const conn of node.conns) {
      if (conn.direction != ConnectionDirection.INCOMING) continue
      const edgeKey = `${node.id}-${conn.node_id}-${conn.id}`
      let edge = edgeMap.get(`${edgeKey}`)
      if (!edge) {
        edge = {
          key: edgeKey,
          from: node.id,
          to: conn.node_id,
          label: `${conn.protocol}`,
          color: conn.status === ConnectionStatus.CONNECTED ? 'green' : 'red',
        }
      } else {
        edge = {
          ...edge,
          color: conn.status === ConnectionStatus.CONNECTED ? 'green' : 'red',
        }
      }
      edgeMap.set(`${edgeKey}`, edge)
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
      upsertNode(node)
    }
  },
}))
