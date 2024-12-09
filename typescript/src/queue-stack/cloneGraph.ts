export class _Node {
	val: number;
	neighbors: _Node[];

	constructor(val?: number, neighbors?: _Node[]) {
		this.val = (val === undefined ? 0 : val);
		this.neighbors = (neighbors === undefined ? [] : neighbors);
	}
}

// Recursive dfs approach.
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
		.filter((neighbor) => neighbor !== null);

	return new_node;
}

// Iterative dfs approach.
export function cloneGraphStack(node: _Node | null): _Node | null {
	if (!node) {
		return null;
	}

	const map: Map<_Node, _Node> = new Map();
	const stack: _Node[] = [node];

	while (stack.length) {
		const og = stack.pop()!;
		const copy = map.get(og) ?? new _Node(og.val);
		map.set(og, copy);

		og.neighbors.forEach(neighbor => {
			let neighbor_copy = map.get(neighbor);

			if (neighbor_copy) {
				copy.neighbors.push(neighbor_copy);
				return;
			}

			neighbor_copy = new _Node(neighbor.val);
			copy.neighbors.push(neighbor_copy);
			map.set(neighbor, neighbor_copy);
			stack.push(neighbor);
		});
	}

	return map.get(node)!;
}
