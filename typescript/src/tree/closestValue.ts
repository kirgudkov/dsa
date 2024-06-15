import type { TreeNode } from "./TreeNode.ts";

// https://leetcode.com/problems/closest-binary-search-tree-value
// DFS approach: perform incomplete inorder traversal that stops when the target is between the current and previous node values
// Time complexity: O(H + k), where k is an index of the closest element
// Space complexity: O(H), where H is a height of the tree
function closestValue(root: TreeNode | null, target: number): number {
	if (!root) {
		return -1;
	}

	let pred = -Infinity;
	const stack: TreeNode[] = [];

	while (root || stack.length) {
		while (root) {
			stack.push(root);
			root = root.left;
		}

		root = stack.pop()!;

		if (pred <= target && target < root.val) {
			return [pred, root.val].reduce((prev, curr) =>
				Math.abs(target - prev) <= Math.abs(target - curr) ? prev : curr
			);
		}

		pred = root.val;
		root = root.right;
	}

	return pred;
}

export { closestValue };
