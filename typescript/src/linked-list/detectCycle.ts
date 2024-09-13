import type { ListNode } from "./ListNode.ts";

export function detectCycle(head: ListNode | null): ListNode | null {
	const hash_map = new Set<ListNode>();

	while (head) {
		if (hash_map.has(head)) {
			return head;
		}

		hash_map.add(head);
		head = head.next;
	}

	return null;
}

export function detectCycleSlowFastPointers(head: ListNode | null): ListNode | null {
	let slow = head;
	let fast = head;

	while (slow && fast && fast.next) {
		slow = slow.next;
		fast = fast.next.next;

		if (slow == fast) {
			fast = head;

			while (slow != fast) {
				slow = slow!.next;
				fast = fast!.next;
			}

			return slow;
		}
	}

	return null;
}
