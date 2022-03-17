mod tictactoe;

fn main() {
    println!("You can type coordinate (e. g. A1, B3, C2) to make your turn.");
    println!("^C (Ctrl-C) to exit.");
    println!("");
    println!("A1 B1 C1");
    println!("A2 B2 C2");
    println!("A3 B3 C3");

    tictactoe::start_game();
}
