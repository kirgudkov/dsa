import { test, expect } from "bun:test";
import { replaceWords } from "./replaceWords";

test("replaceWords", () => {
	const dictionary = ["cat", "bat", "rat"];
	const sentence = "the cattle was rattled by the battery";
	expect(replaceWords(dictionary, sentence)).toBe("the cat was rat by the bat");
});

test("replaceWords", () => {
	const dictionary = ["a", "b", "c"];
	const sentence = "aadsfasf absbs bbab cadsfafs";
	expect(replaceWords(dictionary, sentence)).toBe("a a b c");
});

test("replaceWords", () => {
	const dictionary = ["a", "aa", "aaa", "aaaa"];
	const sentence = "a aa a aaaa aaa aaa aaa aaaaaa bbb baba ababa";
	expect(replaceWords(dictionary, sentence)).toBe("a a a a a a a a bbb baba a");
});
