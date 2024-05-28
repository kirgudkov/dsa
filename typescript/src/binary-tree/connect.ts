export class _Node {
	val: number;
	left: _Node | null;
	right: _Node | null;
	next: _Node | null;

	constructor(val?: number, left?: _Node, right?: _Node, next?: _Node) {
		this.val = (val === undefined ? 0 : val);
		this.left = (left === undefined ? null : left);
		this.right = (right === undefined ? null : right);
		this.next = (next === undefined ? null : next);
	}
}

// Use level order traversal to connect nodes at the same level.
// After traversing the tree, we will have something like this:
// [
//    [1],
//    [2, 3],
//    [4, 5, 6, 7]
// ]
// Each row represents a level in the tree. Iterate over each row and connect the nodes.
export function connect(root: _Node | null): _Node | null {
	if (!root) {
		return null;
	}

	const queue: _Node[] = [root];

	while (queue.length) {
		const level: _Node[] = [];
		const length = queue.length;

		for (let i = 0; i < length; i++) {
			const node = queue.shift()!;
			level.push(node);

			node.left && queue.push(node.left);
			node.right && queue.push(node.right);
		}

		if (level.length > 1) {
			let i = 1;
			let prev = level[i - 1];
			let current = level[i];

			while (current) {
				i++;

				prev.next = current;
				prev = current;
				current = level[i];
			}
		}
	}

	return root;
}
