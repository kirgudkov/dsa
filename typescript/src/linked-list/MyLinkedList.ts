export class MyLinkedList {
    private head?: MyListNode<number>;

    get(index: number): number {
        if (!this.head) {
            return -1;
        }

        let i = 0;
        let curr: MyListNode<number> | undefined = this.head;

        while (i < index) {
            curr = curr?.next;
            i++;
        }

        return curr?.val ?? -1;
    }

    addAtHead(val: number): void {
        const node = new MyListNode(val);

        if (this.head) {
            node.next = this.head;
        }

        this.head = node;
    }

    addAtTail(val: number): void {
        if (!this.head) {
            return this.addAtHead(val);
        }

        let curr: MyListNode<number> | undefined = this.head;
        while (curr?.next) {
            curr = curr.next;
        }

        if (curr) {
            curr.next = new MyListNode<number>(val);
        }
    }

    addAtIndex(index: number, val: number): void {
        if (!this.head && index === 0) {
            return this.addAtHead(val);
        }

        let prev: MyListNode<number> | undefined = undefined;
        let curr: MyListNode<number> | undefined = this.head;
        let i = 0;

        while (i < index) {
            prev = curr;
            curr = curr?.next;
            i++;
        }

        const node = new MyListNode(val);

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

        let prev: MyListNode<number> | undefined = undefined;
        let curr: MyListNode<number> | undefined = this.head;
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

export class MyListNode<T> {
    val?: T;
    next?: MyListNode<T>;

    constructor(val?: T, next?: MyListNode<T>) {
        this.val = val;
        this.next = next;
    }
}
