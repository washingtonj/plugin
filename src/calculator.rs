pub fn imc(weight: i32, height: i32) -> f32 {
  let correct_weight = (weight * 1000_0) as f32;
  let correct_height = (height * height) as f32;
  return correct_weight / correct_height;
}