import type { TreeNode } from "./TreeNode.ts";

function deleteNode(node: TreeNode | null, key: number): TreeNode | null {
	if (!node) {
		return null;
	}

	if (key < node.val) {
		node.left = deleteNode(node.left, key);
	} else if (key > node.val) {
		node.right = deleteNode(node.right, key);
	} else {
		if (!node.left) {
			return node.right;
		}

		if (!node.right) {
			return node.left;
		}

		node.val = leftmost(node.right);
		node.right = deleteNode(node.right, node.val);
	}

	return node;
}

const leftmost = (node: TreeNode): number => {
	let left = node;

	while (left.left) {
		left = left.left;
	}

	return left.val;
};

export { deleteNode };
