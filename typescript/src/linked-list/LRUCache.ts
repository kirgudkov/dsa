// https://leetcode.com/problems/lru-cache

class LRUListNode {
	key: number;
	value: number;

	next?: LRUListNode;
	prev?: LRUListNode;

	constructor(key: number, value: number) {
		this.key = key;
		this.value = value;
	}
}

class LRUCache {

	private readonly capacity: number;
	private readonly map: Map<number, LRUListNode>;

	private head?: LRUListNode;
	private tail?: LRUListNode;

	constructor(capacity: number) {
		this.map = new Map();
		this.capacity = capacity;
	}

	get(key: number): number {
		return this.map.has(key)
			? this.update_node(this.map.get(key)!).value
			: -1;
	}

	put(key: number, value: number): void {
		if (this.map.has(key)) {
			this.update_node(this.map.get(key)!, value);
		} else {
			const new_node = new LRUListNode(key, value);
			this.map.set(key, new_node);

			if (this.tail) {
				this.tail.next = new_node;
				new_node.prev = this.tail;
			}

			if (!this.head) {
				this.head = this.tail;
			}

			this.tail = new_node;
			this.truncate();
		}
	}

	private truncate() {
		if (this.map.size > this.capacity) {
			const next_head = this.head!.next;
			this.map.delete(this.head!.key);
			this.head = next_head;
		}
	}

	private update_node(node: LRUListNode, value?: number) {
		if (value !== undefined) {
			node.value = value;
		}

		if (node === this.head) {
			this.head = node.next;
		}

		if (node === this.tail) {
			return node;
		}

		const prev = node.prev;
		const next = node.next;

		if (prev) {
			prev.next = next;
		}

		if (next) {
			next.prev = prev;
		}

		this.tail!.next = node;
		node.prev = this.tail;
		node.next = undefined;
		this.tail = node;

		return node;
	}
}

export { LRUCache };
