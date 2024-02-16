import Graph from 'graphology'
import {
  ConnectionDirection,
  ConnectionStatus,
  getNetworkNodes,
} from './service'
import { useEffect, useState } from 'react'
import { useLoadGraph } from '@react-sigma/core'
import { useLayoutForceAtlas2 } from '@react-sigma/layout-forceatlas2'
import { randomIntFromInterval } from '../../util'

type GraphNode = {
  id: number
  label: string
}

type GraphEdge = {
  from: number
  to: number
  label: string
  status: ConnectionStatus
}

async function getNetworkGraph(): Promise<{
  nodes: GraphNode[]
  edges: GraphEdge[]
}> {
  const retval: {
    nodes: GraphNode[]
    edges: GraphEdge[]
  } = {
    nodes: [],
    edges: [],
  }

  const networkNodes = await getNetworkNodes()
  for (let node of networkNodes) {
    for (let transport of node.transports) {
      retval.nodes.push({
        id: transport.id,
        label: transport.addr,
      })
      for (let conn of transport.connections) {
        if (conn.direction != ConnectionDirection.INCOMING) continue
        retval.edges.push({
          from: transport.id,
          to: conn.id,
          label: `${conn.metric.latency}ms`,
          status: conn.status,
        })
      }
    }
  }

  return retval
}

export function useNetworkHook() {
  const loadGraph = useLoadGraph()
  const { assign } = useLayoutForceAtlas2()
  const [nodes, setNodes] = useState<GraphNode[]>([])
  const [edges, setEdges] = useState<GraphEdge[]>([])
  const fetchGraphData = async () => {
    const res = await getNetworkGraph()
    setNodes([...res.nodes])
    setEdges([...res.edges])
  }

  useEffect(() => {
    let interval = setInterval(() => {
      fetchGraphData()
    }, 5000)

    return () => {
      clearInterval(interval)
    }
  }, [])

  useEffect(() => {
    const graph = new Graph()
    for (let node of nodes) {
      graph.addNode(node.id, {
        x: randomIntFromInterval(300, 700),
        y: randomIntFromInterval(300, 700),
        size: 15,
        label: `${node.label}`,
      })
    }
    for (let edge of edges) {
      graph.addDirectedEdge(edge.from, edge.to, {
        label: edge.label,
        color: edge.status === ConnectionStatus.CONNECTED ? 'green' : 'red',
      })
    }
    loadGraph(graph)
    assign()
  }, [nodes, edges, loadGraph])

  return {}
}
