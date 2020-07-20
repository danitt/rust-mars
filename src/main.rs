use std::io;

fn main() {
  println!("Please enter your earth weight in kilograms:");
  let mut earth_weight_input = String::new();
  io::stdin().read_line(&mut earth_weight_input).unwrap();
  let earth_weight: f32 = earth_weight_input.trim().parse().unwrap();
  let mars_weight: f32 = calc_mars_weight(earth_weight);
  println!("Mars weight is {}kg", mars_weight);
}

fn calc_mars_weight(weight: f32) -> f32 {
  (weight / 9.81) * 3.711
}