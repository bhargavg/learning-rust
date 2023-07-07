use std::{ops::Deref, vec};

fn main() {
    using_deref_operator();
    using_deref_trait_to_access_inner_value_simple();
    using_deref_trait_to_access_inner_value_complex();
    using_deref_trait_to_invoke_methods_of_inner_value();

    // Deref coercion occurs in following cases:
    // 1. From     `&T` to     `&U` when `T:    Deref<Target=U>`
    // 2. From `&mut T` to     `&U` when `T:    Deref<Target=U>`
    // 3. From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
    string_str_coercion_fails();
}

fn using_deref_operator() {
    let x = vec![1, 2, 3];
    let y = Box::new(&x);

    // The following fails with error: "Can't compare `Vec{integer}` with `Box<&Vec<{integer}>>`"
    // ```
    // assert_eq!(x, y);
    // ```
    // But, we can access the inner value by dereferencing using `*` operator.
    //
    // Box acts like a pointer. In above case, `y` is a pointer, containing a pointer to a `Vec`.
    // Hence, `y` is a double pointer and we can peel it as follows:
    //  *y = &Vec<i32>
    // **y = Vec<i32>
    assert_eq!(x, **y);
}

fn using_deref_trait_to_access_inner_value_simple() {
    #[derive(Debug)]
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(value: T) -> MyBox<T> {
            MyBox(value)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = vec![1, 2, 3];
    let y = MyBox::new(MyBox::new(vec![1, 2, 3]));
    // Compiler can only dereferene a `&T`.
    // For custom types, we have to help the compiler in dereferencing.
    //
    // In other words,
    // ```
    // *x => *(x.deref())            => This invokes `deref(&x)`, which returns a ref to pointee.
    //    => *(REF_TO_INNER_MEMBER)  => This invokes `deref(&x)` on `&T`, which is already defined for us.
    //    => VALUE_OF_INNER_MEMBER
    // ```
    assert_eq!(x, **y);
}

#[allow(clippy::deref_addrof)]
fn using_deref_trait_to_access_inner_value_complex() {
    struct X {
        val: i32,
    }

    // Add the ability to dereference `X` as `i32`
    impl Deref for X {
        type Target = i32;

        fn deref(&self) -> &Self::Target {
            &self.val
        }
    }

    // Reference of container => Reference of contained value
    {
        fn hi(val: &i32) {
            println!("{val}");
        }

        hi(&X { val: 5 });
    }

    // value of container => value of contained value
    {
        fn hi(val: i32) {
            println!("{val}");
        }

        // `**&X` => `*X`
        //        => `i32`
        hi(**&X { val: 5 });

        let a = X { val: 5 };
        let b = &a;

        // `**b` => `**&X`
        //       => `*X`
        //       => `i32`
        hi(**b);
    }
}

fn using_deref_trait_to_invoke_methods_of_inner_value() {
    struct MyBox {
        val: MyValue,
    }

    struct MyValue {
        val: i32,
    }

    impl MyValue {
        fn method_of_inner_value(&self) {
            println!("MyValue::foo => {}", self.val);
        }
    }

    impl Deref for MyBox {
        type Target = MyValue;

        fn deref(&self) -> &Self::Target {
            &self.val
        }
    }

    let bx = MyBox {
        val: MyValue { val: 5 },
    };

    // `MyBox` doesn't have this method, and yet this works.
    bx.method_of_inner_value();

    // It's because we declare that `MyBox` can be dereferenced as `MyValue`.
    // ie., `*(MyBox)` can be converted to `&MyValue` and the above call
    // loosely transitions to:
    (*bx).method_of_inner_value();
}

fn string_str_coercion_fails() {
    trait TraitExample {}
    impl<'a> TraitExample for &'a str {}

    #[allow(unused_variables, non_snake_case)]
    fn function_that_takes_TraitExample<T: TraitExample>(args: T) {}

    let string = String::from("This is a String");
    let str = "This is a slice";

    // This works
    function_that_takes_TraitExample(str);

    // This fails
    // ```
    // function_that_takes_TraitExample(string);
    // ```
    // Rust doesn't have enough information to perform `Deref` coersion.
    //
    // There are two ways to make it work:
    // 1. Use `String::as_str` function.
    function_that_takes_TraitExample(string.as_str());
    // 2. Pass `&*` to dereference `String` to `str` and then take the reference.
    function_that_takes_TraitExample(&*string)
}
