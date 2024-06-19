class _Node {
	val: number;
	next: _Node | null;
	random: _Node | null;

	constructor(val?: number, next?: _Node, random?: _Node) {
		this.val = (val === undefined ? 0 : val);
		this.next = (next === undefined ? null : next);
		this.random = (random === undefined ? null : random);
	}
}

function copyRandomList(head: _Node | null): _Node | null {
	if (!head) {
		return null;
	}

	const map = new Map<_Node, _Node>();

	const get_copy = (node?: _Node | null) => {
		if (!node) {
			return null;
		}

		let copy = map.get(node);

		if (!copy) {
			copy = new _Node(node.val);
			map.set(node, copy);
		}

		return copy;
	};

	let old_node: _Node | null = head;
	let new_node: _Node | null = get_copy(old_node);

	while (new_node) {
		new_node.next = get_copy(old_node?.next);
		new_node.random = get_copy(old_node?.random);

		new_node = new_node.next;
		old_node = old_node?.next ?? null;
	}

	return get_copy(head);
}

export { copyRandomList, _Node };
