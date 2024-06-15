import type { TreeNode } from "./TreeNode.ts";

function verticalOrder(root: TreeNode | null): number[][] {
	if (!root) {
		return [];
	}

	const map = new Map<number, number[]>();
	const q: [TreeNode, number][] = [[root, 0]];

	while (q.length) {
		let n = q.length;

		while (n > 0) {
			const [node, index] = q.shift()!;

			if (map.has(index)) {
				map.get(index)!.push(node.val);
			} else {
				map.set(index, [node.val]);
			}

			node.left && q.push([node.left, index - 1]);
			node.right && q.push([node.right, index + 1]);

			n--;
		}
	}

	return Array.from(map.keys())
		.sort((a, b) => a - b)
		.map((key) => map.get(key)!);
}

export { verticalOrder };
