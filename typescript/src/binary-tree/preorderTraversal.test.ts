import { describe, test, expect } from "bun:test";
import { preorderTraversal } from "./preorderTraversal";
import { TreeNode } from "./TreeNode";

describe("preorderTraversal", () => {
	test("Example test case", () => {
		const root = new TreeNode(1, new TreeNode(2), new TreeNode(3));
		expect(preorderTraversal(root)).toEqual([1, 2, 3]);
	});

	test("Empty tree", () => {
		expect(preorderTraversal(null)).toEqual([]);
	});

	test("Single node", () => {
		const root = new TreeNode(1);
		expect(preorderTraversal(root)).toEqual([1]);
	});

	test("Left child only", () => {
		const root = new TreeNode(1, new TreeNode(2));
		expect(preorderTraversal(root)).toEqual([1, 2]);
	});

	test("Right child only", () => {
		const root = new TreeNode(1, null, new TreeNode(3));
		expect(preorderTraversal(root)).toEqual([1, 3]);
	});

	test("Big tree", () => {
		const root = new TreeNode(1,
			new TreeNode(2,
				new TreeNode(4),
				new TreeNode(5)
			),
			new TreeNode(3,
				new TreeNode(6),
				new TreeNode(7)
			)
		);

		expect(preorderTraversal(root)).toEqual([1, 2, 4, 5, 3, 6, 7]);
	});
});
