
use rbt::RedBlackTreeSet;
use rand::Rng;
use std::collections::HashSet;
use std::io;
use std::io::Write;

fn main() {
   
    let mut rb_tree_fix:RedBlackTreeSet<u32>= RedBlackTreeSet::new();
    
    
    rb_tree_fix.insert(1);
    
    rb_tree_fix.insert(2);
    rb_tree_fix.insert(3);
    rb_tree_fix.insert(4);
    rb_tree_fix.insert(5);
    rb_tree_fix.insert(6);
    rb_tree_fix.insert(7);
    rb_tree_fix.insert(8);
    rb_tree_fix.insert(9);
    rb_tree_fix.insert(11);
    rb_tree_fix.insert(12);
    rb_tree_fix.insert(13);
    rb_tree_fix.insert(14);
    rb_tree_fix.insert(15);
    rb_tree_fix.insert(16);
    rb_tree_fix.insert(17);
    rb_tree_fix.insert(18);
    rb_tree_fix.insert(19);

    rb_tree_fix.print_tree();

    rb_tree_fix.delete(15);
    
    rb_tree_fix.print_tree();





    // let mut tree:RedBlackTreeSet<u32>= RedBlackTreeSet::new();
    // loop {
         
    //     println!("Red-Black Tree Operations:");
    //     println!("1. Insert");
    //     println!("2. Search");
    //     println!("3. Delete");
    //     println!("4. Exit");

    //     print!("Please enter your choice: ");
    //     io::stdout().flush().unwrap();

    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).unwrap();
    //     let choice: u32 = match input.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => {
    //             println!("Invalid input. Please enter a number.");
    //             continue;
    //         }
    //     };

    //     match choice {
    //         1 => {
    //             print!("What would you like to insert");
    //             io::stdout().flush().unwrap();
    //             // Insert operation
    //             let mut key_input = String::new();
    //             io::stdin().read_line(&mut key_input).unwrap();
    //             let key: u32 = match key_input.trim().parse() {
    //                 Ok(num) => num,
    //                 Err(_) => {
    //                     println!("Invalid input. Please enter a number.");
    //                     continue;
    //                 }
    //             };
    //             tree.insert(key);
    //             println!("\n\n");
    //             tree.print_tree();
    //             println!("\n\n");
    //             io::stdout().flush().unwrap();
    //         }
    //         2 => {
    //             print!("Enter a number to find in the tree");
    //             io::stdout().flush().unwrap();
    //             // search operation
    //             let mut key_input = String::new();
    //             io::stdin().read_line(&mut key_input).unwrap();
    //             let key: u32 = match key_input.trim().parse() {
    //                 Ok(num) => num,
    //                 Err(_) => {
    //                     println!("Invalid input. Please enter a number.");
    //                     continue;
    //                 }
    //             };
    //             if tree.find(key).is_some() {

    //                println!("Found {}", key);

    //             } else {
    //                 println!("{} not found", key);
    //             }
    //             io::stdout().flush().unwrap();
    //         }
    //         3 => {
    //             print!("What would you like to delete: ");
    //             io::stdout().flush().unwrap();
    //             // Insert operation
    //             let mut key_input = String::new();
    //             io::stdin().read_line(&mut key_input).unwrap();
    //             let key: u32 = match key_input.trim().parse() {
    //                 Ok(num) => num,
    //                 Err(_) => {
    //                     println!("Invalid input. Please enter a number.");
    //                     continue;
    //                 }
    //             };
    //             tree.delete(key);
    //             println!("\n\n");
    //             tree.print_tree();
    //             println!("\n\n");
    //             io::stdout().flush().unwrap();
    //         }
    //         4 => {
    //             // Exit the loop
    //             println!("Exiting...");
    //             break;
    //         }
    //         _ => {
    //             println!("Invalid choice. Please enter a valid option.");
    //         }
    //     }
    // }



}

mod rbt;