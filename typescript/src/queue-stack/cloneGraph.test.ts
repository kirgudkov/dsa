import { test, expect } from "bun:test";
import { cloneGraphRecursive, cloneGraphStack, _Node } from "./cloneGraph.ts";

const node1 = new _Node(1);
const node2 = new _Node(2);
const node3 = new _Node(3);
const node4 = new _Node(4);

node1.neighbors.push(node2, node4);
node2.neighbors.push(node1, node3);
node3.neighbors.push(node2, node4);
node4.neighbors.push(node1, node3);

function testCloneFunction(cloneFunction: (node: _Node) => _Node | null) {
	const cloned_node1 = cloneFunction(node1);

	expect(cloned_node1).not.toBe(node1);
	expect(cloned_node1).not.toBe(node2);
	expect(cloned_node1).not.toBe(node3);
	expect(cloned_node1).not.toBe(node4);

	expect(cloned_node1?.val).toBe(node1.val);
	expect(cloned_node1?.neighbors.length).toBe(node1.neighbors.length);

	const cloned_node2 = cloned_node1?.neighbors[0];
	const cloned_node4 = cloned_node1?.neighbors[1];

	expect(cloned_node2).not.toBe(node2);
	expect(cloned_node4).not.toBe(node4);

	expect(cloned_node2?.val).toBe(node2.val);
	expect(cloned_node2?.neighbors.length).toBe(node2.neighbors.length);

	expect(cloned_node4?.val).toBe(node4.val);
	expect(cloned_node4?.neighbors.length).toBe(node4.neighbors.length);

	const cloned_node1_neighbor2 = cloned_node2?.neighbors[0] ?? null;
	const cloned_node1_neighbor4 = cloned_node4?.neighbors[0] ?? null;

	expect(cloned_node1_neighbor2).toBe(cloned_node1);
	expect(cloned_node1_neighbor4).toBe(cloned_node1);
}

test("Recursive", () => {
	testCloneFunction(cloneGraphRecursive);
});

test("Stack", () => {
	testCloneFunction(cloneGraphStack);
});

test("Empty graph", () => {
	expect(cloneGraphRecursive(null)).toBe(null);
	expect(cloneGraphStack(null)).toBe(null);
});

test("Single node", () => {
	{
		const node = new _Node(1);
		const cloned_node = cloneGraphRecursive(node);

		expect(cloned_node).not.toBe(node);
		expect(cloned_node?.val).toBe(node.val);
		expect(cloned_node?.neighbors.length).toBe(0);
	}

	{
		const node = new _Node(1);
		const cloned_node = cloneGraphStack(node);

		expect(cloned_node).not.toBe(node);
		expect(cloned_node?.val).toBe(node.val);
		expect(cloned_node?.neighbors.length).toBe(0);
	}
});
