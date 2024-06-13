import type { TreeNode } from "./TreeNode.ts";

export function preorderTraversal(root: TreeNode | null): number[] {
	if (!root) {
		return [];
	}

	const result: number[] = [];
	result.push(root.val, ...preorderTraversal(root.left), ...preorderTraversal(root.right));

	return result;
}
