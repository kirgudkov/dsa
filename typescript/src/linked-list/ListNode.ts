export class ListNode {
	val: number;
	next: ListNode | null;

	constructor(val?: number, next?: ListNode | null) {
		this.val = (val === undefined ? 0 : val);
		this.next = (next === undefined ? null : next);
	}

	// print() {
	// 	let result = this.val.toString();
	// 	let curr = this.next;
	//
	// 	while (curr && curr !== this) {
	// 		result += " -> " + curr.val;
	// 		curr = curr.next;
	// 	}
	//
	// 	console.log(result);
	// }
}
