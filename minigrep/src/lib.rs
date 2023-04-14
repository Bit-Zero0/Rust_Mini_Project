use std::env;
use std::error::Error;
use std::fs;
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("Not Enoght arguments");
        // }
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didm't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok(); //is_ok 值得说道下。该方法是 Result 提供的，用于检查是否有值，有就返回 true，没有则返回 false，刚好完美符合我们的使用场景，因为我们并不关心 Ok<T> 中具体的值。

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;

        let results = if config.ignore_case {
            search_case_insensitive(&config.query, &contents)
        } else {
            search(&config.query, &contents)
        };

        for line in results {
            println!("{line}");
        }

        Ok(())
    }
}

//忽略大小写的搜索
// pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let query = query.to_lowercase();
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.to_lowercase().contains(&query) {
//             results.push(line);
//         }
//     }

//     results
// }

//迭代器写法
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

//不忽略大小写 (默认操作)
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.contains(query) {
//             results.push(line);
//         }
//     }

//     results
// }

//迭代器版 search
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
