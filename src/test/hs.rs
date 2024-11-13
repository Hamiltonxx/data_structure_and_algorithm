use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert("apple");
    set.insert("banana");

    let set2:HashSet<_> = ["apple","orange"].iter().cloned().collect(); // iter() creates an iterator of
                                                               // reference to &str (&&str)
    // cloned(): dereference and copy
    // if not want copy
    let set3: HashSet<_> = ["apple", "orange"].iter().map(|&x| x).collect();
    let set4: HashSet<_> = HashSet::from(["apple","orange"]);
    let union:HashSet<_> = set.union(&set4).cloned().collect();
    println!("{:?}", union);
}
