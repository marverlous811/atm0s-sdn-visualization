import ky from 'https://esm.sh/ky'

const connectionNodes = new Map()

const nodes = new vis.DataSet()
const edges = new vis.DataSet()

const fetch = () => {
    return ky
      .get(`/nodes`, {
        headers: {
          'content-type': 'application/json',
        },
      })
      .json()
}

const updateDataset = (data) => {
    const nodeToAdd = []
    const edgeToAdd = []
    const edgeToUpdate = []
    for(let node of data) {
        const oldNode = nodes.get(node.id)
        if(!oldNode) {
            nodeToAdd.push({id: node.id, label: `Node ${node.id}`})
        }

        for(let conn of node.conns) {
            // skip if outgoing connection
            if(conn.direction != 1) continue
            const edgeId = `${node.id}-${conn.node_id}-${conn.id}`
            const edge = edges.get(edgeId)
            if(edge) {
                edgeToUpdate.push({...edge, color: conn.status === 'CONNECTED' ? 'green' : 'red'})
            } else {
                edgeToAdd.push({id: edgeId, to: conn.node_id, from: node.id, arrows: {
                    from: true
                }, color: conn.status === 'CONNECTED' ? 'green' : 'red', label: `protocol: ${conn.protocol}, ping: ${conn.metric.latency}ms`})
            }
        }
    }
    if(nodeToAdd.length > 0) nodes.add(nodeToAdd)
    if(edgeToAdd.length > 0) edges.add(edgeToAdd)
    if(edgeToUpdate.length > 0) edges.update(...edgeToUpdate)
}

const load = async () => {
    const data = await fetch()
    updateDataset(data.nodes)
    setTimeout(load, 5000)
}



const container = document.getElementById("network-visualization");
const data = {
    nodes: nodes,
    edges: edges,
};
const options = {};
const network = new vis.Network(container, data, options);
load()