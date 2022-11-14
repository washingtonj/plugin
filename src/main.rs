mod calculator {
    pub fn imc(weight: i32, height: i32) -> f32 {
        let correct_weight = (weight * 1000_0) as f32;
        let correct_height = (height * height) as f32;
        return correct_weight / correct_height;
    }
}

mod cli {
    use std::io::stdin;

    pub fn ask_about_weight_and_height() -> [i32; 2] {
        let mut height = String::new();
        let mut weight = String::new();

        println!("What is your Height");
        stdin()
            .read_line(&mut height)
            .expect("Error on reading height line");

        println!("What is your Weight");
        stdin()
            .read_line(&mut weight)
            .expect("Error on reading weight line");

        return [
            weight.trim().parse().unwrap(),
            height.trim().parse().unwrap(),
        ];
    }

    pub fn answer_imc(imc: f32) {
        println!("Your imc is {}", &format!("{:.1}", imc))
    }

}

fn main() {
    let [weight, height] = cli::ask_about_weight_and_height();
    let imc = calculator::imc(weight, height);
    cli::answer_imc(imc)
}
