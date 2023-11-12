use std::fs::File;
use std::io::Read;
use std::{fmt, fs, io, num};

pub fn run() {
    custom_app_error();
    from_trait();
    custom_error_handling();
    simplify_custom_error_with_thiserror();
    anyhow_crate();
}

fn anyhow_crate() {
    let result = anyhow_test();
    println!("result: {:?}", result);

    fn anyhow_test() -> anyhow::Result<()> {
        let render = render()?;
        println!("render: {}", render);
        Ok(())
    }

    fn render() -> anyhow::Result<String> {
        let file = std::env::var("MARKDOWN")?;
        let source = fs::read_to_string(file)?;
        Ok(source)
    }
}

fn simplify_custom_error_with_thiserror() {
    let result = thiserror_test();
    println!("{:?}", result);

    fn thiserror_test() -> Result<(), MyError> {
        let html = render()?;
        println!("{}", html);
        Ok(())
    }

    fn render() -> Result<String, MyError> {
        let file = std::env::var("MARKDOWN")?;
        let source = fs::read_to_string(file)?;
        Ok(source)
    }

    #[derive(thiserror::Error, Debug)]
    enum MyError {
        #[error("Environment variable not found")]
        EnvironmentVariableNotFound(#[from] std::env::VarError),
        #[error(transparent)]
        IOError(#[from] io::Error),
    }
}

fn custom_error_handling() {
    let result = custom_error_handling_test();
    println!("{:?}", result);

    fn custom_error_handling_test() -> Result<(), MyError> {
        let html = render()?;
        println!("{}", html);
        Ok(())
    }

    fn render() -> Result<String, MyError> {
        let file = std::env::var("MARKDOWN")?;
        let source = fs::read_to_string(file)?;
        Ok(source)
    }

    #[derive(Debug)]
    enum MyError {
        EnvironmentVariableNotFound,
        IOError(io::Error),
    }

    impl From<std::env::VarError> for MyError {
        fn from(_: std::env::VarError) -> Self {
            Self::EnvironmentVariableNotFound
        }
    }

    impl From<io::Error> for MyError {
        fn from(value: io::Error) -> Self {
            Self::IOError(value)
        }
    }

    // 只有为自定义错误类型实现 Error trait 后才能转换成相应的特征对象
    // impl std::error::Error for MyError {}

    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                MyError::EnvironmentVariableNotFound => write!(f, "Environment variable not found"),
                MyError::IOError(err) => write!(f, "IO Error: {}", err.to_string()),
            }
        }
    }
}

fn from_trait() {
    let result = from_trait_test();
    println!("{:?}", result);

    #[derive(Debug)]
    struct AppError {
        kind: String,
        // 错误类型
        message: String, // 错误信息
    }

    // 为 AppError 实现 std::convert::From 特征，由于 From 包含在 std::prelude 中，因此可以直接简化引入。
    // 实现 From<io::Error> 意味着我们可以将 io::Error 错误转换成自定义的 AppError 错误
    impl From<io::Error> for AppError {
        fn from(error: io::Error) -> Self {
            AppError {
                kind: String::from("io"),
                message: error.to_string(),
            }
        }
    }

    impl From<num::ParseIntError> for AppError {
        fn from(error: num::ParseIntError) -> Self {
            AppError {
                kind: String::from("parse"),
                message: error.to_string(),
            }
        }
    }

    fn from_trait_test() -> Result<(), AppError> {
        // let _file = File::open("nonexistent_file.txt")?;
        let mut file = File::open("./error/hello_world.txt")?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let _number: usize;
        _number = content.parse()?;

        Ok(())
    }
}

struct AppError {
    code: usize,
    message: String,
}

// 根据错误码显示不同的错误信息
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            404 => "Sorry, Can not find the Page!",
            _ => "Sorry, something is wrong! Please Try Again!",
        };

        write!(f, "{}", err_msg)
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AppError {{ code: {}, message: {} }}",
            self.code, self.message
        )
    }
}

fn produce_error() -> Result<(), AppError> {
    Err(AppError {
        code: 404,
        message: String::from("Page not found"),
    })
}

fn custom_app_error() {
    match produce_error() {
        Err(e) => eprintln!("{}", e), // 抱歉，未找到指定的页面!
        _ => println!("No error"),
    }

    eprintln!("{:?}", produce_error()); // Err(AppError { code: 404, message: Page not found })

    eprintln!("{:#?}", produce_error());
    // Err(
    //     AppError { code: 404, message: Page not found }
    // )
}
