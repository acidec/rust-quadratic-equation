fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Please input a");
    std::io::stdin().read_line(&mut a).expect("Failed to read line");
    let a: f64 = a.trim().parse().expect("Please type a number!");
    println!("Please input b");
    std::io::stdin().read_line(&mut b).expect("Failed to read line");
    let b: f64 = b.trim().parse().expect("Please type a number!");
    println!("Please input c");
    std::io::stdin().read_line(&mut c).expect("Failed to read line");
    let c: f64 = c.trim().parse().expect("Please type a number!");
    nya(a, b, c);
}
fn nya(a: f64, b: f64, c: f64) {
    let delta = (b * b) - (4.0 * a * c);
    if delta > 0.0 {
        let x1 = ((-b) + f64::sqrt(delta)) / (2.0 * a);
        let x2 = ((-b) - f64::sqrt(delta)) / (2.0 * a);
        println!("Two results: x1 = {}; x2 = {}.", x1, x2);
    }
    if delta == 0.0 {
        let x = (-b) / (2.0 * a);
        println!("One result: x = {}.", x);
    }
    if delta < 0.0 {
        println!("No result.");
    }
}