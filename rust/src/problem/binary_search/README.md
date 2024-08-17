# Variations of Binary Search

## 1. Basic BS

```rust
fn binary_search<T: Ord>(vec: &[T], target: T) -> Option<usize> {
    let mut l = 0;
    let mut r = vec.len();

    while l < r {
        let m = l + (r - l) / 2;

        match vec[m].cmp(&target) {
            Ordering::Less => {
                l = m + 1;
            }
            Ordering::Greater => {
                r = m;
            }
            Ordering::Equal => {
                return Some(m);
            }
        }
    }

    None
}
```

An alternative way:

```rust
fn binary_search<T: Ord>(vec: &[T], target: T) -> Option<usize> {
    let mut l = 0;
    let mut r = vec.len() - 1;

    while l != r {
        let m = (l + r).div_ceil(2);

        if vec[m] > target {
            r = m - 1;
        } else {
            l = m;
        }
    }

    if vec[l] == target {
        Some(l)
    } else {
        None
    }
}
```

[Wikipedia](https://en.wikipedia.org/wiki/Binary_search#Alternative_procedure): _"This results in a faster
comparison loop, as one comparison is eliminated per iteration, while it requires only one more iteration on average"_

## 2. Leftmost search

```rust
fn leftmost<T: Ord>(vec: &[T], target: &T) -> usize {
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

This is the most flexible variation. While previous two could return any index if target is present multiple times,
this one
returns the first occurence index. Additionally, this form of BS returns the insertion position if target isn't there:

```
leftmost(&[1, 2, 3], &3) -> 2
leftmost(&[1, 3], &2) -> 1
leftmost(&[], &2) -> 0
```

Thus, it requires us to check return value, if we need to make sure that target is found:

```rust
fn find_leftmost<T: Ord>(vec: &[T], target: &T) -> Option<usize> {
    let i = leftmost(vec, target);

    if &vec[i] == target {
        Some(i)
    } else {
        None
    }
}
```

## 3. Rightmost search

```rust
fn rightmost<T: Ord>(vec: &[T], target: &T) -> usize {
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

Here we return _the leftmost position of the element that is
strictly greater than_ `target`. Thus, to get the exact position of the last target occurence **(if present)**, we need
to subtract 1 from the return value:

```rust
fn find_rightmost<T: Ord>(vec: &[T], target: &T) -> Option<usize> {
    let i = rightmost(vec, target);

    if i > 0 && &vec[i - 1] == target {
        Some(i - 1)
    } else {
        None
    }
}
```

There is another approach that finds the **exact** position **if present**:

```rust
fn rightmost<T: Ord>(vec: &[T], target: &T) -> Option<usize> {
    let mut res = None;

    let mut l = 0i32;
    let mut r = vec.len() as i32 - 1;

    while l <= r {
        let m = l + (r - l) / 2;

        if &vec[m as usize] <= target {
            if &vec[m as usize] == target {
                res = Some(m as usize);
            }
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    res
}
```

IMO looks more convoluted due to the type casting and nested `if` statements + lacks flexibility. We could get rid of
typecasts and work
with `usize` but it would require us to add empty vector check to prevent underflow which would make this code even more
verbose.

## BS Application

A good application example is the vector insertion:

```rust
fn insert<T: Ord>(vec: &mut Vec<T>, item: T) {
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