import type { TreeNode } from "./TreeNode.ts";

export function postorderTraversal(root: TreeNode | null): number[] {
	if (!root) {
		return [];
	}

	return [...postorderTraversal(root.left), ...postorderTraversal(root.right), root.val];
}
