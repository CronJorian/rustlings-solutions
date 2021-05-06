// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    let vec0 = Vec::new();
    
    // Solution 1: copy vec0
    let vec2 = vec0.clone();
    let mut vec1 = fill_vec(vec2);
    
    // Solution 2 (part 1): borrow vec
    // let mut vec1 = fill_vec(&vec0);
    
    // Solution 3: mutably borrow
    // let mut vec0 = Vec::new();
    // fill_vec(&mut vec0);
    
    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

/* 
// Solution 2 (part 2): borrow vec

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
        let mut vec = vec.clone();
    
        vec.push(22);
        vec.push(44);
        vec.push(66);
    
    vec
}
*/

/* 
// Solution 3: mutably borrow vec

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
*/
