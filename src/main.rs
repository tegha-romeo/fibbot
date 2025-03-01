use std::env;
use regex::Regex;
use octocrab::Octocrab;


// fn extract_numbers(text: &str) -> Vec<u32> {
//     let re = Regex::new(r"\b\d+\b").unwrap();
//     re.find_iter(text)
//     .filter_map(|digits| digits.as_str().parse::<u32>().ok())
//     .collect()
// }
  



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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch the GitHub token from environment variables
    // let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let github_token = env::var("GITHUB_TOKEN").expect("Missing GITHUB_TOKEN");

    // Fetch repository owner and name dynamically (replace as needed)
    let owner = "tegha-romeo"; 
    let repo_name = "fibbot";  // Ensure this matches the repository name

    // Simulated pull request number (this should be dynamically determined)
    let pr_number = 1; // Replace with a real PR number when testing

    // Message to post
    let message = format!("Fibonacci results: {:?}", vec![(5, 5), (8, 21)]);

    // Initialize Octocrab with authentication
    let octocrab = Octocrab::builder()
        .personal_token(github_token)
        .build()?;

    // Check if the repository exists
    match octocrab.repos(owner, repo_name).get().await {
        Ok(repo_info) => println!("Repository found: {:?}", repo_info),
        Err(e) => return Err(format!("Failed to find repository: {:?}", e).into()),
    }

    // Check if the pull request exists
    match octocrab.pulls(owner, repo_name).get(pr_number).await {
        Ok(pr_info) => println!("Pull request found: {:?}", pr_info),
        Err(e) => return Err(format!("Failed to find pull request: {:?}", e).into()),
    }

    // Post a comment to the pull request
    let response = octocrab
        .issues(owner, repo_name)
        .create_comment(pr_number, &message)
        .await;

    // Explicitly handle the response
    match response {
        Ok(comment) => println!("Comment posted successfully: {:?}", comment),
        Err(e) => println!("Failed to post comment: {:?}", e),
    }

    Ok(())
}