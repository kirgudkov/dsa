import type { TreeNode } from "./TreeNode.ts";

function zigzagLevelOrder(root: TreeNode | null): number[][] {
	if (!root) {
		return [];
	}

	const result: number[][] = [];
	const q: TreeNode[] = [root];

	let depth = 0;

	while (q.length) {
		let n = q.length;
		const level: number[] = [];

		while (n > 0) {
			const node = q.shift()!;

			if (depth % 2 === 0) {
				level.push(node.val);
			} else {
				level.unshift(node.val);
			}

			node.left && q.push(node.left);
			node.right && q.push(node.right);

			n--;
		}

		result.push(level);
		depth++;
	}

	return result;
}

export { zigzagLevelOrder };
