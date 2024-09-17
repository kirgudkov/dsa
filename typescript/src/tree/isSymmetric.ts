import type { TreeNode } from "./TreeNode.ts";

export function isSymmetric(root: TreeNode | null): boolean {
	if (!root) {
		return true;
	}

	// Apply level-order traversal to check if the tree is symmetric.
	// We will transform the tree into two-dimensional array where each level is a row
	// and then check if each row is a palindrome array.

	const queue: (TreeNode | null)[] = [root];

	while (queue.length) {
		const level: (number | null)[] = [];

		for (let i = queue.length; i > 0; i--) {
			const node = queue.shift();

			level.push(node?.val ?? null);

			if (node) {
				queue.push(node.left);
				queue.push(node.right);
			}
		}

		for (let i = 0, j = level.length - 1; i < j; i++, j--) {
			if (level[i] != level[j]) {
				return false;
			}
		}
	}

	return true;
}
