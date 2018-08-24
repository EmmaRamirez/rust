use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("   {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn main() {
    let mut table = Table::new();

    table.insert("Gesualdo".to_string(),
                    vec!["many magridals".to_string(), "Tenebrae Resportia".to_string()]);

    table.insert("Carvaggio".to_string(),
                    vec!["The Musicians".to_string(), "Bacchus".to_string()]);
    
    table.insert("Cellini".to_string(),
                    vec!["Persuse with the head of Medusa".to_string(), "a salt cellar".to_string()]);
    
    sort_works(&mut table);
    show(&table);
}