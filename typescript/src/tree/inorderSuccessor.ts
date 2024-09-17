import type { TreeNode } from "./TreeNode.ts";

function inorderSuccessor(root: TreeNode | null, p: TreeNode | null): TreeNode | null {
	if (!root) {
		return null;
	}

	const stack: TreeNode[] = [];
	let curr: TreeNode | null = root;

	while (curr || stack.length) {
		while (curr) {
			stack.push(curr);
			curr = curr.left;
		}

		curr = stack.pop()!;

		if (curr == p) {
			// If the current node has a right child, then the successor is the leftmost node in the right subtree.
			if (curr.right) {
				let leftmost = curr.right;
				while (leftmost.left) {
					leftmost = leftmost.left;
				}

				return leftmost;
			}

			return stack.pop() ?? null;
		}

		curr = curr.right;
	}

	return null;
}

export { inorderSuccessor };
