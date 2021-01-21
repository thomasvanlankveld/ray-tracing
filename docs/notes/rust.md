# Rust

## Shared pointers

> "Either `Rc` or `Arc` is the replacement for `shared_ptr`."

- [What is the Rust equivalent of C++'s shared_ptr?](https://stackoverflow.com/questions/49834414/what-is-the-rust-equivalent-of-cs-shared-ptr#answer-49834496) @ Stack Overflow

Other sources:

- [rust shared pointer at DuckDuckGo](https://duckduckgo.com/?t=canonical&q=rust+shared+pointer&atb=v204-1&ia=web)
- [scope - What is the Rust equivalent of C++'s shared_ptr? - Stack Overflow](https://stackoverflow.com/questions/49834414/what-is-the-rust-equivalent-of-cs-shared-ptr#answer-49834496)
- [Pointers in Rust, a guide | Next.js Blog Example with Markdown](https://steveklabnik.com/writing/pointers-in-rust-a-guide)
- [Working with Shared pointers in Rust: Challenges and Solutions [Tutorial] | Packt Hub](https://hub.packtpub.com/shared-pointers-in-rust-challenges-solutions/)
- [std::rc::Rc - Rust](https://doc.rust-lang.org/std/rc/struct.Rc.html)
- [std::sync::Arc - Rust](https://doc.rust-lang.org/std/sync/struct.Arc.html)

On "Smart pointers":

- [Smart Pointers - The Rust Programming Language](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Rc<T>, the Reference Counted Smart Pointer - The Rust Programming Language](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [RefCell<T> and the Interior Mutability Pattern - The Rust Programming Language](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
- [Reference Cycles Can Leak Memory - The Rust Programming Language](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)

## Polymorphism

When an example in C++ asks you to write an abstract base class, intuitively a beginner in Rust will think of using a trait. However, this creates a problem when the C++ example then goes on to use this abstract type polymorphically:

```cpp
    public:
        std::vector<shared_ptr<hittable>> objects;
```

The following will not compile, because Rust doesn't know at compile time what the size of the object is.

```rust
pub struct HittableList {
    objects: Vec<dyn Hittable>,
}
```

It may be that this would also have been a problem in C++, if there had not been a `shared_ptr` around the `hittable`. In any case, the solution is to heap-allocate the objects of type `Hittable`, and use pointers to the heap allocated memory when we want to use them.

```rust
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}
```

The important source is in the Rust book:

- [Using Trait Objects That Allow for Values of Different Types - The Rust Programming Language](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types)

Types that should similarly to `Box` in this context include `Rc` and `Arc`, both of which are also heap-allocated.

Other sources (not sure if useful):

- [rust vec of trait at DuckDuckGo](https://duckduckgo.com/?t=canonical&q=rust+vec+of+trait&atb=v204-1&ia=web)
- [Rust Trait objects in a vector - non-trivial... - DEV Community](https://dev.to/magnusstrale/rust-trait-objects-in-a-vector-non-trivial-4co5)
- [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=c11f8f2b3e5481faafb6367226de8c1e)
- [casting - Rust Vector of Traits: cast each trait - Stack Overflow](https://stackoverflow.com/questions/27073799/rust-vector-of-traits-cast-each-trait)
- [Type-casting arrays/vectors in Rust - Stack Overflow](https://stackoverflow.com/questions/16755181/type-casting-arrays-vectors-in-rust)
