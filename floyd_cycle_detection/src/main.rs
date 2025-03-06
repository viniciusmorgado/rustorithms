use std::collections::LinkedList;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();

    list.push_front(1);
    list.push_front(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);
    list.push_back(6);

    let mut fast_iter = list.iter().step_by(2);
    let mut slow_iter = list.iter();

    fast_iter.next();

    while let (Some(fast), Some(slow)) = (fast_iter.next(), slow_iter.next()) {
        if fast == slow {
            println!("Cycle detected: {} == {}", fast, slow);
            break;
        } else {
            println!("No cicle detected!");
        }
    }

    println!("Linked List: {list:?}")
}
