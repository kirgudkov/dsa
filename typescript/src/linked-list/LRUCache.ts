// https://leetcode.com/problems/lru-cache
class LruCache<K extends string | number, V> implements Cache<K, V> {
	readonly #map: Map<K, LinkedNode<CacheEntry<K, V>>> = new Map();
	readonly #list: List<LinkedNode<CacheEntry<K, V>>> = new DoublyLinkedList();
	readonly #capacity: number;

	constructor(capacity: number) {
		this.#capacity = capacity;
	}

	put(key: K, value: V): void {
		const existingNode = this.#map.get(key);

		if (existingNode) {
			existingNode.value.value = value;
			this.#promoteNode(existingNode);
			return;
		}

		const newNode = {
			value: { key, value },
		};

		this.#list.pushFront(newNode);
		this.#map.set(key, newNode);

		this.#evict();
	}

	get(key: K): V | undefined {
		const node = this.#map.get(key);

		if (!node) {
			return undefined;
		}

		this.#promoteNode(node);
		return node.value.value;
	}

	#promoteNode(node: LinkedNode<CacheEntry<K, V>>): void {
		this.#list.remove(node);
		this.#list.pushFront(node);
	}

	#evict(): void {
		if (this.#map.size > this.#capacity && this.#list.last) {
			this.#map.delete(this.#list.last.value.key);
			this.#list.remove(this.#list.last);
		}
	}
}

class DoublyLinkedList<T> implements List<LinkedNode<T>> {
	last?: LinkedNode<T>;
	private first?: LinkedNode<T>;

	remove(node: LinkedNode<T>): void {
		const prev = node.prev;
		const next = node.next;

		prev && (prev.next = next);
		next && (next.prev = prev);
		node.prev = undefined;
		node.next = undefined;

		// It's important to update refs
		if (node === this.first) this.first = next;
		if (node === this.last) this.last = prev;
	}

	pushFront(node: LinkedNode<T>): void {
		// No matter what first is, set it as next
		node.next = this.first;
		// if list isn't empty, demote current first
		if (this.first) {
			this.first.prev = node;
		} else {
			// Otherwise, new node is the only one so far.
			// So it is first and last at the same time
			this.last = node;
		}
		// Finally, update first ref
		this.first = node;
	}
}

interface LinkedNode<T> {
	value: T;
	prev?: LinkedNode<T>;
	next?: LinkedNode<T>;
}

interface CacheEntry<K, V> {
	key: K;
	value: V;
}

interface List<T> {
	readonly last?: T;
	remove(node: T): void;
	pushFront(node: T): void;
}

interface Cache<K, V> {
	get(key: K): V | undefined;
	put(key: K, value: V): void;
}

export {
	LruCache,
};
