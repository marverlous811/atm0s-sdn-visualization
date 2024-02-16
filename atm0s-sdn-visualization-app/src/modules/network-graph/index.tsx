import { SigmaContainer } from '@react-sigma/core'
import { useNetworkHook } from './hooks'
import '@react-sigma/core/lib/react-sigma.min.css'

const LoadGraph = () => {
  const graph = useNetworkHook()

  return null
}

export const NetworkGraph = () => {
  return (
    <SigmaContainer
      style={{ height: '1000px', width: '1000px' }}
      settings={{
        defaultEdgeType: 'arrow',
        renderEdgeLabels: true,
      }}
    >
      <LoadGraph />
    </SigmaContainer>
  )
}
