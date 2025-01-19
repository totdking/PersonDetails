use std::io::{self,Write};
use std::sync::atomic::{AtomicUsize,Ordering};
pub mod errors;
#[derive(Debug)]
enum Gender{
    Male, Female
}
#[derive(Debug)]
struct PersonDetails{
    name: String,
    age: u32,
    gender: Gender,
    counter: usize,
}

static COUNTER: AtomicUsize = AtomicUsize::new(1);//creating an atomic size for the counter of PersonDetails
impl PersonDetails{
    fn new(name: String, age: u32, gender: Gender) -> Self {
        let counter = COUNTER.fetch_add(1, Ordering::SeqCst); // get and increment user id as it is being passed in
        PersonDetails { name, age, gender, counter }
    }
}

fn collect_info() -> Result<PersonDetails, String>{
        let mut name  = String::new();
        println!("Input your name");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut name).map_err(|e| format!("could not read name : {}",e))?;
        let person_name = name.trim().to_string();
        
    
    let person_age: u32;
    loop {
        let mut age  = String::new();
        println!("Input your age");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut age) {
            Ok(_) =>{ 
                match age.trim().parse::<u32>() {
                    Ok(parsed_age)=>{
                        person_age = parsed_age; //assign parsed_age to person_age
                        println!("Your age is saved successfully");
                        break;
                    } ,
                    Err(_) => {
                        println!("pls input a valid number type suitable for age.");
                        continue;
                    },
                }
            },
            Err(_) =>{ println!("Could not read line");
                continue;
            }
        }
    }
    
    let person_gender : Gender;
    loop{
        println!("Choose your gender");
        println!("Option 1: Male");
        println!("Option 2: Female");
        io::stdout().flush().unwrap();
        let mut gender = String::new();

        match io::stdin().read_line(&mut gender){
            Ok(_) => {
                match gender.trim().to_lowercase().as_str() {
                    "1" | "male" => {
                        person_gender = Gender::Male;
                        println!("Gender successfully saved as Male");
                        break;
                    }
                    "2" | "female" => {
                        person_gender = Gender::Female;
                        println!("Gender saved successfully as female");
                        break;
                    }
                    _ => {
                        println!("pls pick 1 for male and 2 for female");
                        continue;
                    }
                }
            },
            Err(_) => {
                println!("Could not read line pls try again");
                continue;
            }
        }
    }
    Ok(PersonDetails::new(person_name,person_age,person_gender,))
    
}

fn main() {
    let mut storage: Vec<PersonDetails> = vec![];
    let mut store_details = || {
        //lets' assume we want to take in the first 3 people
        let mut n = 1;
        while n <= 3 {
            match collect_info() {
                Ok(person) => storage.push(person),
                Err(e) => println!("error {}", e),
            }
            n+=1;
        }
        println!("the array of People displayed are : {:#?}", storage);
    };
    store_details();

    // let mut get_details = ||-> Result<PersonDetails,String>{
    //     println!("Whose details do you want to get");
    //     let mut position = String::new();
    //     io::stdout().flush().unwrap();
    //     io::stdin().read_line(&mut position).expect("Could not read line");
    //     let position: usize = position.trim().parse().expect("Expected a valid number");

    //     if position > storage.len() - 1 {
    //         Err("the position you entered is out of range".to_string())
    //     }
    //     else{
    //         for i in 0..storage.len() - 1{
    //             Ok(storage[position])
    //         }
    //     }
    // };
    // get_details();

    // println!("the postiton of storage is {:?}", storage[2])
}
