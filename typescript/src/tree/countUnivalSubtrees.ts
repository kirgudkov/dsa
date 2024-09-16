import type { TreeNode } from "./TreeNode.ts";

function countUnivalSubtrees(root: TreeNode | null): number {
	return count(root)[0];
}

function count(node: TreeNode | null): [number, boolean] /* [count, isSubtreeUnival] */ {
	if (!node) {
		// Base case: null node forms a unival subtree with count 0;
		// This will sufficiently turn leaf nodes into valid unival subtrees with count 1;
		return [0, true];
	}

	const [leftCount, isLeftUnival] = count(node.left);
	const [rightCount, isRightUnival] = count(node.right);

	const isLeftNullOrMatchesRoot = !node.left || node.left.val == node.val;
	const isRightNullOrMatchesRoot = !node.right || node.right.val == node.val;

	if (isLeftUnival && isRightUnival && isLeftNullOrMatchesRoot && isRightNullOrMatchesRoot) {
		return [leftCount + rightCount + 1, true];
	}

	return [leftCount + rightCount, false];
}

export { countUnivalSubtrees };
