mod sort;
mod search;

fn main() {
    // let numbers = vec![1,8,3,6,5,4,7,2,9,10];
    let letters = vec!['a', 'b', 'c', 'd', 'e'];
    let value = 'f';

    let search_result = search::binary_search(&letters, value);

    match search_result {
        Some(_) => println!("{value} found."),
        None => println!("{value} not found."),
    }
}

