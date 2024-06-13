import type { TreeNode } from "./TreeNode.ts";

export function postorderTraversal(root: TreeNode | null): number[] {
	if (!root) {
		return [];
	}

	const result: number[] = [];
	result.push(...postorderTraversal(root.left), ...postorderTraversal(root.right), root.val);

	return result;
}
