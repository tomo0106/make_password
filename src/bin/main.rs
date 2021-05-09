use clap::Clap;
// Like the standard library's prelude, this module simplifies importing of common items. Unlike the standard prelude, the contents of this module must be imported manually:
// 標準ライブラリのプレリュードと同様に、このモジュールは一般的なアイテムのインポートを簡素化します。 標準のプレリュードとは異なり、このモジュールの内容は手動でインポートする必要があります。
use rand::distributions::{Distribution, Uniform};
// use rand::prelude::*;
use anyhow::{bail, ensure, Context, Result};
// write password
use std::fs::File;
use std::io::Write;
#[derive(Clap, Debug)]
#[clap(
    name = "Password maker",
    version = "0.0.1",
    author = "Tomo",
    about = "Make password"
)]
struct Opts {
    /// Sets the random number of digits
    #[clap(short, long)]
    digits: Option<i32>,

    /// Whether to include numbers.
    #[clap(short, long)]
    numbers: bool,

    /// Whether to include lowercase letters.
    #[clap(short, long)]
    lowercase: bool,

    /// Whether to include uppercase letters.
    #[clap(short, long)]
    uppercase: bool,

    /// Whether to include symbols.
    #[clap(short, long)]
    symbol: bool,

    /// Number of passwords to generate
    #[clap(short, long)]
    generated_number: Option<i32>,

    /// Save file path(.csv)
    #[clap(short, long)]
    path: Option<String>,
}

/// パスワード生成
fn gen_password(password_base: &str, password_digits: i32) -> Result<String> {
    if password_base.is_empty() {
        bail!("There is no character string to use in the password")
    }
    let range = Uniform::new(0, password_base.len());
    let mut rng = rand::thread_rng();

    let mut password = String::new();
    for _ in 0..password_digits {
        let rand_index = range.sample(&mut rng);
        let value = &password_base[rand_index..rand_index + 1];
        // println!("{} {}",rand_index,value);
        password.push_str(&value);
    }
    Ok(password)
    // password
}

fn main() -> Result<()> {
    const PASSWORD_BASE_NUMERIC: &str = "1234567890";
    const PASSWORD_BASE_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const PASSWORD_BASE_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const PASSWORD_BASE_SYMBOL: &str = "@%+\\/$%!#$^?':.(){}[]~-_";

    let opts: Opts = Opts::parse();

    let mut password_base = String::new();
    if opts.numbers {
        println!("Include numbers in password...");
        password_base.push_str(&PASSWORD_BASE_NUMERIC);
    }
    if opts.lowercase {
        println!("Include lowercase letters in password...");
        password_base.push_str(&PASSWORD_BASE_LOWERCASE);
    }
    if opts.uppercase {
        println!("Include uppercase letters in password...");
        password_base.push_str(&PASSWORD_BASE_UPPERCASE);
    }
    if opts.symbol {
        println!("Include symbols in password...");
        password_base.push_str(&PASSWORD_BASE_SYMBOL);
    }

    let mut password_digits = 8;
    match opts.digits {
        Some(t) => {
            println!("the random number of digits : {}", t);
            password_digits = t;
        }
        None => {
            println!("default random number of digits : {}", password_digits);
        }
    };

    let mut generated_password_number = 10;
    match opts.generated_number {
        Some(t) => {
            println!("Number of passwords to generate : {}", t);
            generated_password_number = t;
        }
        None => {
            println!(
                "default number of passwords to generate : {}",
                generated_password_number
            );
        }
    };

    if let Some(path) = opts.path {
        let mut file = File::create(path)?;
        for i in 0..generated_password_number {
            let password = gen_password(&password_base, password_digits)?;
            println!("{} password : {}", i + 1, password);
            writeln!(file, "{},{}", i + 1, password)?;
        }
    } else {
        for i in 0..generated_password_number {
            let password = gen_password(&password_base, password_digits)?;
            println!("{} password : {}", i + 1, password);
        }
    }

    Ok(())
}
