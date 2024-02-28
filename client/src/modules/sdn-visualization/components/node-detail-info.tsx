import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { ConnectionStatus, NetworkNode } from '../entity'
import { BsCaretLeftFill, BsCaretRightFill } from 'react-icons/bs'

export type NodeDetailInfoProps = {
  node: NetworkNode
}

export const NodeDetailInfo = (props: NodeDetailInfoProps) => {
  const { node } = props
  return (
    <>
      <div>
        <Card>
          <CardHeader>
            <CardTitle>Node: {node.id}</CardTitle>
            <CardDescription>Addr: {node.addr}</CardDescription>
          </CardHeader>
          <CardContent>
            <CardTitle>List Connection</CardTitle>
            {node.conns.map((conn) => {
              return (
                <>
                  <Card key={conn.id}>
                    <CardHeader>
                      <CardTitle>
                        <div className="flex flex-row">
                          <div>
                            {conn.direction === 0 ? (
                              <BsCaretLeftFill
                                color={
                                  conn.status === ConnectionStatus.CONNECTED
                                    ? 'green'
                                    : 'red'
                                }
                              />
                            ) : (
                              <BsCaretRightFill
                                color={
                                  conn.status === ConnectionStatus.CONNECTED
                                    ? 'green'
                                    : 'red'
                                }
                              />
                            )}
                          </div>
                          <div>Node: {conn.node_id}</div>
                        </div>
                      </CardTitle>
                    </CardHeader>
                    <CardContent>
                      <CardDescription>addr: {conn.addr}</CardDescription>
                      <CardDescription>
                        ping: {conn.metric.latency}ms
                      </CardDescription>
                      <CardDescription>
                        spd: {conn.metric.bandwidth}kbps
                      </CardDescription>
                      <CardDescription>
                        loss: {conn.metric.loss_percent}%
                      </CardDescription>
                    </CardContent>
                  </Card>
                </>
              )
            })}
          </CardContent>
        </Card>
      </div>
    </>
  )
}
