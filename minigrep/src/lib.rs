use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        //从环境变量中获取信息
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
    //println!("target str:{}", config.query);
    //println!("get contents:\n{contents}");

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    println!("===========================");
    for line in results {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        //这样书写,保证每行开头没有空格或其他看不见的符号
        //或者手动加入换行符:Rust:\nsafe, fast, productive.\nPick three.";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        //这样书写,保证每行开头没有空格或其他看不见的符号
        //或者手动加入换行符:Rust:\nsafe, fast, productive.\nPick three.";
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        //遍历contents的每一行
        if line.contains(query) {
            //用查询字符串搜索该行
            results.push(line); //存储匹配的行
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //to_lowercase是创建新的数据,而不是引用现有数据!
    //而且得到的是 String 而不是 &str
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        //遍历contents的每一行
        //并创建新的数据
        if line.to_lowercase().contains(&query) {
            //用查询字符串搜索该行
            results.push(line); //存储匹配的行
        }
    }

    results
}
