import { test, expect } from "bun:test";
import { lowestCommonAncestor, lowestCommonAncestor_h2, lowestCommonAncestor_h_const_s, _Node } from "./lowestCommonAncestorIII.ts";

test("lowestCommonAncestor", () => {
	const root = new _Node(3);
	const p = new _Node(7);
	const q = new _Node(8);

	const p1 = new _Node(5);
	const p2 = new _Node(1);

	const p3 = new _Node(6);
	const p4 = new _Node(2);

	const p6 = new _Node(4);

	const p7 = new _Node(0);

	root.left = p1;
	root.right = p2;
	p1.parent = root;
	p2.parent = root;

	p1.left = p3;
	p1.right = p4;
	p3.parent = p1;
	p4.parent = p1;

	p4.left = p;
	p4.right = p6;
	p.parent = p4;
	p6.parent = p4;

	p2.left = p7;
	p2.right = q;
	p7.parent = p2;
	q.parent = p2;

	expect(lowestCommonAncestor(p, q)).toBe(root);
	expect(lowestCommonAncestor(p3, p4)).toBe(p1);

	expect(lowestCommonAncestor_h_const_s(p, q)).toBe(root);
	expect(lowestCommonAncestor_h_const_s(p3, p4)).toBe(p1);

	expect(lowestCommonAncestor_h2(p, q)).toBe(root);
	expect(lowestCommonAncestor_h2(p3, p4)).toBe(p1);
});
