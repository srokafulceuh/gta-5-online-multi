use std::io;

pub fn display_menu() {
    println!("Welcome to GTA 5 Online Multi");
    println!("1. Aimbot");
    println!("2. Speed Hack");
    println!("3. God Mode");
    println!("4. Infinite Ammo");
    println!("5. Teleport");
    println!("6. Spawn Vehicle");
    println!("7. Money Drop");
    println!("8. Exit");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    match choice.trim() {
        "1" => features::aimbot(),
        "2" => features::speed_hack(),
        "3" => features::god_mode(),
        "4" => features::infinite_ammo(),
        "5" => features::teleport(),
        "6" => features::spawn_vehicle(),
        "7" => features::money_drop(),
        "8" => std::process::exit(0),
        _ => println!("Invalid choice. Please try again."),
    }
}