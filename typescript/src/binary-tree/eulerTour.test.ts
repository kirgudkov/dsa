import { test, expect } from "bun:test";
import { eulerTour } from "./eulerTour.ts";
import { TreeNode } from "./TreeNode";

test("eulerWalk", () => {
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

	const visits = eulerTour(root, 0);

	expect(visits).toEqual([
		{ node: root, depth: 0 },
		{ node: n2, depth: 1 },
		{ node: n4, depth: 2 },
		{ node: n2, depth: 1 },
		{ node: n5, depth: 2 },
		{ node: n2, depth: 1 },
		{ node: root, depth: 0 },
		{ node: n3, depth: 1 },
		{ node: n6, depth: 2 },
		{ node: n3, depth: 1 },
		{ node: n7, depth: 2 },
		{ node: n3, depth: 1 },
		{ node: root, depth: 0 }
	]);
});
