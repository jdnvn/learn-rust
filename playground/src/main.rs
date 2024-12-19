fn main() {
    struct User {
        id: u32,
        username: String,
        role: String
    }

    impl std::fmt::Display for User {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "<User id={} username={} role={}>", self.id, self.username, self.role)
        }
    }

    impl User {
        fn send_message(&self, message: String) {
            println!("{message}");
        }

        fn admin(username: String) -> User {
            User {
                username,
                id: 2,
                role: String::from("admin")
            }
        }
    }

    impl PartialEq for User {
        fn eq(&self, other: &Self) -> bool {
            self.username == other.username
        }
    }

    let user = User {
        username: String::from("joe"),
        role: String::from("peasant"),
        id: 1
    };

    println!("{}", user);
    user.send_message(String::from("sup nerd"));
    let admin_user = User::admin(String::from("joe"));
    println!("{}", admin_user);
    println!("user == admin_user is {}", user == admin_user);

    let value = Some("this is a string");
    let mut none: Option<&str> = None;
    println!("{:?}", value);
    println!("{:?}", none);

    none = Some("now it has a value");
 
    println!("{:?}", none);
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.pop();
    v.push(3);
    println!("{:?}", v[0]);
    println!("{v:?}");

    let mut literal_vec = vec![1, 2, 3];
    println!("{literal_vec:?}");

    literal_vec.pop();
    println!("{literal_vec:?}");

    let string_vec = vec![String::from("hello"), String::from("sup")];
    let first_elem = &string_vec[0];

    println!("{first_elem}");

    let result: Option<&String> = string_vec.get(69);
    match result {
        Some(value) => println!("{value}"),
        None => println!("That value does not exist in the vector brother")
    }

    for i in &string_vec {
        println!("{i}");
    }


    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, field_value);

    println!("{map:?}");

    let mut numbers = vec![5, 6, 7, 2, 4, 2, 1, 5, 6, 7, 7];
    println!("Here are the numbers: {numbers:?}");
    numbers.sort();
    let median = numbers[numbers.len()/2];
    println!("The median is: {median}");

    let mut map = HashMap::new();
    for i in &numbers {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mode = map.iter().max_by_key(|&(_, v)| *v).map(|(k, _)| k).unwrap();
    println!("The mode is: {mode}");

    let largest_num: &i32 = largest(&numbers);
    println!("The largest number in the list is: {largest_num}");

    let word = "Hi I am wicked chill";
    let char_slice = &word.chars().collect::<Vec<char>>();
    let largest_char = largest(&char_slice);
    println!("The largest char is: {largest_char}");
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for element in list {
        if element > largest {
            largest = element;
        }
    }

    largest
}
