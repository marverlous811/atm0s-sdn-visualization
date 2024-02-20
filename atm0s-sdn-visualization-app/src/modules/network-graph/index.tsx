import { SigmaContainer, useSigma } from '@react-sigma/core'
import '@react-sigma/core/lib/react-sigma.min.css'
import {
  INetworkGraphAction,
  NetworkGraphData,
  useNetworkdataStore,
} from './store'
import { useEffect } from 'react'
import { MultiDirectedGraph } from 'graphology'

type LoadGraphProps = {
  store: NetworkGraphData & INetworkGraphAction
}

const LoadGraph = (props: LoadGraphProps) => {
  const store = props.store

  const sigma = useSigma()
  const graph = sigma.getGraph()

  useEffect(() => {
    store.fetch()
    let interval = setInterval(() => {
      store.fetch()
    }, 5000)

    return () => {
      clearInterval(interval)
    }
  }, [])

  useEffect(() => {
    for (let node of [...store.nodeMap.values()]) {
      graph.addNode(node.id, {
        ...node,
      })
    }

    for (let edge of [...store.edgeMap.values()]) {
      graph.addDirectedEdgeWithKey(edge.key, edge.from, edge.to, {
        ...edge,
      })
    }

    console.log(graph.edges())
    return () => {
      graph.clear()
    }
  }, [graph, store])

  return <></>
}

export const NetworkGraph = () => {
  const store = useNetworkdataStore()
  return (
    <SigmaContainer
      style={{ height: '1000px', width: '1000px' }}
      settings={{
        defaultEdgeType: 'arrow',
        renderEdgeLabels: true,
      }}
      graph={MultiDirectedGraph}
    >
      <LoadGraph store={store} />
    </SigmaContainer>
  )
}
