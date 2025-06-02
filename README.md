# A  Fibbot Project To Calculate Fibonacci
- Fibbot is a GitHub action that scans pull request and calculates their Fibonacci.

- This fibbot Project is designed to automate the calculation of numbers for numbers found in pull request conten.

**usage** : 
- to use fibbot you need to add the it to following GitHub action workflow file 
e.g ( .github/workflows/tes.yml )

**configuration** :
- This Fibbot Project is configured to use o2 in inputs :
1. **enable_fib** : To enable or disable fibonacci calculation (default: false)
2. **Max_threshold** : To limit for fibonacci calculation (default: 100)

**How It Works**
- This fibbot project uses the following steps to calculate fibonacci of numbers:
1. Extract numbers from PR content using the **extract_numbers** function
2. Calculate fibonacci numbers for each extracted number using the **fibonacci** function 
3. Post the result as a comment on the PR

**Development Tools** : To develop and run  fibbot you need to install the follwing tools and their Dependencies: 

1. Rust
2. Cargo
3.  GitHub CLI

. To run Fibbot use this commands: 
1.  cargo build
2. cargo run

.  A dockerfile also was included also to run this Project in a Dockerised environment
. To run it using docker you use the following commands:
1. docker build -t fibbot . : to build the image
2. docker run -it fibbot : to run the container

