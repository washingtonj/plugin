mod calculator;
mod cli;

fn main() {
    let [weight, height] = cli::ask_about_weight_and_height();
    let imc = calculator::imc(weight, height);
    cli::answer_imc(imc)
}
