#![allow(unused)]
fn main() {
    {
        // 何の変哲もない例
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", s);
    }

    {
        // s1の所有権がs2にmoveされるので、s1は参照できなくなる
        // よっては下記はエラーになる
        // let s1 = String::from("hello");
        // let s2 = s1;
        // println!("{}, world!", s1);
    }

    {
        // 解決方法1: cloneする。deep copyしている。
        let mut s1 = String::from("hello");
        let s2 = s1.clone();
        s1.push_str(", world");
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    {
        let s = String::from("ぺこ～");
        f1(s);

        // ここではすでに有効ではない
        // println!("s = {}", s);
    }

    {
        // 整数型、論理型、浮動小数点型、文字型（Char）、タプルがこれらだけが代表的なケースとして
        // これらはサイズが既知になるので、shallow copyとdeep copyが区別されない。
        let x = 5;
        f2(x);

        // ここでは有効
        println!("s = {}", x);
    }

    {
        /// 戻り値がs1にムーブされてくる
        let s1 = f3();
        let s2 = String::from("みこ～");

        // ここでは s2 がムーブされている
        let s3 = f4(s2);
        println!("s3 = {}", s3);
    }

    {
        let s1 = String::from("大戦争メンバー: ");

        // 複数の値を返すパターン
        let (s2, s3, len) = pekomiko(s1);
        println!("s2 = {}, s3 = {}, len = {}", s2, s3, len);
    }

    {
        let s1 = String::from("大戦争メンバー: ");

        // 複数の値を返すパターン
        let (s2, s3, len) = pekomiko(s1);
        println!("s2 = {}, s3 = {}, len = {}", s2, s3, len);
    }

    {
        let s1 = String::from("るしふぁあ");

        // 参照を渡す
        // 所有権を渡していないのでs1は無効にならない
        let len = karipeko(&s1);
        println!("{} のながさ: {}", s1, len);
    }

    {
        // 可変できる参照を渡す
        let mut s1 = String::from("ぺこ");

        // 不変な値を指す参照は可能
        let s2 = &s1;
        // 不変・可変の混合は不可能（不変側で可変であることを検知不可能であるため）
        let s3 = &mut s1;

        kahen_karipeko(&mut s1);
        println!("s1 = {}", s1);
    }

    {
        let mut s = String::from("ぺこ みこ ぺこみこ");
        // スライスで返すことによって不変参照が発生
        let word = first_word(&s);

        // なのでこれはエラーになる
        // のちに参照がない場合はここで clear してもOKだが…
        // s.clear();

        // clear() されると不変参照がおかしくなるので、clearは有効にできない
        println!("最初の語句: {}", word);
    }
}

fn f1(s: String) {
    println!("f1: s = {}", s);
}

fn f2(x: i32) {
    println!("f2: x = {}", x);
}

fn f3() -> String {
    let ss = String::from("ぺこ～");
    ss
}

fn f4(a_s: String) -> String {
    String::from(a_s + "ぺ～こぺこぺこ")
}

fn pekomiko(s: String) -> (String, String, usize) {
    let s2 = String::from(s.clone() + "ぺこ");
    let s3 = String::from(s.clone() + "みこ");
    let length = s2.len() + s3.len();
    (s2, s3, length)
}

// 参照を受け取る
// 所有権を持ってないのでsは無効にならない
fn karipeko(s: &str) -> usize {
    s.len()
}

fn kahen_karipeko(s: &mut String) {
    s.push_str("み～こ");
}

// スライスの活用
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}