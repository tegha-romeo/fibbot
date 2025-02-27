// use std::env;

// fn main() {
//     let enable_fibb: bool = env::var("INPUT_ENABLE_FIBB")
//         .unwrap_or("true".to_string())
//         .parse()
//         .unwrap_or(true);

//     let max_threshold: u32 = env::var("INPUT_MAX_THRESHOLD")
//         .unwrap_or("100".to_string())
//         .parse()
//         .unwrap_or(100);

//     println!("enable_fibb: {}", enable_fibb);
//     println!("Max threshold is {}", max_threshold);
// }


use regex::Regex;
fn extract_numbers(text: &str) -> Vec<u32> {
    let re = Regex::new(r"\b\d+\b").unwrap();
    re.find_iter(text)
    .filter_map(|digits| digits.as_str().parse::<u32>().ok())
    .collect()
}
fn main() {
    let text = "PR contains numbers: 3, 5, and 8.";
    let numbers = extract_numbers(text);
    println!("Extracted numbers: {:?}", numbers);
      // Compute Fibonacci numbers
      let fib_results: Vec<(u32, u32)> = numbers.iter().map(|&num| (num, fibb(num))).collect();

      // Print results
      for (num, fib) in &fib_results {
          println!("Fibonacci({}) = {}", num, fib);
      }
    
}


fn fibb(n: u32) -> u32 {
    if n == 0 {
        return 1
    }else if n==1{
        return 1;
    }else {
        fibb(n-1) + fibb(n-2)
    }
}
