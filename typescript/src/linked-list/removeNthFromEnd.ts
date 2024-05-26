import type { ListNode } from "./ListNode.ts";

export function removeNthFromEnd(head: ListNode | null, n: number): ListNode | null {
	let slow = head;
	let slow_prev = null;

	// fast pointer will be n nodes ahead of slow pointer
	let fast = head;

	let delay = n;
	// Once we reach the end of the list, slow pointer will be at the n-th node from the end
	while (fast?.next) {
		fast = fast.next;
		delay--;

		// Delay's been exhausted, start moving slow pointer
		// keep track of the previous node
		if (delay < 1) {
			slow_prev = slow;
			slow = slow?.next ?? null;
		}
	}

	if (slow_prev) {
		slow_prev.next = slow?.next ?? null;

		// No slow_prev means we never got farther than the head position => there is only one item in the list.
		// In this case, return empty list (null head).
	} else if (n === 1) {
		return null;
	} else {
		// shift head to the next node
		head = head?.next ?? null;
	}

	return head;
}
