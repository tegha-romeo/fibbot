use std::env;

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
// fn main() {
//     let pr_content = "This PR includes changes related to issue 5 and 13.";
//     let numbers = extract_numbers(pr_content);
//     let fib_results: Vec<String> = numbers.iter().map(|&n| format!("Fib({}) = {}", n, fibb(n))).collect();
    
//     println!("Extracted Numbers: {:?}", numbers);
//     println!("Fibonacci Results: {}", fib_results.join(", "));
    //   }
    



fn fibb(n: u32) -> u32 {
    if n == 0 {
        return 1
    }else if n==1{
        return 1;
    }else {
        fibb(n-1) + fibb(n-2)
    }
}

use octocrab::Octocrab;


#[tokio::main]
async fn main() {
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let pr_number = env::var("GITHUB_PR_NUMBER").expect("GITHUB_PR_NUMBER not set");

    let numbers = extract_numbers("PR contains number 8 and 21.");
    println!("extracted numbers are {:?}", numbers);
    let fib_results: Vec<String> = numbers.iter().map(|&n| format!("Fib({}) = {}", n, fibb(n))).collect();
    let comment = format!("Fibonacci results: {}", fib_results.join(", "));

    println!("fibo of numbers are {:?}", fib_results);

    let octocrab = Octocrab::builder().personal_token(github_token).build().unwrap();
    
    octocrab.issues(repo.split('/').next().unwrap(), repo.split('/').nth(1).unwrap())
        .create_comment(pr_number.parse::<u64>().unwrap(), comment)
        .await
        .expect("Failed to post comment");
}