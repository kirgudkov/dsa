import type { TreeNode } from "./TreeNode.ts";
import { eulerTour } from "./eulerTour.ts";

// Used magic described here: https://en.wikipedia.org/wiki/Lowest_common_ancestor
// eulerTour modification was applied to walk the tree and collect the depth of each node.
export function lowestCommonAncestor(root: TreeNode | null, p: TreeNode | null, q: TreeNode | null): TreeNode | null {
	const visits = eulerTour(root);

	const index_p = visits.findIndex(([node]) => node === p);
	const index_q = visits.findIndex(([node]) => node === q);

	const range = visits.slice(
		Math.min(index_p, index_q),
		Math.max(index_p, index_q) + 1
	);

	let lca = range[0];

	for (let i = 1; i < range.length; i++) {
		if (range[i][1] < lca[1]) {
			lca = range[i];
		}
	}

	return lca?.[0] ?? null;
}
