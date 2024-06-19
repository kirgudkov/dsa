import type { TreeNode } from "./TreeNode.ts";

function countUnivalSubtrees(root: TreeNode | null): number {
	return count(root)[0];
}

// Returns tuple of [count, isSubtreeUnival]
function count(root: TreeNode | null): [number, boolean] {
	if (!root) {
		return [0, true];
	}

	const [leftCount, isLeftUnival] = count(root.left);
	const [rightCount, isRightUnival] = count(root.right);

	// This sufficiently covers the base case of a leaf node and the case when higher roots are unival:
	// for the leaf node it'll get two null subtrees with count 0 and both of them are unival, so it'll add 1 to the count.
	// for the higher roots, if both left and right subtrees are unival and the root value is the same as the left and right values, it'll add 1 to the count.
	if (isLeftUnival && isRightUnival) {
		if (
			(!root.left || root.left.val === root.val) &&
			(!root.right || root.right.val === root.val)
		) {
			return [leftCount + rightCount + 1, true];
		}
	}

	return [leftCount + rightCount, false];
}

export { countUnivalSubtrees };
