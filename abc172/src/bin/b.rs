use proconio::input;

fn main() {
    input! {
        S: String,
        T: String,
    }
    println!("{}", S.chars().zip(T.chars()).filter(|(s, t)| s != t).count());
}
