fn beer_song(mut reps: u8) {
    while reps > 0 {
        let nxt = reps - 1;
        println!(
            "{} bottles of beer on the wall, {} bottles of beer.
        Take one down and pass it around, {} bottles of beer on the wall.",
            reps, reps, nxt
        );

        reps -= 1;
    }
}

pub fn main() {
    beer_song(10);
}
