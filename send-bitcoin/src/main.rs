use std::io;
use rand::Rng;

fn send_bitcoin() {
    println!("\n We're going to send some Bitcoin! \n");

    let clients = vec!["john", "mary", "samuel", "ehis"];

    println!("who do you want to send bitcoin to? \n");

    for client in &clients{
        
        print!("{} ", client);
    }
    println!("\n");

    let mut receipient = String::new();
    io::stdin().read_line(&mut receipient);


    if clients.contains(&receipient.trim()){
        println!("How many bitcoin do you want to send? \n");

        
    let mut amount = String::new();
    io::stdin().read_line(&mut amount);

    println!("\n you sent {} bitcoin to {}", amount.trim(), receipient.trim() );

    } else {
        println! ("{} is not in your contacts list!", receipient.trim());
    }

}

fn receive_bitcoin() {
    println!("\n We're going to receive some Bitcoin! \n");

    let amount = rand::thread_rng().gen_range(1, 10);

    println!("you just received {} Bitcoin! \n", amount)
}

fn exit_console() {
    println!("Invalid option, must be (s) or (r)");
}

fn console() {
    println!("\nLet's have some fun with bitcoin \n");

    println!("Do you want to send(s) or receive (r) Bitcoin? \n");

    let mut command = String::new();

    io::stdin().read_line(&mut command);

    if command.trim().eq("s") {
        send_bitcoin()
    } else if command.trim().eq("r") {
        receive_bitcoin()
    } else {
        exit_console()
    }

}



fn main() {
    console();
}
