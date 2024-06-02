import { TreeNode } from "./TreeNode.ts";

export function generateTrees(n: number): Array<TreeNode | null> {
	const generate_in_range = (l: number, r: number): Array<TreeNode | null> => {
		if (l > r) {
			return [null];
		}

		const result = [];

		for (let i = l; i <= r; i++) {
			const left = generate_in_range(l, i - 1);
			const right = generate_in_range(i + 1, r);

			for (const left_node of left) {
				for (const right_node of right) {
					result.push(new TreeNode(i, left_node, right_node));
				}
			}
		}

		return result;
	};

	return generate_in_range(1, n);
}
