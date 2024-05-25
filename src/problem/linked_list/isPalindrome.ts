// Definition for singly-linked list.
class ListNode {
    val: number
    next: ListNode | null

    constructor(val?: number, next?: ListNode | null) {
        this.val = (val === undefined ? 0 : val)
        this.next = (next === undefined ? null : next)
    }
}

function isPalindrome(head: ListNode | null): boolean {
    const stack = [];

    let current = head;
    while (current) {
        stack.push(current.val);
        current = current.next;
    }

    let isPalindrome = true;
    current = head;
    while (current) {
        if (current.val !== stack.pop()) {
            isPalindrome = false;
            break;
        }
        current = current.next;
    }

    return isPalindrome;
}
