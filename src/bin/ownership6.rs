fn main() {
    let my_stack_value: i32 = 2;
    let my_integer_reference = &my_stack_value;

    println!("my my_integer_reference is : {}", *my_integer_reference);

    let my_heap_value: String = String::from("Toyota");

    let my_heap_reference = &my_heap_value;

    println!("my *my_heap_reference is : {}", *my_heap_reference);

    drop(my_heap_value);

}
