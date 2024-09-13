export class SinglyLinkedList {
	private head?: SinglyLinkedListNode;

	get(index: number) {
		let curr = this.head;

		for (let i = 0; i < index; i++) {
			curr = curr?.next;
		}

		return curr?.val ?? -1;
	}

	addAtHead(val: number) {
		this.head = new SinglyLinkedListNode(val, this.head);
	}

	addAtTail(val: number) {
		let tail = this.head;

		while (tail?.next) {
			tail = tail.next;
		}

		if (tail) {
			tail.next = new SinglyLinkedListNode(val);
		} else {
			this.addAtHead(val);
		}
	}

	addAtIndex(index: number, val: number) {
		if (!this.head || index == 0) {
			return this.addAtHead(val);
		}

		let curr = this.head;
		for (let i = index; i > 1; i--) {
			curr = curr.next!;
		}

		curr.next = new SinglyLinkedListNode(val, curr.next);
	}

	deleteAtIndex(index: number) {
		if (!this.head) {
			return;
		}

		let curr = this.head;
		for (let i = index; i > 1; i--) {
			curr = curr.next!;
		}

		if (curr == this.head) {
			this.head = this.head.next;
		} else {
			curr.next = curr.next?.next;
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
