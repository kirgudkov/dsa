import type { TreeNode } from "./TreeNode.ts";

export function eulerTour(root: TreeNode | null, depth: number): Visit[] {
	const visits: Visit[] = [];
	walk(root, visits, depth);
	return visits;
}

function walk(root: TreeNode | null, visits: Visit[], depth: number) {
	if (!root) {
		return;
	}

	visits.push({ node: root, depth });

	if (root.left) {
		walk(root.left, visits, depth + 1);
		visits.push({ node: root, depth });
	}

	if (root.right) {
		walk(root.right, visits, depth + 1);
		visits.push({ node: root, depth });
	}
}

export type Visit = {
	node: TreeNode;
	depth: number;
};
