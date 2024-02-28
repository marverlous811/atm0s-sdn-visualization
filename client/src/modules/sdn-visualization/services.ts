import ky from 'ky'
import { Config } from '@/common'
import { NetworkNode } from './entity'

export function fetchGraphNode(): Promise<{ nodes: NetworkNode[] }> {
  return ky
    .get(`${Config.API_ENDPOINT}/api/nodes`, {
      headers: {
        'content-type': 'application/json',
      },
    })
    .json()
}

export async function getNodeDetail(
  id: number,
): Promise<NetworkNode | undefined> {
  try {
    const res = await ky
      .get(`${Config.API_ENDPOINT}/api/nodes/${id}`, {
        headers: {
          'content-type': 'application/json',
        },
      })
      .json<NetworkNode>()
    return res
  } catch (e) {
    return undefined
  }
}
