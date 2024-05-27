// Definition for _Node.
export class _Node {
	val: number;
	prev: _Node | null;
	next: _Node | null;
	child: _Node | null;

	constructor(val?: number, prev?: _Node, next?: _Node, child?: _Node) {
		this.val = (val === undefined ? 0 : val);
		this.prev = (prev === undefined ? null : prev);
		this.next = (next === undefined ? null : next);
		this.child = (child === undefined ? null : child);
	}
}

export function flatten(head: _Node | null): _Node | null {
	let current = head;

	while (current) {
		if (current.child) {
			const next = current.next;

			const flatten_head = flatten(current.child);
			current.next = flatten_head;

			if (flatten_head) {
				flatten_head.prev = current;
			}

			let flatten_tail = flatten_head;
			while (flatten_tail?.next) {
				flatten_tail = flatten_tail.next;
			}

			if (flatten_tail) {
				flatten_tail.next = next;
			}

			if (next) {
				next.prev = flatten_tail;
			}

			current.child = null;
			current = next;
		} else {
			current = current.next;
		}
	}

	return head;
}
