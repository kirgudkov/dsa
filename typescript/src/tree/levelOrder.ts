import type { TreeNode } from "./TreeNode.ts";

export function levelOrder(root: TreeNode | null): number[][] {
	if (!root) {
		return [];
	}

	const result: number[][] = [];
	const queue: TreeNode[] = [root];

	while (queue.length) {
		const level: number[] = [];

		for (let i = queue.length; i > 0; i--) {
			const node = queue.shift()!;
			level.push(node.val);

			node.left && queue.push(node.left);
			node.right && queue.push(node.right);
		}

		result.push(level);
	}

	return result;
}
