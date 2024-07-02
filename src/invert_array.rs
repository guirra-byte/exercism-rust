pub fn main() {
  let mut numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let edge = (0, numbers.len() - 1);
  for index in 0..=(edge.1 / 2) {
      let tmp = numbers[index];
      let target = if index == 0 { edge.1 } else { edge.1 - index };

      numbers[index] = numbers[target];
      numbers[target] = tmp;
  }

  println!("{:?}", numbers);
}