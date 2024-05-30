export class MyCircularQueue {

	private readonly k: number;
	private readonly queue: number[];

	private head?: number;
	private tail?: number;

	private length: number;

	constructor(k: number) {
		this.k = k;
		this.length = 0;
		this.queue = new Array(k);
	}

	enQueue(value: number): boolean {
		if (this.isFull()) {
			return false;
		}

		if (this.isEmpty()) {
			this.tail = 0;
			this.head = 0;
			this.queue[0] = value;
		} else {
			this.tail = (this.tail! + 1) % this.k;
			this.queue[this.tail] = value;
		}

		this.length++;

		return true;
	}

	deQueue(): boolean {
		if (this.isEmpty()) {
			return false;
		}

		if (this.head === this.tail) {
			this.head = undefined;
			this.tail = undefined;
		} else {
			this.head = (this.head! + 1) % this.k;
		}

		this.length--;

		return true;
	}

	Front(): number {
		return this.head === undefined
			? -1
			: this.queue[this.head];
	}

	Rear(): number {
		return this.tail === undefined
			? -1
			: this.queue[this.tail];
	}

	isEmpty(): boolean {
		return this.length === 0;
	}

	isFull(): boolean {
		return this.length === this.k;
	}
}
