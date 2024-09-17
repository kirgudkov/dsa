import type { TreeNode } from "./TreeNode.ts";

export function preorderTraversal(root: TreeNode | null): number[] {
	if (!root) {
		return [];
	}

	return [root.val, ...preorderTraversal(root.left), ...preorderTraversal(root.right)];
}
