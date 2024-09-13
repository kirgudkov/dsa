import type { ListNode } from "./ListNode.ts";

// https://leetcode.com/problems/remove-nth-node-from-end-of-list
export function removeNthFromEnd(head: ListNode, n: number): ListNode | null {
	let [nth_prev, tail] = [head, head];

	while (tail.next) {
		if (n <= 0) {
			nth_prev = nth_prev.next!;
		}

		tail = tail.next;
		n--;
	}

	if (n > 0) { // nth_prev wasn't moved -> head is the one to be deleted
		return head.next;
	}

	nth_prev.next = nth_prev.next?.next ?? null;
	return head;
}
