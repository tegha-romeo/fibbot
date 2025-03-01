use fib::fibonacci;
use octocrab::Octocrab;
use std::env;
use extract::extract_numbers;
use get_pull_request::get_pr;
use post::post_comment;

mod get_pull_request;
mod extract;
mod fib;
mod post;



#[tokio::main]
async fn main() {
    
    let pr_number: u64 = env::var("PR_NUMBER")
    .expect("PR_NUMBER not set")
    .parse::<u64>()
    .expect("Invalid PR_NUMBER");

    // Simulated pull request content (Replace this with actual PR content fetching logic)
    let pr_content = get_pr(pr_number).await;

    if pr_content.is_empty() {
        println!("No numbers found in this pull_request.");
    }
    let mut response =
        String::from("#### Fibonacci output of each number in the pull_request is:\n");
    for &num in &pr_content {
        let fib = fibonacci(num);
        response.push_str(&format!("- Fibonacci({}) = {}\n", num, fib));
    }
        if let Err(e) = post_comment(&response).await {
            eprintln!("Error posting comment: {}", e);
        }

}
