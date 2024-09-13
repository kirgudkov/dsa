import { test, expect } from "bun:test";
import { AutocompleteSystem } from "./AutocompleteSystem";

test("AutocompleteSystem", () => {
	const obj = new AutocompleteSystem(
		["foo", "bar", "baz", "qux"],
		[5, 3, 2, 2]
	);

	expect(obj.input("f")).toEqual(["foo"]);
	expect(obj.input("b")).toEqual([]);
	expect(obj.input("#")).toEqual([]);
	expect(obj.input("f")).toEqual(["foo", "fb"]);
	expect(obj.input("b")).toEqual(["fb"]);
});

test("AutocompleteSystem2", () => {
	const obj = new AutocompleteSystem(
		["car", "cat", "cart", "cartoon"],
		[5, 4, 3, 50]
	);

	expect(obj.input("c")).toEqual(["cartoon", "car", "cat"]);
	expect(obj.input("a")).toEqual(["cartoon", "car", "cat"]);
	expect(obj.input("r")).toEqual(["cartoon", "car", "cart"]);
});
