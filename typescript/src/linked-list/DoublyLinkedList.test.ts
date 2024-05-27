import { describe, test, expect } from "bun:test";
import { DoublyLinkedList } from "./DoublyLinkedList.ts";

describe("DoublyLinkedList", () => {
	test("get and addAtTail", () => {
		const list = new DoublyLinkedList();
		list.addAtTail(1);
		list.addAtTail(2);
		list.addAtTail(3);

		expect(list.get(0)).toBe(1);
		expect(list.get(1)).toBe(2);
		expect(list.get(2)).toBe(3);
	});

	test("addAtHead", () => {
		const list = new DoublyLinkedList();
		list.addAtHead(1);
		list.addAtHead(2);
		list.addAtHead(3);

		expect(list.get(0)).toBe(3);
		expect(list.get(1)).toBe(2);
		expect(list.get(2)).toBe(1);
	});

	test("addAtIndex", () => {
		const list = new DoublyLinkedList();
		list.addAtTail(1);
		list.addAtTail(2);
		list.addAtTail(3);
		list.addAtIndex(1, 4);
		list.addAtIndex(0, 5);
		list.addAtIndex(4, 6);

		expect(list.get(0)).toBe(5);
		expect(list.get(1)).toBe(1);
		expect(list.get(2)).toBe(4);
		expect(list.get(3)).toBe(2);
		expect(list.get(4)).toBe(6);
		expect(list.get(5)).toBe(3);
	});

	test("deleteAtIndex", () => {
		const list = new DoublyLinkedList();
		list.addAtTail(1);
		list.addAtTail(2);
		list.addAtTail(3);

		list.deleteAtIndex(2);

		expect(list.get(0)).toBe(1);
		expect(list.get(1)).toBe(2);

		list.deleteAtIndex(0);
		expect(list.get(0)).toBe(2);
	});

	test("random 1", () => {
		const list = new DoublyLinkedList();
		list.addAtHead(7);
		list.addAtHead(2);
		list.addAtHead(1);
		list.addAtIndex(3, 0);
		list.deleteAtIndex(2);
		list.addAtHead(6);
		list.addAtTail(4);
		expect(list.get(4)).toBe(4);
		list.addAtHead(4);
		list.addAtIndex(5, 0);
		list.addAtHead(6);
	});

	test("random 2", () => {
		const list = new DoublyLinkedList();
		list.addAtHead(5);
		list.addAtIndex(1, 2);
		expect(list.get(1)).toBe(2);
		list.addAtHead(6);
		list.addAtTail(2);
		expect(list.get(3)).toBe(2);
		list.addAtTail(1);
		expect(list.get(5)).toBe(-1);
		list.addAtHead(2);
		expect(list.get(2)).toBe(5);
	});
});
