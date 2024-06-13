import type { TreeNode } from "./TreeNode.ts";
import { eulerTour, type Visit } from "./eulerTour.ts";

// Used magic described here: https://en.wikipedia.org/wiki/Lowest_common_ancestor
// eulerTour modification was applied to walk the tree and collect the depth of each node.
export function lowestCommonAncestor(root: TreeNode | null, p: TreeNode | null, q: TreeNode | null): TreeNode | null {
	const visits: Visit[] = eulerTour(root, 0);

	const index_p = visits.findIndex((v) => v.node === p);
	const index_q = visits.findIndex((v) => v.node === q);

	const range = visits.slice(
		Math.min(index_p, index_q),
		Math.max(index_p, index_q) + 1
	);

	let lca = range[0];

	for (let i = 1; i < range.length; i++) {
		if (range[i].depth < lca.depth) {
			lca = range[i];
		}
	}

	return lca?.node ?? null;
}
