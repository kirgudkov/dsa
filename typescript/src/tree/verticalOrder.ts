import type { TreeNode } from "./TreeNode.ts";

function verticalOrder(root: TreeNode | null): number[][] {
	if (!root) {
		return [];
	}

	const map = new Map<number, number[]>();
	const queue: [TreeNode, number][] = [[root, 0]];

	while (queue.length) {
		for (let i = queue.length; i > 0; i--) {
			const [node, index] = queue.shift()!;

			if (map.has(index)) {
				map.get(index)!.push(node.val);
			} else {
				map.set(index, [node.val]);
			}

			node.left && queue.push([node.left, index - 1]);
			node.right && queue.push([node.right, index + 1]);
		}
	}

	return Array.from(map.keys())
		.sort((a, b) => a - b)
		.map(key => map.get(key)!);
}

export { verticalOrder };
