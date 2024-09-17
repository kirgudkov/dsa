import { TreeNode } from "./TreeNode.ts";

export function generateTrees(n: number): (TreeNode | null)[] {
	return generate(1, n);
}

function generate(l: number, r: number) {
	const result: (TreeNode | null)[] = [];

	if (l > r) {
		return [null];
	}

	for (let i = l; i <= r; i++) {
		const left = generate(l, i - 1);
		const right = generate(i + 1, r);

		for (const left_node of left) {
			for (const right_node of right) {
				result.push(new TreeNode(i, left_node, right_node));
			}
		}
	}

	return result;
}
