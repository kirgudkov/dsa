// https://leetcode.com/problems/lru-cache
class LRUCache {
	private readonly capacity: number;
	private readonly cache = new Map<number, LRUListNode>();

	private least?: LRUListNode;
	private most?: LRUListNode;

	constructor(capacity: number) {
		this.capacity = capacity;
	}

	get = (key: number) => this.cache.has(key)
		? this.updateNode(this.cache.get(key)!).value
		: -1;

	put = (key: number, value: number) => {
		this.cache.has(key)
			? this.updateNode(this.cache.get(key)!, value)
			: this.createNode(key, value);

		if (this.cache.size > this.capacity) {
			this.deleteNode(this.least!);
		}
	};

	private updateNode(node: LRUListNode, value?: number) {
		this.deleteNode(node);
		return this.createNode(node.key, value ?? node.value);
	}

	private deleteNode(node: LRUListNode): void {
		this.cache.delete(node.key);

		const next = node.next;
		const prev = node.prev;

		node.prev && (node.prev.next = next);
		node.next && (node.next.prev = prev);
		node.prev = undefined;
		node.next = undefined;

		if (node == this.least) {
			this.least = next;
		}

		if (node == this.most) {
			this.most = prev;
		}
	}

	private createNode(key: number, value: number) {
		const node = new LRUListNode(key, value);

		if (this.most) {
			this.most.next = node;
			node.prev = this.most;
		}

		if (!this.least) {
			this.least = this.most;
		}

		this.most = node;
		this.cache.set(key, node);

		return node;
	}
}

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

export { LRUCache };
