use std::{env, error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // 跳过第一个参数
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|&line| line.contains(query))
        .collect()
    // let mut res = Vec::new();
    // // 遍历迭代 contents 的每一行
    // for line in contents.lines() {
    //     // 检查该行内容是否包含我们的目标字符串
    //     // 若包含，则放入返回值列表中，否则忽略
    //     if line.contains(query) {
    //         res.push(line);
    //     }
    // }
    // // 返回匹配到的返回值列表
    // res
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();
    contents
        .lines()
        .filter(|&line| line.to_lowercase().contains(query))
        .collect()
    // let mut res = Vec::new();
    // // 遍历迭代 contents 的每一行
    // for line in contents.lines() {
    //     // 检查该行内容是否包含我们的目标字符串
    //     // 若包含，则放入返回值列表中，否则忽略
    //     if line.to_lowercase().contains(query) {
    //         res.push(line);
    //     }
    // }
    // // 返回匹配到的返回值列表
    // res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
