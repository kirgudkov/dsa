import { test, expect } from "bun:test";
import { lowestCommonAncestor } from "./lowestCommonAncestor.ts";
import { TreeNode } from "./TreeNode";

test("Perfect tree", () => {
	const root = new TreeNode(1);
	const n2 = new TreeNode(2);
	const n3 = new TreeNode(3);
	const n4 = new TreeNode(4);
	const n5 = new TreeNode(5);
	const n6 = new TreeNode(6);
	const n7 = new TreeNode(7);

	root.left = n2;
	root.right = n3;

	n2.left = n4;
	n2.right = n5;

	n3.left = n6;
	n3.right = n7;

	expect(lowestCommonAncestor(root, n2, n3)).toBe(root);
	expect(lowestCommonAncestor(root, n2, n5)).toBe(n2);
	expect(lowestCommonAncestor(root, n4, n5)).toBe(n2);
	expect(lowestCommonAncestor(root, n2, n7)).toBe(root);
});

test("Two nodes tree", () => {
	const root = new TreeNode(1);
	const n2 = new TreeNode(2);

	root.left = n2;

	expect(lowestCommonAncestor(root, n2, root)).toBe(root);
});
