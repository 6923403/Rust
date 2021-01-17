use std::collections::HashMap;

fn main()
{
    te5();
}

fn te5()
{
    let text = "A C N Q";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace()
    {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn te4()
{
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn te3()
{
    /*
     * map owner
     */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    //let mut map = HashMap::new();
    //map.insert(field_name, fiele_value);
}

fn te2()
{
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    /*
     * use '_' auto analysis type
     */
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}

fn te1()
{
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores
    {
        println!("{}: {}", key, value);
    }
}
