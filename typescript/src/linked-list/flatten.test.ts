import { describe, test, expect } from "bun:test";
import { flatten, _Node } from "./flatten.ts";

describe("flatten", () => {
	test("should flatten the linked list", () => {
		// 1 -> 2 -> 3 -> 4 -> 5 -> 6
		//           |
		//           7 -> 8 -> 9 -> 10
		//                |
		//               11 -> 12

		const head = new _Node(1);
		const n2 = new _Node(2);
		const n3 = new _Node(3);
		const n4 = new _Node(4);
		const n5 = new _Node(5);
		const n6 = new _Node(6);
		const n7 = new _Node(7);
		const n8 = new _Node(8);
		const n9 = new _Node(9);
		const n10 = new _Node(10);
		const n11 = new _Node(11);
		const n12 = new _Node(12);

		head.next = n2;
		n2.prev = head;

		n2.next = n3;
		n3.prev = n2;

		n3.next = n4;
		n4.prev = n3;

		n4.next = n5;
		n5.prev = n4;

		n5.next = n6;
		n6.prev = n5;

		n3.child = n7;

		n7.next = n8;
		n8.prev = n7;

		n8.next = n9;
		n9.prev = n8;

		n9.next = n10;
		n10.prev = n9;

		n8.child = n11;

		n11.next = n12;
		n12.prev = n11;

		const result = flatten(head);

		{
			// 1 -> 2 -> 3 -> 7 -> 8 -> 11 -> 12 -> 9 -> 10 -> 4 -> 5 -> 6

			head.next = n2;
			n2.prev = head;

			n2.next = n3;
			n3.prev = n2;

			n3.next = n7;
			n7.prev = n3;

			n7.next = n8;
			n8.prev = n7;

			n8.next = n11;
			n11.prev = n8;

			n11.next = n12;
			n12.prev = n11;

			n12.next = n9;
			n9.prev = n12;

			n9.next = n10;
			n10.prev = n9;

			n10.next = n4;
			n4.prev = n10;

			n4.next = n5;
			n5.prev = n4;

			n5.next = n6;
			n6.prev = n5;

			expect(result).toEqual(head);
		}
	});
});
