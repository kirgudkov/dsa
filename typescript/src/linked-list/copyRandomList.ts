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
	const map = new Map<_Node, _Node>();

	const getCopy = (original?: _Node | null): _Node | null => {
		if (!original) return null;

		if (!map.has(original)) {
			map.set(original, new _Node(original.val));
		}

		return map.get(original)!;
	};

	let oldNode = head;
	let newNode = getCopy(head);

	while (newNode && oldNode) {
		newNode.next = getCopy(oldNode.next);
		newNode.random = getCopy(oldNode.random);

		newNode = newNode.next;
		oldNode = oldNode.next;
	}

	return getCopy(head);
}

export { copyRandomList, _Node };
