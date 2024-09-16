import type { TreeNode } from "./TreeNode.ts";

export function eulerTour(root: TreeNode | null): [TreeNode, number][] {
	const tour: [TreeNode, number][] = [];

	if (!root) {
		return tour;
	}

	const stack: [TreeNode, number, boolean][] = [[root, 0, false]];

	while (stack.length) {
		const [node, depth, visited] = stack.pop()!;

		tour.push([node, depth]);

		if (visited) {
			continue;
		}

		if (node.right) {
			stack.push([node, depth, true]);
			stack.push([node.right, depth + 1, false]);
		}

		if (node.left) {
			stack.push([node, depth, true]);
			stack.push([node.left, depth + 1, false]);
		}
	}

	return tour;
}
