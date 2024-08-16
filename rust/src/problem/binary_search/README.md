# Here I've tried to collect most useful shapes and forms of Binary Search.

## 1. Sorted vector insertion.

```rust
fn insert<T: PartialOrd>(vec: &mut Vec<T>, item: T) {
    let mut l = 0;
    let mut r = vec.len();

    while l < r {
        let m = l + (r - l) / 2;

        if vec[m] == item {
            vec.insert(m, item);
            return;
        }

        if vec[m] > item {
            r = m;
        } else {
            l = m + 1;
        }
    }

    vec.insert(l, item);
}
```

1. The key insight here is that by allowing `r` to be equal to `vec.len()`,
   we create a _"virtual"_ position at the end of the vector where elements larger than all existing elements would be
   inserted:

    ```text
    [1, 2] _ // insert 3
     ^     ^
     l     r
     
     // in the last iteration
    [1, 2] __
           ^^
           lr // l == r -> loop break -> insert in l pos
    ```

2. Strict loop condition efficiently handles empty vector case when `l`
   and `r` are both zeros: the loop is skipped entirely.

This form is beautiful from every perspective. It gracefully handles all cases:

- empty vector insertion
- start/end insertion
- insertion when there is no exact match (insertion at closest)
- when there is exactly one match
- when there are few matches

Additionaly, this form of BS doesn't do index subtraction: we use `r = m` instead of `r = m - 1`, which prevents
unsigned
integer underflow. So it is safe to use `usize`

## 2. Leftmost item search

```rust
fn leftmost<T: PartialOrd>(vec: &[T], target: &T) -> usize {
    let mut l = 0;
    let mut r = vec.len();

    while l < r {
        let m = l + (r - l) / 2;

        if &vec[m] >= target {
            r = m;
        } else {
            l = m + 1;
        }
    }

    l
}
```

Returns either target position if present or insertion position otherwise.

## 3. Rightmost item search

```rust
fn rightmost<T: PartialOrd>(vec: &[T], target: &T) -> usize {
    let mut l = 0;
    let mut r = vec.len();

    while l < r {
        let m = l + (r - l) / 2;

        if &vec[m] > target {
            r = m;
        } else {
            l = m + 1;
        }
    }

    l
}
```

This approach is slightly different from the leftmost one: here we return the leftmost position of the element that is
strictly greater than `target`. Thus, to get position of the rightmost item **(if present)**, we need to subtract 1 from
the return
value:

```rust
fn find_rightmost<T: PartialOrd>(vec: &[T], target: &T) -> Option<usize> {
    let idx = rightmost(vec, target);

    if idx > 0 && &vec[idx - 1] == target {
        Some(idx - 1)
    } else {
        None
    }
}
```

---

There is another approach that find the **exact** position, but it works only when element **is in the vector**:

```rust
fn rightmost<T: PartialOrd>(vec: &[T], target: &T) -> usize {
    let mut l = 0i32;
    let mut r = vec.len() as i32 - 1;
    let mut pos = -1;

    while l < r {
        let m = l + (r - l) / 2;

        if nums[m as usize] <= target {
            if nums[m as usize] == target { // pos changes only when target is detected
                pos = m;
            }
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    pos
}
```

IMO looks more conviluted due to the type casting and nested `if` statements.