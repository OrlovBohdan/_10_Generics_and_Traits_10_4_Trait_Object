#[test]

/*
trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// IMPLEMENT below with generics.
fn static_dispatch...

// Implement below with trait objects.
fn dynamic_dispatch...

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!");
}
*/

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!");
}
trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// Реализация со статической диспетчеризацией с использованием дженериков.
fn static_dispatch<T: Foo>(x: T) {
    println!("{}", x.method());
}

// Реализация с динамической диспетчеризацией с использованием трейтов.
fn dynamic_dispatch(x: &dyn Foo) {
    println!("{}", x.method());
}

/*
Статическая диспетчеризация (static_dispatch):

Используется дженерик T, который ограничивается трейтами с помощью T: Foo.
Компилятор во время компиляции знает точный тип, и вызовы метода будут встроены.
Динамическая диспетчеризация (dynamic_dispatch):

В качестве параметра используется &dyn Foo, что позволяет использовать объекты, реализующие Foo, через указатель (в данном случае, ссылку).
Динамическая диспетчеризация работает во время выполнения, используя таблицу виртуальных функций.
*/