import type { TreeNode } from "./TreeNode.ts";

// Recursive approach;
//
// - All values on the left subtree of a node should be less than the value of the node.
// - All values on the right subtree of a node should be greater than the value of the node.
// - Both the left and right subtrees must also be binary search trees
export function isValidBST(node: TreeNode | null): boolean {
	const isValid = (node: TreeNode | null, min: number, max: number): boolean => {
		if (!node) {
			return true;
		}

		if (node.val <= min || node.val >= max) {
			return false;
		}

		return isValid(node.left, min, node.val) && isValid(node.right, node.val, max);
	};

	return isValid(node, -Infinity, Infinity);
}
