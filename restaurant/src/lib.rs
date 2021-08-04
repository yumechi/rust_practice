mod front_of_house {
    pub mod hosting {
        // このレベルまで公開として書かないと見えない
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_paymant() {}
    }
}

/*
pub fn eat_at_restaurant() {
    // 絶対パス
    // モジュールの移動とか考えると絶対パスのほうがよさそう
    crate::front_of_house::hosting::add_to_waitlist();

    // 相対パス
    front_of_house::hosting::add_to_waitlist();
}
*/

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        // back_of_houseの親はcrate
        // 今回は直前にあるメソッドが呼ばれる想定
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    // 構造体はフィールドの公開、非公開を制御可能
    pub struct Breakfast {
        // 公開フィールド
        pub toast: String,
        // 非公開フィールド
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // toastは公開されてるので書き換え可能
    meal.toast = String::from("Wheat");
    // seasonal_fruit は公開されていないので不可能
    // meal.seasonal_fruit = String::from("Carrots");
    println!("これをお願いします: {} トースト", meal.toast);

    // Enumは公開にすると全部アクセスできる
    let mut order1 = back_of_house::Appetizer::Soup;
    let mut order2 = back_of_house::Appetizer::Salad;
}

// useすると記載が簡潔になる
// 慣習的にこのレベルまでの読み込みにするとよい
// 例えば2つのパッケージ両方でResultがあって、それを利用先で区別する際に役立つ
// 絶対パス
pub use crate::front_of_house::hosting;
// 相対パス
// use self::front_of_house::hosting;

pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

/*
// ここだけとるのもできる
// ここまでやってしまうとどこで定義されたものかわかりづらくなるので、やらないほうが良い
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant3() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
*/

// as を使って同じ型名のものを区別する
use std::fmt::Result;
use std::io::Result as IoResult;

// importをまとめて書く
use std::{cmp::Ordering, io};

// 片方がサブパスである場合
// use std::io;
// use std::io::Write;
// これをまとめると、こう書ける
// use std::io::{self, Write};

// すべての公開要素をスコープに持ち込みたいとき
// testsの場合などに使えそう
use std::collections::*;


