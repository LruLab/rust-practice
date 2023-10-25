//! # FizzBuzz
//!
//! 1から指定された数値までのFizzBuzzを出力してください
//!
//! ## FizzBuzzのルール
//!
//! - 3で割り切れる数値の場合は、"Fizz"という文字列を出力します.
//! - 5で割り切れる数値の場合は、"Buzz"という文字列を出力します.
//! - 3と5の両方で割り切れる数値の場合は、"FizzBuzz"という文字列を出力します.
//! - それ以外の数値の場合は、数値を文字列としてそのまま出力します.

pub fn fizz_buzz(n: u64) {
    /* WRITE YOUR CODE HERE */
}





///// 以下は実行処理本体. /////

use clap::Parser;

#[derive(Parser)]
#[command(about)]
struct Args {
    number: u64,
}

fn main() {
    // コマンドライン引数の解析
    let args = Args::parse();
    fizz_buzz(args.number);
}
