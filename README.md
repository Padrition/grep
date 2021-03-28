# grep
It's a simple Rust implementation of grep instrument from [The Book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).

## Instalation 

To install you need to clone this repository 
```bash
git clone https://github.com/Padrition/grep.git
```
## Usage
* To run this programm you need to have a [rust compiler](https://www.rust-lang.org/learn/get-started)

cd into the project folder
```bach
cd grep
```
Put a file you want to search in in the root directory(or use poem.txt)

And run
```bash
cargo run [string you want to find in the file] [filename .txt]
```
to run case sensetive search.

Or run
```
CASE_SENSETIVE=1 cargo run [string you want to find in the file] [filename .txt]
```
to run case insensetive search.

## Use example
```
CASE_SENSETIVE=1 cargo run to poem.txt
```
Output
```
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```
