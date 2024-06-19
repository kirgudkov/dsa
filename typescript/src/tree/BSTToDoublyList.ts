import { TreeNode } from "./TreeNode.ts";

// https://leetcode.com/problems/convert-binary-search-tree-to-sorted-doubly-linked-list
// Dump tree into sorted array using LNR (in-order) traverse and then rearrange left and right pointers.
function treeToDoublyList(root: TreeNode | null): TreeNode | null {
	if (!root) {
		return null;
	}

	const sorted: TreeNode[] = [];
	const stack: TreeNode[] = [];
	let curr: TreeNode | null = root;

	while (curr || stack.length) {
		while (curr) {
			stack.push(curr);
			curr = curr.left;
		}

		curr = stack.pop()!;
		sorted.push(curr);
		curr = curr.right;
	}

	for (let i = 0; i < sorted.length; i++) {
		// Magic that loops next_i back to 0 when "i" is the last index,
		// and prev_i back to the last index when "i" is the first index.
		const next_i = (i + 1) % sorted.length;
		const prev_i = (i - 1 + sorted.length) % sorted.length;

		sorted[i].right = sorted[next_i];
		sorted[i].left = sorted[prev_i];
	}

	return sorted[0];
}

export { treeToDoublyList };
