export class DoublyLinkedList {
	private length: number = 0;

	private head?: DoublyLinkedListNode;
	private tail?: DoublyLinkedListNode;

	get(index: number): number {
		if (index < 0 || index >= this.length) {
			return -1;
		}

		if (index === 0) {
			return this.head?.val ?? -1;
		}

		if (index === this.length - 1) {
			return this.tail?.val ?? -1;
		}

		return this.getNode(index)?.val ?? -1;
	}

	addAtHead(val: number): void {
		const head = new DoublyLinkedListNode(val, this.head);

		if (this.head) {
			this.head.prev = head;
		}

		this.head = head;

		if (!this.tail) {
			this.tail = head;
		}

		this.length++;
	}

	addAtTail(val: number): void {
		if (!this.head) {
			this.addAtHead(val);
			return;
		}

		const tail = new DoublyLinkedListNode(val, undefined, this.tail);

		if (this.tail) {
			this.tail.next = tail;
		}

		this.tail = tail;
		this.length++;
	}

	addAtIndex(index: number, val: number): void {

		if (index === 0) {
			this.addAtHead(val);
			return;
		}

		if (index === this.length) {
			this.addAtTail(val);
			return;
		}

		if (index < 0 || index > this.length || !this.head) {
			return;
		}

		const curr = this.getNode(index)!;
		const node = new DoublyLinkedListNode(val);

		node.prev = curr!.prev;
		node.next = curr;
		curr!.prev!.next = node;
		curr!.prev = node;

		this.length++;
	}

	deleteAtIndex(index: number): void {
		if (index < 0 || index >= this.length || !this.head) {
			return;
		}

		if (index === 0) {
			this.deleteHead();
			return;
		}

		if (index === this.length - 1) {
			this.deleteTail();
			return;
		}

		const curr = this.getNode(index)!;

		curr.prev!.next = curr.next;
		curr.next!.prev = curr.prev;

		this.length--;
	}

	private getNode(index: number): DoublyLinkedListNode | undefined {
		if (index < 0 || index >= this.length) {
			return;
		}

		if (index === 0) {
			return this.head;
		}

		if (index === this.length - 1) {
			return this.tail;
		}

		let curr: DoublyLinkedListNode | undefined;

		if (index < this.length / 2) {
			curr = this.head;
			for (let i = 0; i < index; i++) {
				curr = curr?.next;
			}
		} else {
			curr = this.tail;
			for (let i = this.length - 1; i > index; i--) {
				curr = curr?.prev;
			}
		}

		return curr;
	}

	private deleteHead(): void {
		if (!this.head) {
			return;
		}

		this.head = this.head.next;

		if (this.head) {
			this.head.prev = undefined;
		}

		this.length--;
	}

	private deleteTail(): void {
		if (!this.tail) {
			return;
		}

		this.tail = this.tail.prev;

		if (this.tail) {
			this.tail.next = undefined;
		}

		this.length--;
	}
}

class DoublyLinkedListNode {
	val: number;
	next?: DoublyLinkedListNode;
	prev?: DoublyLinkedListNode;

	constructor(val: number, next?: DoublyLinkedListNode, prev?: DoublyLinkedListNode) {
		this.val = val;
		this.next = next;
		this.prev = prev;
	}
}
