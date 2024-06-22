use std::io::stdin;

fn main() {
    println!("version alpha 0.1");

    println!("Welcome, traveller! Enjoy this text-based amazing journey\n");
    println!("First, choose your class:\n");
    println!("Warrior");
    println!("Mage");
    println!("Scuf");

    let mut answer = String::new();

    loop
    {
        let _ = stdin().read_line(&mut answer).expect("HOW DID U DO THIS");
        let res = match answer.trim_end()
        {
            "Warrior" => "Great choice! I think you wanna crush your enemies with rage and pathos >=|",
            "Wizard" => "Some magic and idk. Just cast your shit",
            "Scuf" => "what",
            _ => { "think again retard..."; continue }
        };
        break;
    }

    //add structs for each class
    //add more actions
    //add random fighting
    //add skills like you have random from 15 to 20 how do you manage them

}
