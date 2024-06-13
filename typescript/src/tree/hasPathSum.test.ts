import { test, expect } from "bun:test";
import { hasPathSum } from "./hasPathSum.ts";
import { TreeNode } from "./TreeNode.ts";

test("Has sum", () => {
	const root = new TreeNode(5,
		new TreeNode(4,
			new TreeNode(11,
				new TreeNode(7),
				new TreeNode(2)
			)
		),
		new TreeNode(8,
			new TreeNode(13),
			new TreeNode(4,
				null,
				new TreeNode(1)
			)
		)
	);
	expect(hasPathSum(root, 22)).toBe(true);
});

test("No sum", () => {
	const root = new TreeNode(1,
		new TreeNode(2),
		new TreeNode(3)
	);
	expect(hasPathSum(root, 5)).toBe(false);
});

test("Empty tree", () => {
	expect(hasPathSum(null, 0)).toBe(false);
});
