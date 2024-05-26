import { describe, test, expect } from "bun:test";
import { ListNode } from "./ListNode.ts";
import { removeElements } from "./removeElements.ts";

describe("removeElements", () => {
	test("Example 1", () => {
		const head = new ListNode(1,
			new ListNode(2,
				new ListNode(6,
					new ListNode(3,
						new ListNode(4,
							new ListNode(5,
								new ListNode(6)
							)
						)
					)
				)
			)
		);
		const val = 6;
		const expected = new ListNode(1,
			new ListNode(2,
				new ListNode(3,
					new ListNode(4,
						new ListNode(5)
					)
				)
			)
		);
		expect(removeElements(head, val)).toEqual(expected);
	});

	test("Example 2", () => {
		const head = new ListNode(7,
			new ListNode(7,
				new ListNode(7,
					new ListNode(7)
				)
			)
		);
		const val = 7;
		const expected = null;
		expect(removeElements(head, val)).toEqual(expected);
	});

	test("Example 3", () => {
		const head = new ListNode(1,
			new ListNode(2,
				new ListNode(1,
					new ListNode(4,
						new ListNode(5)
					)
				)
			)
		);
		const val = 1;
		const expected = new ListNode(2,
			new ListNode(4,
				new ListNode(5)
			)
		);
		expect(removeElements(head, val)).toEqual(expected);
	});
});
