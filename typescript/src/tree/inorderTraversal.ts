import type { TreeNode } from "./TreeNode.ts";

export function inorderTraversal(root: TreeNode | null): number[] {
	if (!root) {
		return [];
	}

	const result: number[] = [];
	result.push(...inorderTraversal(root.left), root.val, ...inorderTraversal(root.right));

	return result;
}
