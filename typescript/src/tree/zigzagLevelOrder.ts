import type { TreeNode } from "./TreeNode.ts";

function zigzagLevelOrder(root: TreeNode | null): number[][] {
	if (!root) {
		return [];
	}

	const result: number[][] = [];
	const queue: [TreeNode, number][] = [[root, 0]];

	while (queue.length) {
		const level: number[] = [];

		for (let i = queue.length; i > 0; i--) {
			const [node, depth] = queue.shift()!;

			if (depth % 2 == 0) {
				level.push(node.val);
			} else {
				level.unshift(node.val);
			}

			node.left && queue.push([node.left, depth + 1]);
			node.right && queue.push([node.right, depth + 1]);
		}

		result.push(level);
	}

	return result;
}

export { zigzagLevelOrder };
