#![allow(unused_mut)]
#![allow(unused_variables)]

use std::{collections::HashMap, io};

fn main() {
    vector_stuff();
    string_stuff();
    hashmap_stuff();

    let mut vec = vec![1, 3, 9, 5, 2, 4, 8, 7, 6, 3];
    println!("Median is {}", median(&mut vec));
    println!("Mode is {}", mode(&vec));

    add_employees();
    pig_latin();
}

fn is_vowel(c: &char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn pig_latin() {
    loop {
        println!("What word to pig latinify (type q to quit)?");

        let mut word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");

        word = String::from(word.trim());
        if word == "q" {
            break;
        }

        let first_char: char = match word.chars().next() {
            Some(c) => c,
            None => continue
        };

        if is_vowel(&first_char)  {
            println!("{}-hay", word);
        } else {
            let first_consonant = word.find(|c| is_vowel(&c)).unwrap_or(0);
            let (consonant, body) = word.split_at(first_consonant);
            println!("{}-{}ay", body, consonant);
        }
    }
}

fn add_employees() {
    let mut dep_list: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Add who to what department (type q to quit)?");

        let mut addition = String::new();
        io::stdin()
            .read_line(&mut addition)
            .expect("Failed to read line");

        if addition.trim() == "q" {
            break;
        }

        if !addition.contains("Add ") || !addition.contains(" to ") {
            println!(
                "Expected addition in the form 'Add * to *', case sensitive, please try again"
            );
            continue;
        }

        let parts: Vec<&str> = addition.split(' ').collect();
        dep_list
            .entry(String::from(parts[3].trim()))
            .or_insert_with(Vec::new)
            .push(String::from(parts[1].trim()));
    }
    println!("Employees are {dep_list:?}");
}

fn median(mut list: &mut Vec<i32>) -> i32 {
    list.sort();
    list[list.len() / 2]
}

fn mode(list: &Vec<i32>) -> i32 {
    let mut occur = HashMap::new();
    for ele in list {
        let count = occur.entry(ele).or_insert(0);
        *count += 1
    }

    let mut mode = 0;
    let mut count = 0;
    for (key, value) in &occur {
        if *value > count {
            mode = **key;
            count = *value;
        }
    }
    mode
}

fn hashmap_stuff() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Insert if doesn't exist
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");

    scores.entry(String::from("Red")).or_insert(30);
    println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}

fn string_stuff() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let mut s = String::from(data);

    s.push_str(" bar");
    s.push('!');

    let s1 = s + " And " + &String::from(data);
    // s has been moved
    println!("{s1}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let s1 = String::from("hi");
    // CANNOT do this index, as strings are UTF-8 encoded Vec<u8>
    // let h = s1[0];

    // Use bytes or chars
    for c in "Зд".chars() {
        println!("{c}");
    }
}

fn vector_stuff() {
    let vn: Vec<i32> = Vec::new();
    let vm = vec![1, 2, 3];

    let mut vn = Vec::new();
    vn.push(1);
    vn.push(2);

    let third = &vm[2];
    println!("The third element is {third}");

    let third: Option<&i32> = vn.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &mut vn {
        *i += 50;
        println!("{i}");
    }
}
