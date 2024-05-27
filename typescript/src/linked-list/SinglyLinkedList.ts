export class SinglyLinkedList {
	private head?: SinglyLinkedListNode;

	get(index: number): number {
		if (!this.head) {
			return -1;
		}

		let i = 0;
		let curr: SinglyLinkedListNode | undefined = this.head;

		while (i < index) {
			curr = curr?.next;
			i++;
		}

		return curr?.val ?? -1;
	}

	addAtHead(val: number): void {
		const node = new SinglyLinkedListNode(val);

		if (this.head) {
			node.next = this.head;
		}

		this.head = node;
	}

	addAtTail(val: number): void {
		if (!this.head) {
			return this.addAtHead(val);
		}

		let curr: SinglyLinkedListNode | undefined = this.head;
		while (curr?.next) {
			curr = curr.next;
		}

		if (curr) {
			curr.next = new SinglyLinkedListNode(val);
		}
	}

	addAtIndex(index: number, val: number): void {
		if (!this.head && index === 0) {
			return this.addAtHead(val);
		}

		let prev: SinglyLinkedListNode | undefined = undefined;
		let curr: SinglyLinkedListNode | undefined = this.head;
		let i = 0;

		while (i < index) {
			prev = curr;
			curr = curr?.next;
			i++;
		}

		const node = new SinglyLinkedListNode(val);

		if (prev) {
			prev.next = node;
		}

		if (curr) {
			node.next = curr;
		}

		if (curr && curr === this.head) {
			this.head = node;
		}
	}

	deleteAtIndex(index: number): void {
		if (!this.head) {
			return;
		}

		let prev: SinglyLinkedListNode | undefined = undefined;
		let curr: SinglyLinkedListNode | undefined = this.head;
		let i = 0;

		while (i < index) {
			prev = curr;
			curr = curr?.next;
			i++;
		}

		if (curr && curr === this.head) {
			this.head = curr.next;
		}

		if (prev) {
			prev.next = curr?.next;
		}
	}
}

export class SinglyLinkedListNode {
	val?: number;
	next?: SinglyLinkedListNode;

	constructor(val?: number, next?: SinglyLinkedListNode) {
		this.val = val;
		this.next = next;
	}
}
