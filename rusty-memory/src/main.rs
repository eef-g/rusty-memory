mod list;

fn main() {
    println!("Hello, world!");
    let mut list:list::List<u32> = list::List::new();
    list.push(1);
}
