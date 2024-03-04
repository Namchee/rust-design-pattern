// Iterator is a design pattern that allows traversal of
// an object without exposing its internal details.
//
// A good example of it is array iterator, where we don't
// have to jump to correct address when iterating arrays.

fn _main() {
    let arr = [1, 2, 3];
    for num in arr.iter() {
        print!("{} -> ", num);
    }
}
