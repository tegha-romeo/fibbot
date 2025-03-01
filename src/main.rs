use std::env;
use regex::Regex;
use octocrab::Octocrab;
use std::collections::HashMap;

// Function to extract numbers from a given text
fn extract_numbers(text: &str) -> Vec<u32> {
    let re = Regex::new(r"\b\d+\b").unwrap();
    re.find_iter(text)
        .filter_map(|digits| digits.as_str().parse::<u32>().ok())
        .collect()
}

// Optimized Fibonacci function with memoization
fn fib(n: u32, memo: &mut HashMap<u32, u32>) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }
    if let Some(&val) = memo.get(&n) {
        return val;
    }
    let result = fib(n - 1, memo) + fib(n - 2, memo);
    memo.insert(n, result);
    result
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch environment variables
    let github_repository = env::var("GITHUB_REPOSITORY").unwrap_or_else(|_| "tegha-romeo/fibbot".to_string());
    let parts: Vec<&str> = github_repository.split('/').collect();
    let (owner, repo) = (parts[0], parts[1]);
    let github_token = env::var("GITHUB_TOKEN").expect("Missing GITHUB_TOKEN");

    // Simulated pull request content (Replace this with actual PR content fetching logic)
    let pr_content = "Calculate Fibonacci for: 5, 8, 13";

    // Extract numbers and compute Fibonacci
    let numbers = extract_numbers(pr_content);
    let mut memo = HashMap::new();
    let fib_results: Vec<(u32, u32)> = numbers.iter().map(|&n| (n, fib(n, &mut memo))).collect();

    // Message to post
    let message = format!("Fibonacci results: {:?}", fib_results);

    // Initialize Octocrab with authentication
    let octocrab = Octocrab::builder().personal_token(github_token).build()?;
    
    // Simulated PR number (Replace with real PR number fetching logic)
    let pr_number = 1;
    
    // Post comment to PR
    let response = octocrab.issues(owner, repo).create_comment(pr_number, &message).await?;
    
    println!("Comment posted: {:?}", response);
    Ok(())
}
