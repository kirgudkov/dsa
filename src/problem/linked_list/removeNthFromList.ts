// Definition for singly-linked list.
class ListNode {
    val: number
    next: ListNode | null

    constructor(val?: number, next?: ListNode | null) {
        this.val = (val === undefined ? 0 : val)
        this.next = (next === undefined ? null : next)
    }
}

function removeNthFromEnd(head: ListNode | null, n: number): ListNode | null {
    const arr = [head];

    let current = head;
    while (current.next) {
        arr.push(current.next);
        current = current.next;
    }

    // More likely that we got n == list.size; we need to remove head
    if ((arr.length - n - 1) === -1) {
        return arr[1] ?? null; // New head or null if list had one item
    }

    arr[arr.length - n - 1].next = arr[arr.length - n + 1] ?? null;

    return head;
}
