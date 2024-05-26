import { describe, test, expect } from "bun:test";
import { MyLinkedList, MyListNode } from "./MyLinkedList";

describe("MyLinkedList", () => {
	test("get", () => {
		const linkedList = new MyLinkedList();
		expect(linkedList.get(0)).toBe(-1);
		linkedList.addAtHead(1);
		expect(linkedList.get(0)).toBe(1);
		linkedList.addAtTail(3);
		expect(linkedList.get(1)).toBe(3);
		linkedList.addAtIndex(1, 2);
		expect(linkedList.get(1)).toBe(2);
	});

	test("addAtHead", () => {
		const linkedList = new MyLinkedList();
		linkedList.addAtHead(1);
		expect(linkedList.get(0)).toBe(1);
		linkedList.addAtHead(2);
		expect(linkedList.get(0)).toBe(2);
	});

	test("addAtTail", () => {
		const linkedList = new MyLinkedList();
		linkedList.addAtTail(1);
		expect(linkedList.get(0)).toBe(1);
		linkedList.addAtTail(2);
		expect(linkedList.get(1)).toBe(2);
	});

	test("addAtIndex", () => {
		const linkedList = new MyLinkedList();
		linkedList.addAtIndex(0, 1);
		expect(linkedList.get(0)).toBe(1);
		linkedList.addAtIndex(0, 2);
		expect(linkedList.get(0)).toBe(2);
		linkedList.addAtIndex(1, 3);
		expect(linkedList.get(1)).toBe(3);
	});

	test("deleteAtIndex", () => {
		const linkedList = new MyLinkedList();
		linkedList.addAtHead(1);
		linkedList.addAtTail(3);
		linkedList.addAtIndex(1, 2);
		linkedList.deleteAtIndex(1);
		expect(linkedList.get(1)).toBe(3);
		linkedList.deleteAtIndex(0);
		expect(linkedList.get(0)).toBe(3);
		linkedList.deleteAtIndex(0);
		expect(linkedList.get(0)).toBe(-1);
	});
});
