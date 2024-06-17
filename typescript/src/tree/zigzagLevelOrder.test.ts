import { test, expect } from "bun:test";
import { zigzagLevelOrder } from "./zigzagLevelOrder";
import { TreeNode } from "./TreeNode";

test("zigzag level order #1", () => {
	const root = new TreeNode(3);
	root.left = new TreeNode(9);
	root.right = new TreeNode(20);
	root.right.left = new TreeNode(15);
	root.right.right = new TreeNode(7);
	expect(zigzagLevelOrder(root)).toStrictEqual([[3], [20, 9], [15, 7]]);
});

test("zigzag level order #2", () => {
	const root = new TreeNode(1);
	root.left = new TreeNode(2);
	root.left.left = new TreeNode(4);
	root.right = new TreeNode(3);
	root.right.right = new TreeNode(5);
	expect(zigzagLevelOrder(root)).toStrictEqual([[1], [3, 2], [4, 5]]);
});
