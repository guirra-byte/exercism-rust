fn build_acronym(phrase: &str) -> String {
    let chunks = phrase.split(" ");
    let mut acronym: Vec<&str> = vec![];
    for chunk in chunks {
        let word = chunk.split("").collect::<Vec<&str>>();
        for letter in word {
            if letter != "" {
                acronym.push(letter);
                break;
            }
        }
    }

    return acronym.concat();
}

pub fn main() {
    println!("{}", build_acronym("As Soon As Possible"));
}
