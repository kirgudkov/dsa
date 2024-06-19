import { test, expect } from "bun:test";
import { insert } from "./insertInSortedCircularList";
import { ListNode } from "./ListNode";

test("insert in sorted circular list #1", () => {
	const head = new ListNode(1);
	head.next = head;
	insert(head, 0);

	expect(head.next.val).toBe(0);
	expect(head.next.next?.val).toBe(1);
});

test("insert in sorted circular list #2", () => {
	const head = new ListNode(1);
	head.next = head;
	insert(head, 2);

	expect(head.next.val).toBe(2);
	expect(head.next.next?.val).toBe(1);
});

test("insert in sorted circular list #3", () => {
	const head = new ListNode(1);
	head.next = new ListNode(3);
	head.next.next = new ListNode(5);
	head.next.next.next = head;
	insert(head, 0);

	expect(head.next.val).toBe(3);
	expect(head.next.next?.val).toBe(5);
	expect(head.next.next?.next?.val).toBe(0);
	expect(head.next.next?.next?.next === head).toBe(true);
});

test("insert in sorted circular list #4", () => {
	const head = new ListNode(1);
	head.next = new ListNode(3);
	head.next.next = new ListNode(5);
	head.next.next.next = head;
	insert(head, 4);

	expect(head.next.val).toBe(3);
	expect(head.next.next?.val).toBe(4);
	expect(head.next.next?.next?.val).toBe(5);
	expect(head.next.next?.next?.next === head).toBe(true);
});

test("insert in sorted circular list #5", () => {
	const head = new ListNode(1);
	head.next = new ListNode(3);
	head.next.next = new ListNode(5);
	head.next.next.next = head;
	insert(head, 6);

	expect(head.next.val).toBe(3);
	expect(head.next.next?.val).toBe(5);
	expect(head.next.next?.next?.val).toBe(6);
	expect(head.next.next?.next?.next === head).toBe(true);
});

test("insert in sorted circular list #6", () => {
	const head = insert(null, 1);
	expect(head?.val).toBe(1);
	expect(head?.next === head).toBe(true);
});

test("insert in sorted circular list #7", () => {
	const head = new ListNode(3);
	head.next = new ListNode(2);
	head.next.next = new ListNode(2, head);
	insert(head, 0);

	expect(head.val).toBe(3);
	expect(head.next?.val).toBe(0);
	expect(head.next?.next?.val).toBe(2);
	expect(head.next?.next?.next?.val).toBe(2);
});

test("insert in sorted circular list #8", () => {
	const head = new ListNode(3);
	head.next = new ListNode(3);
	head.next.next = new ListNode(3, head);
	insert(head, 0);

	expect(head.val).toBe(3);
	expect(head.next?.val).toBe(3);
	expect(head.next?.next?.val).toBe(3);
	expect(head.next?.next?.next?.val).toBe(0);
});
