import type { TreeNode } from "./TreeNode.ts";

export function maxDepth(root: TreeNode | null): number {
	if (!root) {
		return 0;
	}

	return Math.max(maxDepth(root.left) + 1, maxDepth(root.right) + 1);
}
