#[derive(Debug)]

pub struct HelicopterData {
    helicopter_number: usize,
    fuel: u64,
    visibility: u64
}

pub trait Flying {
    fn api(&self);
    fn enter_data()->Vec<HelicopterData>;
    fn fly_helicopter(helicopter: &mut Vec<HelicopterData>);
}

impl Flying for HelicopterData {
    fn api(&self) {

        let mut helo_list:Vec<HelicopterData> = Self::enter_data();

        Self::fly_helicopter(&mut helo_list);
    }

    fn enter_data() -> Vec<HelicopterData> {

        let mut helo_count_input = String::new();
        println!("How many helicopters are flying?");

        std::io::stdin().read_line(&mut helo_count_input).expect("Error: Input was unreadable.");
        let helo_count: usize = helo_count_input.trim().parse().ok().expect("Enter an integer bewtween 1-100.");

        if helo_count == 1 {
            println!("\nThere is {} helicopter flying.", helo_count);
        } else {
            println!("\nThere are {} helicopters flying.", helo_count);
        }

        let mut helicopter: Vec<HelicopterData> = Vec::new();

        for i in 0..helo_count {

            // Fuel Input
            let mut fuel_level_input = String::new();
            println!("\nPlease enter the fuel level of Helicopter {} (Any integer between 1-100):", (i+1));

            std::io::stdin().read_line(&mut fuel_level_input).expect("Error: Input was not an acceptable integer.");
            let fuel_level: u64 = fuel_level_input.trim().parse().ok().expect("Enter an integer bewtween 1-100.");

            if fuel_level <= 0 {
                println!("\n{}% is an invalid entry, there must be some fuel in the helicopter.", fuel_level);
                println!("Please refuel Helicopter {}, restart the program and enter the new fuel level.\n", (i+1));
                panic!("PROGRAM TERMINATED.");
            } else if fuel_level > 100 {
                println!("\n{}% is an invalid entry, the fuel is overflowing from the helicopter.", fuel_level);
                println!("Please remove some fuel from Helicopter {}, restart the program and enter the new fuel level.\n", (i+1));
                panic!("PROGRAM TERMINATED.");
            }

            // Visibility Input
            let mut visibility_input = String::new();
            println!("\nPlease enter the visibility of Helicopter {} (Any integer between 1-100):", (i+1));

            std::io::stdin().read_line(&mut visibility_input).expect("Error: Input was not an acceptable integer.");
            let visibility: u64 = visibility_input.trim().parse().ok().expect("Enter an integer bewtween 1-100.");
            if visibility <= 0 {
                println!("\n{}% is an invalid entry, visibility cannot be at or below 0%. Our pilots would be flying blind.", visibility);
                println!("Please check the visibility for Helicopter {} again, restart the program and enter an integer between 1-100.\n", (i+1));
                panic!("PROGRAM TERMINATED.");
            } else if visibility > 100 {
                println!("\n{}% is an invalid entry, more than 100% visibility is impossible.", visibility);
                println!("If this is the case, you've entered an alternate universe..."); 
                println!("Please check the visibility for Helicopter {} again, restart the program and enter an integer between 1-100.\n", (i+1));
                panic!("PROGRAM TERMINATED.");
            }      

            println!("\nHelicopter {}:", (i+1));
            println!("Fuel level is at {}%.", fuel_level);
            println!("Visibility is at {}%.", visibility);
            
            let helicopter_inputs = HelicopterData {
                fuel: fuel_level,
                visibility,
                helicopter_number: i+1,
            };

            helicopter.push(helicopter_inputs);
        }

        println!("\nHelicopter Data Summary:");
        println!("{:?}", helicopter);

        helicopter
    }

    fn fly_helicopter(helicopter: &mut Vec<HelicopterData>) {

        while helicopter.len() > 0 {

            let mut i = 0;

            while i < helicopter.len() {

                if helicopter[i].visibility < 60 {

                    println!("\nHelicopter {} Status: Grounded", helicopter[i].helicopter_number);
                    println!("Due to visibility concerns, Helicopter {} is not safe to fly.", helicopter[i].helicopter_number);

                    helicopter.remove(i);
                }
                
                if helicopter[i].fuel >= 10 {

                    println!("\nHelicopter {} Status: Flying", helicopter[i].helicopter_number);
                    println!("Helicopter {} fuel level at {}%.", helicopter[i].helicopter_number, helicopter[i].fuel);

                    helicopter[i].fuel = helicopter[i].fuel - 2;

                } else {

                    println!("\nHelicopter {} fuel level at {}%, which is below the 10% threshold.", helicopter[i].helicopter_number, helicopter[i].fuel);
                    println!("Helicopter {} Status: Landing...", helicopter[i].helicopter_number);

                    helicopter.remove(i);
                }
                
                i += 1;
            }
        }
        
        println!("\nAll helicopters are on the ground.");
        println!("Refuel, check visibility conditions, and restart program to fly again!\n")
    }
}

fn main() {
    let helicopter = HelicopterData {  
        helicopter_number: 1,       
        fuel: 10,
        visibility: 75
    };

    helicopter.api();
}