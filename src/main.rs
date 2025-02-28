use std::env;
use regex::Regex;
use octocrab::Octocrab;


fn extract_numbers(text: &str) -> Vec<u32> {
    let re = Regex::new(r"\b\d+\b").unwrap();
    re.find_iter(text)
    .filter_map(|digits| digits.as_str().parse::<u32>().ok())
    .collect()
}
  



// fn fibb(n: u32) -> u32 {
//     if n == 0 {
//         return 1
//     }else if n==1{
//         return 1;
//     }else {
//         fibb(n-1) + fibb(n-2)
//     }
// }





#[tokio::main]
async fn main() {
    // Fetch the GitHub token from environment variables
    let github_token = env::var("GITHUB_TOKEN").expect("Missing GITHUB_TOKEN");

    // Fetch repository owner and name dynamically (replace as needed)
    let owner = "tegha-romeo"; 
    let repo = "fibbot"; 

    // Simulated pull request number (this should be dynamically determined)
    let pr_number = 1; // Replace with a real PR number when testing

    // Message to post
    let message = format!("Fibonacci results: {:?}", vec![(5, 5), (8, 21)]);

    // Initialize Octocrab with authentication
    let octocrab = Octocrab::builder().personal_token(github_token).build().unwrap();

    // Post a comment to the pull request
    let response = octocrab
        .issues(owner, repo)
        .create_comment(pr_number, &message) 
        .await;

    match response {
        Ok(_) => println!("Comment posted successfully!"),
        Err(e) => eprintln!("Failed to post comment: {:?}", e),
    }
}
