use std::thread;

const THRESHOLD: usize = 3;

fn sqare (x: i32) -> i32 { 
    x * 2 
}

fn uppercase (x: &str) -> String { 
    x.to_uppercase().to_string()
}

fn helper <T, R>(input: Vec<T> , f: fn(T) -> R) -> Vec<R>
where 
    T: Send + Sync + std::fmt::Debug + Copy + 'static,
    R: Send + Sync + std::fmt::Debug + 'static
{

    if input.len() <= THRESHOLD {
        println!("processed without thread {:?}", input);
        return input.into_iter().map(f).collect();
    }

    let chanks = input.chunks(THRESHOLD);
    let mut children = vec![];
   
    for chank in chanks {
        let owned_chank = chank.to_vec();
        children.push(thread::spawn(move || -> Vec<R> {
            println!("hello from thread {:?}", owned_chank);
            owned_chank.iter()
                .map(|&item| f(item) )
                .collect()
        }))
    }

    children
        .into_iter()
        .flat_map(|thread_result| thread_result.join().unwrap())
        .collect()
}


fn main() {
    println!("======== Results w/o threads ========");

    let res_i32_1 = helper(vec![10, 20, 30], sqare);
    println!("{:?}", res_i32_1);

    println!("======== Results with threads ========");

    let res_i32_2 = helper(vec![10, 20, 30, 40, 50, 60, 70], sqare);
    println!("{:?}", res_i32_2);

    println!("======== Results w/o threads ========");

    let res_str_1 = helper(vec!["aa", "bb", "cc"], uppercase);
    println!("{:?}", res_str_1);

    println!("======== Results with threads ========");

    let res_str_2 = helper(vec!["aa", "bb", "cc", "dd", "ee", "ff"], uppercase);
    println!("{:?}", res_str_2);
}