#![allow(unused)]
fn main() {
    println!("03-01: 変数");
    let mut x = 5;
    println!("xの値: {}", x);
    x = 6;
    println!("かきかえたxの値: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    println!("スコープとShadowing");
    {
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        println!("再定義に前のxの値が使われるので12になる: {}", x);
    }

    println!("03-02: データ型");
    {
        // 型ばらばらのtupleが作れる、取り出しは一気にできる
        // なお、x, zは使わないので unusedのマクロか、 _x, _zのように書く必要がある
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup;
        println!("Tupleから取り出したyの値: {}", y);
    }

    {
        // tupleは .0 のような形式で取り出す
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let gohyaku = tup.0;
        let roku_ten_four = tup.1;
        let ichiban = tup.2;
        println!(
            "Tupleから取り出した値: {}, {}, {}",
            gohyaku, roku_ten_four, ichiban
        );
    }

    {
        // Arrayの定義
        let a = [1, 2, 3, 4, 5];
        let index = 2;
        // index = 10 だとエラーになる
        let element = a[index];
        println!("Arrayから取り出した値: {}", element);
    }

    {
        // 配列を全て同じ値で定義可能
        let a = [334; 5];
        for i in a.iter() {
            println!("初期値埋めた配列の中身！: {}", i);
        }
    }

    println!("03-03: 関数(function)");
    func_1();
    func_2(5, 6);

    {
        // ブロックの一番最後の結果が反映される
        let y = {
            let x = 3;
            x + 1
        };
        println!("ブロックyの評価結果: {}", y);
    }

    {
        // 戻り値ありのメソッド
        let x = get_five();
        println!("5を取得した！: {}", x);
    }

    {
        // 戻り値ありのメソッド
        let x = get_plus_one(5);
        println!("1足した値を得る: {}", x);
    }

    println!("03-05: if statement");
    {
        // trueになるケース
        let number = 0.16;
        if number < 5.0 {
            println!("5.0未満です: {}", number);
        } else {
            println!("5.0以上です: {}", number);
        }
        // if文の条件式はboolである必要があるので、下記のようには書けない
        // if number {
        //     println!("数値です");
        // }
    }

    {
        // falseになるケース
        let number = 446;
        if number < 5 {
            println!("5.0未満です: {}", number);
        } else {
            println!("5.0以上です: {}", number);
        }
    }

    {
        let number = 294;
        // else-if を使ってみるやつ
        if number % 4 == 0 {
            println!("4で割り切れる: {}", number);
        } else if number % 3 == 0 {
            println!("3で割り切れる: {}", number);
        } else if number % 2 == 0 {
            println!("2で割り切れる: {}", number);
        } else {
            println!("2,3,4では少なくとも割り切れない: {}", number);
        }
    }

    {
        let cond = true;
        let number = if cond { 567 } else { 563 };
        // 異なる型が返るようには書けない
        // let number = if condition {
        //     567
        // } else {
        //     "korone"
        // };
        println!("numberの値: {}", number);
    }

    // 無限ループ
    // loop {
    //     println!("ドドンドドンドンドン！");
    // }

    // whileを使ったループ
    {
        let mut counter = 3;
        while counter != 0 {
            println!("カウントダウン！: {}", counter);
            counter = counter - 1;
        }
        println!("しゅっこ～!!!");
    }

    {
        // loopのbreak結果を受け取る
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("resultの値: {}", result);
    }

    {
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;

        while index < 5 {
            println!("whileでindexアクセス: {}", a[index]);
            index = index + 1;
        }

        for elem in a.iter() {
            println!("iterでアクセス: {}", elem);
        }
    }

    {
        for counter in (1..4).rev() {
            println!("カウントダウン: {}", counter);
        }
        println!("おつかれさまでした～!!!");
    }
}

fn func_1() {
    println!("Another function.");
}

fn func_2(x: i32, y: i32) {
    println!("{}, {}", x, y);
}

fn get_five() -> i32 {
    5
}

fn get_plus_one(x: i32) -> i32 {
    x + 1
}
