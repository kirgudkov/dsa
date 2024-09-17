import type { TreeNode } from "./TreeNode.ts";
import { eulerTour } from "./eulerTour.ts";

// Used magic described here: https://en.wikipedia.org/wiki/Lowest_common_ancestor
// eulerTour modification was applied to walk the tree and collect the depth of each node.
export function lowestCommonAncestor(root: TreeNode | null, p: TreeNode | null, q: TreeNode | null): TreeNode | null {
	const tour = eulerTour(root);

	const pi = tour.findIndex(([node]) => node == p);
	const qi = tour.findIndex(([node]) => node == q);

	const range = tour.slice(
		Math.min(pi, qi),
		Math.max(pi, qi) + 1
	);

	return range.reduce((min, curr) =>
		curr[1] < min[1] ? curr : min
	)[0];
}
