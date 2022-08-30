//所有権と借用についてのメモ
fn get_ownership<T:std::fmt::Display>(arg: T) {
    println!("using {}", arg)
}

fn borrow<T:std::fmt::Display>(arg: &T) {
    println!("borrow {}", arg)
}

fn main() {
    let input = String::from("input");
    // エラー 1つ目のget_ownershipに所有権が移っている
    // get_ownership(input);
    // get_ownership(input);
    
    // 通る 所有権を引数に求める関数でも、&を付けて(値を借用して)呼び出せる
    // get_ownership(&input);
    // get_ownership(&input);
    
    // エラー 引数の型が異なる
    // borrow(input);
}
