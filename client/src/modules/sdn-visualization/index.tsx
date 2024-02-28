import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { NetworkGraphController } from './graph.controller'
import { useEffect, useRef, useState } from 'react'
import * as service from './services'
import { NetworkNode } from './entity'
import { NodeDetailInfo } from './components/node-detail-info'

export const SdnVisualization = () => {
  const controller = new NetworkGraphController()
  const graphDom = useRef<HTMLDivElement>(null)
  const [selectedNode, setSelectedNode] = useState<NetworkNode | undefined>(
    undefined,
  )
  const loadGraph = async () => {
    const res = await service.fetchGraphNode()
    controller.updateNetwork(res.nodes)
  }
  const getNodeDetail = async (nodeId: number) => {
    const node = await service.getNodeDetail(nodeId)
    setSelectedNode(node)
  }
  useEffect(() => {
    let interval: any = null
    if (graphDom.current) {
      console.log('init network graph...')
      let network = controller.initNetwork(graphDom.current)
      network.on('click', (e) => {
        console.log('on network clicked...', e)
        if (e.nodes && e.nodes.length > 0) {
          getNodeDetail(e.nodes[0])
        } else {
          setSelectedNode(undefined)
        }
      })
      interval = setInterval(loadGraph, 5000)
      loadGraph()
    }

    return () => {
      if (interval) clearInterval(interval)
    }
  }, [])
  return (
    <>
      <div className="w-full h-full flex flex-col">
        <div className="flex-1 space-y-4 p-8 pt-6 h-[15%]">
          <div className="flex items-center justify-between space-y-2">
            <h2 className="text-3xl font-bold tracking-tight">
              Sdn Visualization
            </h2>
          </div>
        </div>
        <div className="space-y-4 h-[85%]">
          <div className="flex flex-row p-8 pt-6 h-full">
            <div className="w-3/4 h-full p-3">
              <Card className="w-full h-full">
                <CardHeader className="h-[10%]">
                  <CardTitle>Network nodes Graph</CardTitle>
                </CardHeader>
                <CardContent className="pl-2 h-[90%]">
                  <div className="w-full h-full" ref={graphDom}></div>
                </CardContent>
              </Card>
            </div>
            <div className="w-1/4 h-full p-3">
              <Card className="h-full w-full">
                <CardHeader>
                  <CardTitle>Network nodes Detail</CardTitle>
                </CardHeader>
                <CardContent className="pl-2">
                  {selectedNode ? <NodeDetailInfo node={selectedNode} /> : null}
                </CardContent>
              </Card>
            </div>
          </div>
        </div>
      </div>
    </>
  )
}
