export class DoublyLinkedList {
	private length: number = 0;

	private head?: DoublyLinkedListNode;
	private tail?: DoublyLinkedListNode;

	get(index: number): number {
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
			return this.addAtHead(val);
		}

		const tail = new DoublyLinkedListNode(val, undefined, this.tail);

		if (this.tail) {
			this.tail.next = tail;
		}

		this.tail = tail;
		this.length++;
	}

	addAtIndex(index: number, val: number): void {
		if (index == 0) {
			return this.addAtHead(val);
		}

		if (index == this.length) {
			return this.addAtTail(val);
		}

		if (index < 0 || index > this.length) {
			return;
		}

		const new_node = new DoublyLinkedListNode(val);
		const old_node = this.getNode(index)!;

		new_node.prev = old_node!.prev;
		new_node.next = old_node;
		old_node!.prev!.next = new_node;
		old_node!.prev = new_node;

		this.length++;
	}

	deleteAtIndex(index: number): void {
		if (index < 0 || index >= this.length) {
			return;
		}

		if (index == 0) {
			return this.deleteHead();
		}

		if (index == this.length - 1) {
			return this.deleteTail();
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
