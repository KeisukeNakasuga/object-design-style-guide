use std::string::ToString;

struct Foo {
    some_number: i16,
    some_string: String,
}

impl Foo {
    const SOME_NUMBER_CONST: i16 = 1000;

    pub fn new(some_number: i16, some_string: String) -> Self {
        println!("call new");
        Foo { some_number, some_string }
    }

    pub fn some_method(&self) {
        println!("call some_method");
    }

    fn some_method_2() {
        println!("call some_method_2");
    }
}

struct Mutable {
    some_number: i32,
}

impl Mutable {
    pub fn new(some_number: i32) -> Self {
        Mutable { some_number }
    }

    pub fn increase(&mut self) {
        self.some_number += 1;
    }
}

struct Immutable {
    some_number: i32
}

impl Immutable {
    pub fn new(some_number: i32) -> Self {
        Immutable{ some_number }
    }

    pub fn increase(&self) {
        Self::new(self.some_number + 1);
    }
}

struct Foo2 {}

impl Foo2 {
    pub fn some_method() {
        let logger = Logger {};
    }
}

struct Foo3 {
    logger: Logger,
}

impl Foo3 {
    pub fn new(&self, logger: Logger) -> Self {
        Foo3 {
            logger
        }
    }

    pub fn some_method(&self) {
        self.logger.some_method();
    }
}

struct Logger {}

impl Logger {
    pub fn some_method() {
        println!("call Logger some_method");
    }
}

trait FooTraint {
    fn foo();
}

struct Bar {}

impl FooTraint for Bar {
    fn foo() {
        todo!()
    }
}

fn main() {
    let object1 = Foo::new(100, "Foo1".to_string());
    let object2 = Foo::new(200, "Foo2".to_string());
    object1.some_method();

    let mut object3 = Mutable::new(10);
    object3.increase();

    let object4 = Immutable::new(10);
    object4.increase();

    let object5: dyn FooTraint = Bar{};

    println!("DEBUG");
}
