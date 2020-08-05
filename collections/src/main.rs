use std::io;
use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    vectors();
    strings();
    hash_maps();
    exercises();
}

fn vectors() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn strings() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

fn hash_maps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(sc) => println!("Score: {} ", sc),
        None => println!("Score not found")
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //Overwriting a value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    //Only insert if the key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn exercises() {
    //1.
    first_exercise();
    //2.
    second_exercise();
    //3.
    third_exercise();
    
}

fn first_exercise() {
    let mut v = vec![1, 2, 3, 2, 4];
    let mut sum = 0;
    for i in &v {
        sum += i;
    }
    let mean = sum / v.len();
    println!("The mean is: {}", mean);
    v.sort();
    match v.get(v.len() / 2) {
        Some(median) => println!("The median is {}", median),
        None => println!("Could not find the median."),
    }

    let mut elements = HashMap::new();
    let mut mode = &v[0];
    let mut max = 0;

    for number in &v {
        let count = elements.entry(number).or_insert(0);
        *count += 1;
        if *count > max {
            max = *count;
            mode = number;
        }
    }
    println!("The mode is: {}", mode);
}

fn second_exercise() {
    let word1 = String::from("first");
    get_pig_latin(word1);
    let word2 = String::from("apple");
    get_pig_latin(word2);
    
}

fn get_pig_latin(word: String) {
    let (head, tail) = word.split_at(1);
    let result = match head {
        "a" | "e" | "i" | "o" | "u" => format!("{}-hay", &word),
        _ => format!("{}-{}ay", tail, head)
    };
    println!("The pig latin of '{}' is '{}'", word, result);
}

fn third_exercise() {
    let mut company:HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut text_input = String::new();

        println!("Please input your the employee to add. Type 'exit' to finish.");
        io::stdin()
            .read_line(&mut text_input)
            .expect("Failed to read line");
        println!("input: {}", text_input);
        if text_input.trim() == "exit" {
            println!("finishing loop");
            break;
        }
        let text_split: Vec<&str> = text_input.split_whitespace().collect();
        if text_split.len() == 4 {
            let name = match text_split.get(1) {
                Some(word) => word,
                None => ""
            };
            let department = match text_split.get(3) {
                Some(word) => word,
                None => ""
            };

            let names = company.entry(department.to_string()).or_insert(Vec::new());
            names.push(name.to_string());
            names.sort();
        } else {
            continue;
        }
    }
    loop {
        let mut text_input = String::new();
        println!("\n Please input the department you want to retrieve. \n Type 'all' if you want all people in the company. \n Type 'exit' to finish");
        io::stdin()
            .read_line(&mut text_input)
            .expect("Failed to read line");
        if text_input.trim() == "exit" {
            break;
        } else if text_input.trim() == "all" {
            println!("All employess from company: {:?} \n ", company);
        } else {
            match company.get(text_input.trim()) {
                Some(names) => println!("Employees: {:?} \n ", names),
                None => println!("Unknown department!")
            }
        }
    }
    
}