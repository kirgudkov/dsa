// https://leetcode.com/problems/lru-cache
class LRUCache<KeyType, ValueType> {
	private readonly map = new Map<KeyType, LRUListNode<KeyType, ValueType>>();

	private lru?: LRUListNode<KeyType, ValueType>;
	private mru?: LRUListNode<KeyType, ValueType>;

	constructor(
		private readonly capacity: number,
	) {}

	get = (key: KeyType) => this.map.has(key)
		? this.handleCacheHit(this.map.get(key)!).value
		: -1; // todo: -1 is not a good choice, since we're using generics, we should return undefined or null

	put = (key: KeyType, value: ValueType) => {
		this.map.has(key)
			// If the key exists, update the value and promote the node
			? this.handleCacheHit(this.map.get(key)!, value)
			// Otherwise create a new node
			: this.handleCacheMiss(key, value);
	};

	// This method is used to promote the node to the "mru", making it the mru recently used.
	// Instead of modifying the node, we delete it and create a new one with the same key and value.
	// While this is not the mru efficient way to do it, it simplifies the code.
	private handleCacheHit(node: LRUListNode<KeyType, ValueType>, value?: ValueType) {
		this.deleteNode(node);
		return this.createNode(node.key, value ?? node.value);
	}

	private handleCacheMiss(key: KeyType, value: ValueType) {
		this.createNode(key, value);

		if (this.map.size > this.capacity) {
			this.deleteNode(this.lru!);
		}
	}

	private deleteNode(node: LRUListNode<KeyType, ValueType>): void {
		// Step 1: Delete the node from the map
		this.map.delete(node.key);

		const next = node.next;
		const prev = node.prev;

		// Step 2: Delete the node from the linked list
		if (prev) prev.next = next;
		if (next) next.prev = prev;
		node.prev = undefined;
		node.next = undefined;

		// Step 3: Update the lru and mru if necessary
		if (node == this.lru) this.lru = next;
		if (node == this.mru) this.mru = prev;
	}

	private createNode(key: KeyType, value: ValueType): LRUListNode<KeyType, ValueType> {
		// Step 1: Create a new node
		const node = new LRUListNode<KeyType, ValueType>(key, value);

		// Step 2: Add the node to the linked list
		if (this.mru) {
			node.prev = this.mru;
			this.mru.next = node;
		}
		this.mru = node;
		if (!this.lru) this.lru = node;

		// Step 3: Add the node to the map
		this.map.set(key, node);

		return node;
	}
}

class LRUListNode<KeyType, ValueType> {
	constructor(
		public key: KeyType,
		public value: ValueType,
	) {}

	next?: LRUListNode<KeyType, ValueType>;
	prev?: LRUListNode<KeyType, ValueType>;
}

export { LRUCache };
