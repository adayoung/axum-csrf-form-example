1. cargo run
2. Open http://localhost:2024/
3. Submit form on the browser -> note token is valid
4. Open http://localhost:2024/ again (in a new tab or from the address bar)
5. Stop cargo run with ctrl-c and start it again
6. Submit form in the still open page from step 4 -> note token is not valid

We were missing salt! Adding salt fixed everything. Yay salt!
