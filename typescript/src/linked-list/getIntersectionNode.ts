import type { ListNode } from "./ListNode.ts";

// https://leetcode.com/problems/intersection-of-two-linked-lists/
export function getIntersectionNode(headA: ListNode | null, headB: ListNode | null): ListNode | null {
	let p1 = headA;
	let p2 = headB;

	while (p1 !== p2) {
		p1 = p1 ? p1.next : headB;
		p2 = p2 ? p2.next : headA;
	}

	return p1;
}
