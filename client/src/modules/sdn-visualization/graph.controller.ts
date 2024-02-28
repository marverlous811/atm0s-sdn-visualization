import { DataSet } from 'vis-data'
import { Network } from 'vis-network'
import { ConnectionStatus, NetworkNode } from './entity'

export class NetworkGraphController {
  private _nodes = new DataSet()
  private _edges = new DataSet()
  constructor() {}

  initNetwork = (div: HTMLDivElement) => {
    const data = {
      nodes: this._nodes,
      edges: this._edges,
    } as any
    return new Network(div, data, {})
  }

  updateNetwork = (data: NetworkNode[]) => {
    const nodeToAdd = []
    const edgeToAdd = []
    const edgeToUpdate = []
    for (let node of data) {
      const oldNode = this._nodes.get(node.id)
      if (!oldNode) {
        nodeToAdd.push({ id: node.id, label: `Node ${node.id}` })
      }

      for (let conn of node.conns) {
        // skip if outgoing connection
        if (conn.direction != 1) continue
        const edgeId = `${node.id}-${conn.node_id}-${conn.id}`
        const edge = this._edges.get(edgeId)
        if (edge) {
          edgeToUpdate.push({
            ...edge,
            color: conn.status === ConnectionStatus.CONNECTED ? 'green' : 'red',
          })
        } else {
          edgeToAdd.push({
            id: edgeId,
            to: conn.node_id,
            from: node.id,
            arrows: {
              from: true,
            },
            color: conn.status === 'CONNECTED' ? 'green' : 'red',
            label: `protocol: ${conn.protocol}, ping: ${conn.metric.latency}ms`,
          })
        }
      }
    }
    if (nodeToAdd.length > 0) this._nodes.add(nodeToAdd)
    if (edgeToAdd.length > 0) this._edges.add(edgeToAdd)
    if (edgeToUpdate.length > 0) this._edges.update(edgeToUpdate)
  }
}
