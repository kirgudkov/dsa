import { test, expect } from "bun:test";
import { isSymmetric } from "./isSymmetric.ts";
import { TreeNode } from "./TreeNode.ts";

test("Symmetric test", () => {
	const root = new TreeNode(1, new TreeNode(2, new TreeNode(3), new TreeNode(4)), new TreeNode(2, new TreeNode(4), new TreeNode(3)));

	expect(isSymmetric(root)).toBe(true);
});

test("Asymmetric test", () => {
	const root = new TreeNode(1, new TreeNode(2, null, new TreeNode(3)), new TreeNode(2, null, new TreeNode(3)));

	expect(isSymmetric(root)).toBe(false);
});
