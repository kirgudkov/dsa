import type { TreeNode } from "./TreeNode.ts";

function deleteNode(root: TreeNode | null, key: number): TreeNode | null {
	if (!root) {
		return null;
	}

	if (key < root.val) {
		root.left = deleteNode(root.left, key);
	} else if (key > root.val) {
		root.right = deleteNode(root.right, key);
	} else {
		if (!root.left) {
			return root.right;
		}

		if (!root.right) {
			return root.left;
		}

		root.val = leftmost(root.right);
		root.right = deleteNode(root.right, root.val);
	}

	return root;
}

const leftmost = (root: TreeNode): number => {
	let node = root;

	while (node.left) {
		node = node.left;
	}

	return node.val;
};

export { deleteNode };
