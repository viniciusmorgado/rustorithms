use std::collections::LinkedList;
use std::collections::linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();

    list.push_back(32);
    list.push_back(42);
    list.push_front(92);
    list.push_front(23);
    list.push_back(42);
    list.push_back(42);

    let fast_iter: linked_list::Iter<'_, u32> = list.iter();

    let mut fast: &u32;
    for value in fast_iter.step_by(2) {
        fast = value;
        println!("Fast Iterator: {} \n", fast);
    }

    let slow_iter: linked_list::Iter<'_, u32> = list.iter();

    let mut slow: &u32;
    for value in slow_iter {
        slow = value;
        println!("Slow Iterator: {} \n", slow);
    } 
}
