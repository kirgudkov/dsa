// Definition for singly-linked list.
class ListNode {
    val: number
    next: ListNode | null

    constructor(val?: number, next?: ListNode | null) {
        this.val = (val === undefined ? 0 : val)
        this.next = (next === undefined ? null : next)
    }
}

// Using slow and fast pointers to detect a cycle aka Floyd's Tortoise and Hare algorithm:
// Fast pointer moves 2x speed of slow pointer, if there is a cycle, they will eventually meet
// If there is no cycle, fast pointer will reach the end of the list and break the loop
//
// Time complexity: O(n)
// Space complexity: O(1)

function hasCycle(head: ListNode | null): boolean {
    let slow = head;
    let fast = head;

    while (fast && fast.next) {
        slow = slow.next;
        fast = fast.next.next;

        if (slow == fast) {
            return true;
        }
    }

    return false;
}
