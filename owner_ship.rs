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
    
    // 通る get_ownershipのTが&Stringだと解釈される
    // get_ownership(&input);
    // get_ownership(&input);
    
    // エラー 引数の型が異なる
    // borrow(input);
}
