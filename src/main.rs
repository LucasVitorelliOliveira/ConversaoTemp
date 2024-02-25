use std::io;

fn convert_to_celcius(data_input: &String) -> f64 {
    let x = data_input.trim().parse::<f64>().unwrap();
    let y = (x - 32.0) * (5.0/9.0);
    y
}

fn main() {
    let mut temp = String::new();
    println!("Digite um valor: ");
    io::stdin().read_line(&mut temp).expect("Error reading");

    let x: f64 = convert_to_celcius(&temp);   
    println!("{:.2?}", x);
}