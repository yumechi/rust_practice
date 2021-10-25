fn main() {
    vec_data();
    string_data();
}

fn vec_data() -> () {
    {
        // Genericsで型を指定する必要がある
        let v: Vec<i32> = Vec::new();
        // 明らかならば型をヒントする必要はない
        let v = vec![1, 2, 3];

        // 可変にするにはやっぱり mut が必要
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);

        // vはここで解放される
    }

    {
        let v = vec![1, 2, 3, 4, 5];

        // 添え字記法でも、getでもOK。これらは参照を得ている。
        let third: &i32 = &v[2];
        let third: Option<&i32> = v.get(2);

        // 存在しないところにアクセスした場合、添え字記法の場合はpanic, get記法の場合はNoneになる
        // let does_not_exist: &i32 = &v[5];
        // let does_not_exist: Option<&i32> = v.get(8);
    }

    {
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        // 上で参照されているので、不変扱いにある。よって次のpushが失敗する。
        // v.push(6);
        println!("最初の要素: {}", first);
    }

    {
        let mut v = vec![123, 456, 789, 1230];
        for i in &mut v {
            // 可変参照の値を変更するためには、参照外しをしないといけない
            *i += 50;
        }
        for i in &mut v {
            println!("{}!", i);
        }
    }

    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        // enumを使って複数の型を保持することができる
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("purple")),
            SpreadsheetCell::Float(10.12),
        ];
    }
}

fn string_data() -> () {
    {
        let mut s = String::new();

        // 初期化
        // Displayトレイトを実装する型なら to_string が使える
        let data = "initial contents";
        let s = data.to_string();
        let s = "initial contents".to_string();

        let s = String::from("initial contents");

        // UTF-8なので、これも問題なくできる
        let hello = String::from("こんぺこ～");
    }

    {
        // 文字列を更新する
        let mut s = String::from("うるは");
        s.push_str("るしぁ");

        let mut s1 = String::from("うるは");
        let s2 = "るしぁ";
        s1.push_str(s2);
        println!("s1 is {}", s1);
        println!("s2 is {}", s2);
    }

    {
        let s1 = String::from("うぺー");
        let s2 = String::from("るーぺー");
        // s1はムーブされ、もう使用できない
        let s3 = s1 + &s2;
        println!("s3 is {}", s3);
    }
}
