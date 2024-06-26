fn reverse_string(data: [&str; 1]) -> Vec<String> {
    let mut agroup_results: Vec<String> = vec![];
    for word in data {
        let chars: Vec<&str> = word.split("").collect::<Vec<&str>>();
        let mut result: Vec<&str> = vec![];
        let mut positions = vec![];
        for n in 0..chars.len() {
            positions.push(n);
        }

        for position in 0..positions.len() {
            let nxt = position + 1;
            for nxt_position in nxt..positions.len() {
                if position < nxt_position {
                    let tmp = positions[position];
                    positions[position] = positions[nxt_position];
                    positions[nxt_position] = tmp;
                }
            }
        }

        for index in positions {
            result.push(chars[index]);
        }

        agroup_results.push(result.concat());
    }

    return agroup_results;
}

pub fn main() {
    println!("{:?}", reverse_string([&"stressed"]));
}
