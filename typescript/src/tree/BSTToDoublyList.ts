import { TreeNode } from "./TreeNode.ts";

// https://leetcode.com/problems/convert-binary-search-tree-to-sorted-doubly-linked-list
// Dump tree into sorted array using LNR (in-order) traverse and then rearrange left and right pointers.
function treeToDoublyList(node: TreeNode | null): TreeNode | null {
	const sorted: TreeNode[] = [];
	const stack: TreeNode[] = [];

	while (node || stack.length) {
		while (node) {
			stack.push(node);
			node = node.left;
		}

		sorted.push(stack.pop()!);
		node = sorted[sorted.length - 1].right;
	}

	for (let i = 0; i < sorted.length; i++) {
		sorted[i].right = sorted[(i + 1) % sorted.length];
		sorted[i].left = sorted[(i - 1 + sorted.length) % sorted.length];
	}

	return sorted[0] ?? null;
}

export { treeToDoublyList };
