//! 生成命令产生的文本：密码、十六进制串和用户名。
//!
//! 生成结果替换每个选区（没有选区时插入到光标处），多个选区使用同一份文本。

use rand::seq::SliceRandom;
use rand::Rng;

/// 密码的长度。
const PASSWORD_LENGTH: usize = 16;
/// 十六进制串的字符数，等于 `openssl rand -hex 16` 的输出长度。
const HEX_LENGTH: usize = 32;
const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
/// 安全密码在字母和数字之外追加的符号。
const SYMBOLS: &str = "!@#$%^&*_-+=|/?:;<>~";
const HEX_DIGITS: &str = "0123456789abcdef";

/// 常见的英语名，用户名取其中的名和姓。
const FIRST_NAMES: &[&str] = &[
    "John", "Sarah", "Mark", "Mary", "Mike", "Jessica", "David", "Laura", "Robert", "Ashley",
    "James", "Michelle", "Michael", "Elizabeth", "William", "Jennifer", "Richard", "Rebecca",
    "Joseph", "Amanda", "Thomas", "Barbara", "Charles", "Melissa", "Christopher", "Deborah",
    "Daniel", "Stephanie", "Matthew", "Kimberly", "Anthony", "Amy", "Donald", "Sharon", "Steven",
    "Cynthia", "Paul", "Ruth", "Andrew", "Brenda", "Joshua", "Pamela", "Kenneth", "Nicole",
    "Kevin", "Helen", "Brian", "Sandra",
];

/// 常见的英语姓氏。
const LAST_NAMES: &[&str] = &[
    "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis", "Rodriguez",
    "Martinez", "Hernandez", "Lopez", "Gonzalez", "Wilson", "Anderson", "Thomas", "Taylor",
    "Moore", "Jackson", "Martin", "Lee", "Perez", "Thompson", "White", "Harris", "Sanchez",
    "Clark", "Ramirez", "Lewis", "Robinson", "Walker", "Young", "Allen", "King", "Wright",
    "Scott", "Torres", "Nguyen", "Hill", "Flores", "Green", "Adams", "Nelson", "Baker", "Hall",
    "Rivera", "Campbell", "Mitchell", "Carter", "Roberts",
];

/// 生成 16 位的密码，字符取自字母和数字。
pub(in crate::app) fn password() -> String {
    sample_chars(&format!("{LETTERS}{DIGITS}"), PASSWORD_LENGTH)
}

/// 生成 16 位的安全密码，在字母和数字之外再加符号。
pub(in crate::app) fn secure_password() -> String {
    sample_chars(&format!("{LETTERS}{DIGITS}{SYMBOLS}"), PASSWORD_LENGTH)
}

/// 生成 32 个十六进制字符。
pub(in crate::app) fn hex() -> String {
    let bytes = HEX_DIGITS.as_bytes();
    let mut rng = rand::thread_rng();
    (0..HEX_LENGTH)
        .map(|_| bytes[rng.gen_range(0..bytes.len())] as char)
        .collect()
}

/// 生成用户名：名的首字母、姓，再加三位数字，如 `MJohnson327`。
pub(in crate::app) fn username() -> String {
    let mut rng = rand::thread_rng();
    let first = FIRST_NAMES[rng.gen_range(0..FIRST_NAMES.len())];
    let last = LAST_NAMES[rng.gen_range(0..LAST_NAMES.len())];
    format!("{}{}{}", &first[..1], last, rng.gen_range(100..1000))
}

/// 从 `population` 中无放回地抽出 `n` 个字符，同 Python 的 `random.sample`。
fn sample_chars(population: &str, n: usize) -> String {
    let chars: Vec<char> = population.chars().collect();
    let mut rng = rand::thread_rng();
    chars.choose_multiple(&mut rng, n).into_iter().collect()
}
