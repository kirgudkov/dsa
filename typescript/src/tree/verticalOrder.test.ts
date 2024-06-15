import { test, expect } from "bun:test";
import { verticalOrder } from "./verticalOrder";
import { TreeNode } from "./TreeNode";

test("verticalOrder", () => {
	const tree = new TreeNode(3);
	tree.left = new TreeNode(9);
	tree.left.left = new TreeNode(4);
	tree.left.right = new TreeNode(0);
	tree.right = new TreeNode(8);
	tree.right.left = new TreeNode(1);
	tree.right.right = new TreeNode(7);

	expect(verticalOrder(tree)).toEqual([[4], [9], [3, 0, 1], [8], [7]]);
});

