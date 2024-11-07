use std::collections::HashSet;

fn main() {
    // 创建和插入
    let mut set = HashSet::new();
    set.insert("apple");set.insert("banana");set.insert("orange");
    println!("{set:?}");
    // 是否包含
    println!("{}", set.contains("apple"));
    // 删除
    set.remove("apple");
    println!("{set:?}");
    // 遍历
    for item in &set {
        println!("{}", item);
    }
    // 集合操作
    let set2:HashSet<_> = ["apple","banana"].iter().cloned().collect();
    let union:HashSet<_> = set.union(&set2).cloned().collect();
    println!("{union:?}");
    let intersection:HashSet<_> = set.intersection(&set2).cloned().collect();
    println!("{intersection:?}");
    let difference:HashSet<_> = set.difference(&set2).cloned().collect();
    println!("{difference:?}");
}
