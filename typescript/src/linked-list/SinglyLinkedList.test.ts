import { describe, test, expect } from "bun:test";
import { SinglyLinkedList } from "./SinglyLinkedList.ts";

describe("MyLinkedList", () => {
	test("get", () => {
		const linkedList = new SinglyLinkedList();
		expect(linkedList.get(0)).toBe(-1);
		linkedList.addAtHead(1);
		expect(linkedList.get(0)).toBe(1);
		linkedList.addAtTail(3);
		expect(linkedList.get(1)).toBe(3);
		linkedList.addAtIndex(1, 2);
		expect(linkedList.get(1)).toBe(2);
	});

	test("addAtHead", () => {
		const linkedList = new SinglyLinkedList();
		linkedList.addAtHead(1);
		expect(linkedList.get(0)).toBe(1);
		linkedList.addAtHead(2);
		expect(linkedList.get(0)).toBe(2);
	});

	test("addAtTail", () => {
		const linkedList = new SinglyLinkedList();
		linkedList.addAtTail(1);
		expect(linkedList.get(0)).toBe(1);
		linkedList.addAtTail(2);
		expect(linkedList.get(1)).toBe(2);
	});

	test("addAtIndex", () => {
		const linkedList = new SinglyLinkedList();
		linkedList.addAtIndex(0, 1);
		expect(linkedList.get(0)).toBe(1);
		linkedList.addAtIndex(0, 2);
		expect(linkedList.get(0)).toBe(2);
		linkedList.addAtIndex(1, 3);
		expect(linkedList.get(0)).toBe(2);
		expect(linkedList.get(1)).toBe(3);
		expect(linkedList.get(2)).toBe(1);
	});

	test("deleteAtIndex", () => {
		const linkedList = new SinglyLinkedList();
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
