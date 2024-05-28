import type { TreeNode } from "./TreeNode.ts";

export function levelOrder(root: TreeNode | null): number[][] {
	if (!root) {
		return [];
	}

	const result: number[][] = [];
	const queue: TreeNode[] = [root];

	while (queue.length) {
		const level: number[] = [];
		const queue_length = queue.length;

		for (let i = 0; i < queue_length; i++) {
			const node = queue.shift()!;
			level.push(node.val);

			node.left && queue.push(node.left);
			node.right && queue.push(node.right);
		}

		result.push(level);
	}

	return result;
}
