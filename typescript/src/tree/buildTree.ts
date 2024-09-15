import { TreeNode } from "./TreeNode.ts";

// Time complexity: O(n^2) because of indexOf in each recursive call
// Space complexity: O(n) because of the recursive call stack
// We can improve the time complexity to O(n) by using a hashmap to store the index of each element in the inorder array
//
// Building tree from preorder is almost the same:
// we only need to change the order of the recursive calls and do shift instead of pop.
// (shift() is also adds O(n) time complexity to the solution, so we probably can optimize it by using a pointer or a linked list)
export function buildTree(inorder: number[], postorder: number[]): TreeNode | null {
	if (inorder.length == 0) {
		return null;
	}

	const root = new TreeNode(postorder.pop()!);
	const idx = inorder.indexOf(root.val);

	// Right subtree goes first because of mutable postorder array, it should be processed first
	root.right = buildTree(inorder.slice(idx + 1), postorder);
	root.left = buildTree(inorder.slice(0, idx), postorder);

	return root;
}
