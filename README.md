# Reading Progress for *The Rust Programming Language*

Record my reading progress for [*The Rust Programming Language*](https://doc.rust-lang.org/stable/book/) to prepare for the summer internship.

Estimated completion date: 05/10/2023

https://rust-book.cs.brown.edu/ 
This interactive version from Brown University turns out to be a better source. It clearly shows memory layouts for some confusing code snippets, and has quizzes for comprehension check.

## Progress Bar

- [x] Chapter 1: Getting Started (02/28/2023)
- [x] Chapter 2: Guessing Game (03/02/2023)
- [x] Chapter 3: Common Programming Concepts (03/05/2023)
- [x] Chapter 4: Understanding Ownership (03/06/2023, 04/22/2023)
- [x] Chapter 5: Struct (04/21/2023)
- [x] Chapter 6: Enum (04/22/2023)
- [x] Chapter 7: Packages, Crates, and Modules (04/23/2023)
- [ ] Chapter 8:
- [ ] Chapter 9:
- [ ] Chapter 10:
- [ ] Chapter 11:
- [ ] Chapter 12:
- [ ] Chapter 13:
- [ ] Chapter 14:
- [ ] Chapter 15:
- [ ] Chapter 16:
- [ ] Chapter 17:
- [ ] Chapter 18:
- [ ] Chapter 19:
- [ ] Chapter 20:

## Cheat Book
### Reference
1. Use references when you want to share access to a value without transferring ownership. This can help reduce memory usage and avoid unnecessary cloning of values.
2. Use mutable references (`&mut`) when you need to modify the borrowed value. Remember that you can have multiple immutable references or only one mutable reference at a time.
3. When iterating over a collection, you can use references to avoid moving or copying the elements. For example:
    ```rust
    for (key, value) in &my_hashmap {
        // ...
    }
    ```
4. When using methods that borrow values, such as `get()` for a `HashMap`, the returned value will be a reference. You should use the appropriate reference type when working with these values.
5. When passing values to a function, prefer using references if the function doesn't need to own the value or if you want to avoid cloning or copying the value.