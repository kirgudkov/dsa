import { test, expect } from "bun:test";
import { buildTree } from "./buildTree.ts";
import { TreeNode } from "./TreeNode.ts";

test("Few elements", () => {
	const inorder = [9, 3, 15, 20, 7];
	const postorder = [9, 15, 7, 20, 3];
	const expected =
		new TreeNode(3,
			new TreeNode(9),
			new TreeNode(20,
				new TreeNode(15),
				new TreeNode(7)
			)
		);
	expect(buildTree(inorder, postorder)).toEqual(expected);
});

test("Single element", () => {
	const inorder = [1];
	const postorder = [1];
	const expected = new TreeNode(1);
	expect(buildTree(inorder, postorder)).toEqual(expected);
});
