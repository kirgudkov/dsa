import { describe, test, expect } from "bun:test";
import { removeNthFromEnd } from "./removeNthFromEnd.ts";

describe("removeNthFromEnd", () => {
	test("should remove the nth node from the end of the list", () => {
		const head = { val: 1, next: { val: 2, next: { val: 3, next: null } } };
		const result = removeNthFromEnd(head, 2);
		expect(result).toEqual({ val: 1, next: { val: 3, next: null } });
	});

	test("should return null if the list has only one node", () => {
		const head = { val: 1, next: null };
		const result = removeNthFromEnd(head, 1);
		expect(result).toBeNull();
	});
});
