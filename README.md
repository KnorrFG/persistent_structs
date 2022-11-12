# Persitent Structs 

A small derive Macro for structs, that generates a `with_<name>` and `update_<name>`
method for each field of a struct, e.g:

```rust
use persistent_structs::PersistentStruct;

#[derive(PersistentStruct, PartialEq)]
struct Foo {
    pub foo: u8,
}

fn main() {
    let foo = Foo { foo: 1 };
    let foo = foo.with_foo(5);
    assert!(foo == Foo { foo: 5 });

    let foo = foo.update_foo(|x| x + 1);
    assert!(foo.foo == 6);
}
```

install via:

```
  cargo add persistent-structs
```

(requires cargo-add, otherwise figure out the current version number in at 
[docs.rs](docs.rs/persistent-structs), and add it manually)
