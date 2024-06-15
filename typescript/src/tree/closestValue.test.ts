import { test, expect } from "bun:test";
import { closestValue } from "./closestValue";
import { TreeNode } from "./TreeNode";

test("closestValue", () => {
	const tree = new TreeNode(4);
	tree.left = new TreeNode(2);
	tree.left.left = new TreeNode(1);
	tree.left.right = new TreeNode(3);
	tree.right = new TreeNode(5);

	expect(closestValue(tree, 3.714286)).toBe(4);
	expect(closestValue(tree, 3.5)).toBe(3);
});
