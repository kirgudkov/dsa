import { DoublyLinkedList } from "../linked-list/DoublyLinkedList.ts";

export class LinkedListCircularQueue {
	private readonly k: number;
	private list = new DoublyLinkedList();

	constructor(k: number) {
		this.k = k;
	}

	enQueue(value: number): boolean {
		if (this.isFull()) return false;

		this.list.addAtTail(value);
		return true;
	}

	deQueue(): boolean {
		if (this.isEmpty()) return false;

		this.list.deleteAtIndex(0);
		return true;
	}

	Front(): number {
		return this.isEmpty() ? -1 : this.list.get(0);
	}

	Rear(): number {
		return this.isEmpty() ? -1 : this.list.get(this.list.size - 1);
	}

	isFull(): boolean {
		return this.list.size === this.k;
	}

	isEmpty(): boolean {
		return this.list.size === 0;
	}
}

export class ArrayCircularQueue {
	private readonly k: number;
	private readonly buffer: number[] = [];
	private size = 0;

	private first: number = 0;
	// each enqueue operation increments this index,
	// so the first entry will be at 0 the position
	private last: number = -1;

	constructor(k: number) {
		this.k = k;
	}

	enQueue(value: number): boolean {
		if (this.isFull()) return false;

		this.last = ++this.last % this.k;
		this.buffer[this.last] = value;
		this.size++;
		return true;
	}

	deQueue(): boolean {
		if (this.isEmpty()) return false;

		this.first = ++this.first % this.k;
		this.size--;
		return true;
	}

	Front(): number {
		return this.isEmpty() ? -1 : this.buffer[this.first];
	}

	Rear(): number {
		return this.isEmpty() ? -1 : this.buffer[this.last];
	}

	isEmpty(): boolean {
		return this.size === 0;
	}

	isFull(): boolean {
		return this.size === this.k;
	}
}
