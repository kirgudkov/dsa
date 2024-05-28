import { test, expect } from "bun:test";
import { serialize, deserialize } from "./serialize";
import { TreeNode } from "./TreeNode.ts";

const root = new TreeNode(1,
	new TreeNode(2,
		null,
		new TreeNode(3)
	),
	new TreeNode(4,
		new TreeNode(5),
		new TreeNode(6)
	)
);

const serialized = "1,2,#,3,#,#,4,5,#,#,6,#,#";

test("Serialize", () => {
	expect(serialize(root)).toBe(serialized);
});

test("Deserialize", () => {
	expect(deserialize(serialized)).toEqual(root);

	expect(deserialize("#")).toBe(null);
	expect(deserialize("1,#,#")).toEqual(new TreeNode(1));
	expect(deserialize("1,2,#,#,#")).toEqual(new TreeNode(1, new TreeNode(2)));

	expect(deserialize("1,12,#,#,#")).toEqual(new TreeNode(1, new TreeNode(12)));

	expect(deserialize("1,-2,#,#,#")).toEqual(new TreeNode(1, new TreeNode(-2)));
});
