import type { TreeNode } from "./TreeNode.ts";

// https://leetcode.com/problems/closest-binary-search-tree-value
// DFS approach: perform incomplete inorder traversal.
// Basically, we're observing "virtual" sorted array using two pointers: current and previous;
// Time complexity: O(H + k), where k is an index of the closest element
// Space complexity: O(H), where H is a height of the tree
function closestValue(node: TreeNode | null, target: number): number {
	if (!node) {
		return -1;
	}

	let prev = -Infinity;
	const stack: TreeNode[] = [];

	while (node || stack.length) {
		while (node) {
			stack.push(node);
			node = node.left;
		}

		node = stack.pop()!;

		if (node.val > target) { // prev here guaranteed to be less/equal to target
			return Math.abs(target - prev) <= Math.abs(target - node.val) // return the closest
				? prev
				: node.val;
		}

		prev = node.val;
		node = node.right;
	}

	return prev;
}

export { closestValue };
