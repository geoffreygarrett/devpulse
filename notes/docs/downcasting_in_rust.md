## Downcasting in Rust

Downcasting in Rust allows you to recover the concrete type from a trait object. This is particularly useful in
scenarios where you have a reference to a value by way of a trait object, and you need to determine its actual type.

### Using `Any`

The `Any` trait enables objects to be treated generically and inspected at runtime to determine their actual type. It is
provided by the standard library and is automatically implemented for any type that has `'static` lifetime.

**Key Points**:

- `Any` provides the `is` and `downcast_ref` methods to check and retrieve references to the concrete type,
  respectively.
- You need to use a `Box<dyn Any>` to hold the trait object for heap allocation.

**Basic Usage**:

1. **Define the trait**:
   Ensure your trait extends `Any`:
   ```rust
   use std::any::Any;

   trait MyTrait: Any {
       fn as_any(&self) -> &dyn Any;
   }
   ```

2. **Implement the trait**:
   Provide implementations for the concrete types:
   ```rust
   struct MyStruct;

   impl MyTrait for MyStruct {
       fn as_any(&self) -> &dyn Any {
           self
       }
   }
   ```

3. **Perform downcasting**:
   Check and downcast the trait object:
   ```rust
   fn test_downcasting(trait_obj: Box<dyn MyTrait>) {
       // Check if the trait object is of type MyStruct using `is`
       if trait_obj.as_any().is::<MyStruct>() {
           println!("It's MyStruct!");
       } else {
           println!("Not MyStruct!");
       }

       // Alternatively, use `downcast_ref` to try obtaining a reference
       if let Some(ref s) = trait_obj.as_any().downcast_ref::<MyStruct>() {
           println!("It's definitely MyStruct!");
       } else {
           println!("Still not MyStruct!");
       }
   }
   ```

### Direct Downcasting Approach

As an alternative, you can avoid the indirectness of `as_any` by implementing type-specific methods directly on the
trait:

```rust
trait MyTrait {
    fn as_my_struct(&self) -> Option<&MyStruct>;
}

impl MyTrait for MyStruct {
    fn as_my_struct(&self) -> Option<&MyStruct> {
        Some(self)
    }
}
```

This method allows you to bypass the need for `Any` if you know the set of types in advance.

### Using the `mopa` Crate

For scenarios where you want the simplicity of `Any` along with additional trait functionality, consider using
the `mopa` crate, which allows traits to be more flexible by combining `Any`-like downcasting with other methods.

**Example**:

```rust
use mopa::Mopa;

mopafy!(MyTrait);

struct MyStruct;
impl MyTrait for MyStruct {}

fn main() {
    let trait_obj = Mopa::new(Box::new(MyStruct) as Box<dyn MyTrait>);
    if trait_obj.downcast_ref::<MyStruct>().is_some() {
        println!("It's MyStruct!");
    } else {
        println!("Not MyStruct!");
    }
}
```

### Conclusion

Downcasting is a powerful feature in Rust that allows for type safety and flexibility in handling trait objects. It
should be used judiciously to maintain the safety and clarity of the code.

### Sources

- [How to get a reference to a concrete type from a trait object - Stack Overflow](https://stackoverflow.com/questions/33687447/how-to-get-a-reference-to-a-concrete-type-from-a-trait-object)
- [downcast-rs on docs.rs](https://docs.rs/downcast-rs/latest/downcast_rs/)
