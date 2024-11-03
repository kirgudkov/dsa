// The easies solution. TC: O(n), SC: O(n) where n is the height of the tree.
// But most likely the follow up question will be to solve it without using extra space.
export function lowestCommonAncestor(p: _Node | null, q: _Node | null): _Node | null {
	const seen = new Set<_Node>();

	while (p) {
		seen.add(p);
		p = p.parent;
	}

	while (q) {
		if (seen.has(q)) return q;
		q = q.parent;
	}

	return null;
}

export function lowestCommonAncestor_h2(p: _Node | null, q: _Node | null): _Node | null {
	let _p = p;
	let _q = q;

	while (_p) {
		while (_q) {
			if (_q === _p) {
				return _q;
			}

			_q = _q.parent;
		}

		_p = _p.parent;
		_q = q;
	}

	return null;
}

export function lowestCommonAncestor_h_const_s(p: _Node | null, q: _Node | null): _Node | null {
	let _p = p;
	let _q = q;

	while (_p !== _q) {
		_p = _p?.parent ?? q;
		_q = _q?.parent ?? p;
	}

	return _p;
}

export class _Node {
	val: number;
	left: _Node | null;
	right: _Node | null;
	parent: _Node | null;

	constructor(val: number) {
		this.val = val;
		this.left = null;
		this.right = null;
		this.parent = null;
	}
}
