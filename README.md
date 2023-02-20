implements Eq for structs, but with the ability to exclude fields from equality checking using an ignore_regex attribute

Example usage:

```rust
use struct_macro_eq;

/*
tow Dish-es are implemented to be equal here if
their carbs and fats fields match (_temp is excluded
because it starts with an underscore and hence
matches ignore_regex="^_")
*/
#[derive(struct_macro_eq::CustomEq)]
#[ignore_regex="^_"]
struct Dish {
    carbs: u64,
    fats: u64,
    _temp: u64
}

fn main() {
    let dish1 = Dish { carbs: 30, fats: 20, _temp: 30 };
    let dish2 = Dish { carbs: 30, fats: 20, _temp: 20 };
    let dish3 = Dish { carbs: 30, fats: 10, _temp: 30 };

    // dish1 and dish2 are equal
    println!("dish1 == dish2: {}", dish1 == dish2);
    // dish1 and dish3 are not equal
    println!("dish1 == dish3: {}", dish1 == dish3);
}
```

