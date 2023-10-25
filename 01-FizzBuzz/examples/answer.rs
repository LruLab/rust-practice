// 繰り返しと分岐処理が記述できれば、FizzBuzz問題は解けます.
// Rustの場合, 上限のある繰り返しは`for`または`while`を使います.
// 分岐処理は`if`または`match`を使います.
// あくまでこの記述は一例であり, 他にも様々な解き方があります.

pub fn fizz_buzz(n: u64) {
    for i in 1..=n {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", i),
        }
    }
}

/// if分岐版
// pub fn fizz_buzz(n: u64) {
//     for i in 1..=n {
//         if i % 3 == 0 && i % 5 == 0 {
//             println!("FizzBuzz");
//         } else if i % 3 == 0 {
//             println!("Fizz");
//         } else if i % 5 == 0 {
//             println!("Buzz");
//         } else {
//             println!("{}", i);
//         }
//     }
// }

/// 文字列結合版
// pub fn fizz_buzz_3(n: u64) {
//     for i in 1..=n {
//         let mut s = String::new();
//         if i % 3 == 0 {
//             s += "Fizz";
//         }
//         if i % 5 == 0 {
//             s += "Buzz";
//         }
//         if s.is_empty() {
//             s += &i.to_string();
//         }
//         println!("{}", s);
//     }
// }

use clap::Parser;

#[derive(Parser)]
#[command(about)]
struct Args {
    number: u64,
}

fn main() {
    let args = Args::parse();
    fizz_buzz(args.number);
}
