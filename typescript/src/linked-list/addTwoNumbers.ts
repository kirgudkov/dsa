import { ListNode } from "./ListNode.ts";

export function addTwoNumbers(l1: ListNode | null, l2: ListNode | null): ListNode | null {
	const head = new ListNode();

	let current = head;
	let carry = 0;

	while (l1 || l2) {

		let sum = (l1?.val ?? 0) + (l2?.val ?? 0) + carry;

		l1 = l1?.next ?? null;
		l2 = l2?.next ?? null;

		carry = Math.floor(sum / 10);
		current.val = sum > 9 ? sum % 10 : sum;

		if (l1 || l2) {
			current.next = new ListNode();
			current = current.next;
		}
	}

	if (carry) {
		current.next = new ListNode(carry);
	}

	return head;
}
