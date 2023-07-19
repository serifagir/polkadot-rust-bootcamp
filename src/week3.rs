
struct FilterConditions<T> {
        condition: T
}

impl<T:PartialEq> FilterConditions<T>{
        fn is_match(&self, item: &T) -> bool {
                &self.condition == item
        }
}

// main.rs

// main.rs

fn custom_filter<'a, T: PartialEq>(collection: &'a [T], condition: &FilterConditions<T>) -> Vec<&'a T> {
    collection.iter().filter(|&item| condition.is_match(item)).collect()
}



fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let condition = FilterConditions { condition: 4 };

    let filtered_numbers = custom_filter(&numbers, &condition);

    println!("Original numbers: {:?}", numbers);
    println!("Filtered numbers: {:?}", filtered_numbers);
}
