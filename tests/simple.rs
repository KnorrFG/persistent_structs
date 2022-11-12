use persistent_structs::PersistentStruct;

#[derive(PersistentStruct, PartialEq)]
struct Foo {
    pub foo: u8,
}

#[derive(PersistentStruct)]
struct GStruct<T> {
    foo: T,
}

#[test]
fn it_works() {
    let foo = Foo { foo: 1 };
    let foo = foo.with_foo(5);
    assert!(foo == Foo { foo: 5 });

    let foo = foo.update_foo(|x| x + 1);
    assert!(foo.foo == 6);
}

#[test]
fn with_generics() {
    let foo = GStruct {
        foo: "Hello".to_string(),
    };
    let foo = foo.update_foo(|x| x + " world");
    assert!(foo.foo == "Hello world");
}
