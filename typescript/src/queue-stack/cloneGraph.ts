export class _Node {
	val: number;
	neighbors: _Node[];

	constructor(val?: number, neighbors?: _Node[]) {
		this.val = (val === undefined ? 0 : val);
		this.neighbors = (neighbors === undefined ? [] : neighbors);
	}
}

// Recursive approach.
export function cloneGraphRecursive(node: _Node | null, visited: Map<number, _Node> = new Map()): _Node | null {
	if (!node) {
		return null;
	}

	if (visited.has(node.val)) {
		return visited.get(node.val)!;
	}

	const new_node = new _Node(node.val);
	visited.set(node.val, new_node);

	new_node.neighbors = node.neighbors
		.map(neighbor => cloneGraphRecursive(neighbor, visited))
		.filter(Boolean) as _Node[];

	return new_node;
}

// Stack approach.
export function cloneGraphStack(node: _Node | null): _Node | null {
	if (!node) {
		return null;
	}

	const map: Map<_Node, _Node> = new Map();
	const stack: _Node[] = [node];

	while (stack.length !== 0) {
		const original_node = stack.pop()!;
		const cloned_node = map.get(original_node) ?? new _Node(original_node.val);
		map.set(original_node, cloned_node);

		original_node.neighbors.forEach(neighbor => {
			let cloned_neighbor = map.get(neighbor);

			if (cloned_neighbor) {
				cloned_node.neighbors.push(cloned_neighbor);
				return;
			}

			cloned_neighbor = new _Node(neighbor.val);
			cloned_node.neighbors.push(cloned_neighbor);
			map.set(neighbor, cloned_neighbor);
			stack.push(neighbor);
		});
	}

	return map.get(node) ?? null;
}
