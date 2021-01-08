use std::env;
use std::error::Error;
use std::fs;

//#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        //&'static str 是字符串字面值的类型
        if args.len() < 3 {
            //增加一个参数数量检查
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        //env::var 返回一个 Result，它在环境变量被设置时返回包含其值的 Ok 成员，并在环境变量未被设置时返回 Err 成员。
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        //如果CASE_INSENSITIVE 环境变量被设置为任何值，is_err 会返回 false 并将进行大小写不敏感搜索。
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); //将 query 字符串转换为小写，并将其覆盖到同名的变量中
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            //query 现在是一个 String 而不是字符串 slice,将 query 作为一个参数传递给 contains 方法时，需要增加一个 & 因为 contains 的签名被定义为获取一个字符串 slice。
            results.push(line);
        }
    }
    results
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //表明返回的 vector 中应该包含引用参数 contents（而不是参数query） slice 的字符串 slice

    let mut results = Vec::new();
    for line in contents.lines() {
        //lines 方法返回一个迭代器
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

//Ok时返回unit类型()；对于错误类型，使用了 trait 对象 Box<dyn Error>，意味着函数会返回实现了 Error trait 的类型，不过无需指定具体将会返回的值的类型。
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?; //读取对应路径的文件的内容，? 会从函数中返回错误值并让调用者来处理它。
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    //println!("With text:\n{}", contents);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
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
