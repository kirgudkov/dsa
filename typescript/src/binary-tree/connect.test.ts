import { test, expect } from "bun:test";
import { connect } from "./connect.ts";
import { _Node } from "./connect.ts";

test("Few elements", () => {
	const root = new _Node(1);
	const n2 = new _Node(2);
	const n3 = new _Node(3);
	const n4 = new _Node(4);
	const n5 = new _Node(5);
	const n6 = new _Node(6);
	const n7 = new _Node(7);

	root.left = n2;
	root.right = n3;

	n2.left = n4;
	n2.right = n5;

	n3.left = n6;
	n3.right = n7;

	connect(root);

	expect(n2.next).toEqual(n3);
	expect(n4.next).toEqual(n5);
	expect(n5.next).toEqual(n6);
	expect(n6.next).toEqual(n7);

	expect(root.next).toBeNull();
	expect(n3.next).toBeNull();
	expect(n7.next).toBeNull();
});
