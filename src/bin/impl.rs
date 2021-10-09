/*

How it works:

There are (3) types of implementation:

1. Standalone (a function that is not part of a type).
2. Inherent implementation (with `impl x`) implements functionality for a type `x`.
3. Trait implementation (with `impl y for x`) implements functionality for a trait (an interface) `y` on a type `x`.

For example (below), only Cat can `meow()` and only Dog can `bark()`, but both can `speak()`.

*/

// Define types for Cat and Dog
#[derive(Debug)]
struct Cat {
    name: String,
}

#[derive(Debug)]
struct Dog {
    name: String,
}


// An inherent implementation for type: Cat
impl Cat {
    fn meow(&self) -> String {
        String::from("Meow!")
    }
}

// An inherent implementation for type: Dog
impl Dog {
    fn bark(&self) -> String {
        String::from("Ruff!")
    }
}

// A shared trait (interface) for both types
trait Speak {
    fn speak(&self) -> String;
}

// A trait implementation of Speak on type Cat
impl Speak for Cat {
    fn speak(&self) -> String {
        self.meow()
    }
}

// A trait implementation of Speak on type Dog
impl Speak for Dog {
    fn speak(&self) -> String {
        self.bark()
    }
}


#[cfg(test)]
mod test_cat {
    use super::Speak;
    use super::Cat;

    // Test inherent implementation for Cat
    #[test]
    fn test_cat_meow() {
        let cat = Cat {name: String::from("Bob")};
        assert_eq!(cat.meow(), "Meow!")
    }

    // Test trait implementation of Speak for Cat
    #[test]
    fn test_cat_speak() {
        let cat = Cat {name: String::from("Bob")};
        assert_eq!(cat.speak(), "Meow!")
    }

}

#[cfg(test)]
mod test_dog {
    use super::Speak;
    use super::Dog;

    // Test inherent implementation for Dog
    #[test]
    fn test_dog_bark() {
        let dog = Dog {name: String::from("Herman")};
        assert_eq!(dog.bark(), "Ruff!")
    }

    // Test trait implementation of Speak for Dog
    #[test]
    fn test_dog_speak() {
        let dog = Dog {name: String::from("Herman")};
        assert_eq!(dog.speak(), "Ruff!")
    }

}
