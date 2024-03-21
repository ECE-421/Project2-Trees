use std::io::{self, Write};

use avl_trees::tree::Tree;


fn main() {
    let mut tree: Tree<i32> = Tree::new();

    let mut tree:Tree<u32>= Tree::new();
    loop {
        println!("AVL Tree Operations:");
        println!("1. Insert");
        println!("2. Search");
        println!("3. Delete");
        println!("4. Exit");

        print!("Please enter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                print!("What would you like to insert");
                io::stdout().flush().unwrap();
                // Insert operation
                let mut key_input = String::new();
                io::stdin().read_line(&mut key_input).unwrap();
                let key: u32 = match key_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };
                tree.insert(key);
                println!("\n\n");
                tree.print_tree(0, true);
                println!("\n\n");
                io::stdout().flush().unwrap();
            }
            2 => {
                print!("Enter a number to find in the tree");
                io::stdout().flush().unwrap();
                // search operation
                let mut key_input = String::new();
                io::stdin().read_line(&mut key_input).unwrap();
                let key: u32 = match key_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };
                if tree.search(key) {
                    println!("Found {}", key);

                } else {
                    println!("{} not found", key);
                }
                io::stdout().flush().unwrap();
            }
            3 => {
                print!("What would you like to delete: ");
                io::stdout().flush().unwrap();
                // Insert operation
                let mut key_input = String::new();
                io::stdin().read_line(&mut key_input).unwrap();
                let key: u32 = match key_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };
                tree.delete(key);
                println!("\n\n");
                tree.print_tree(0, true);
                println!("\n\n");
                io::stdout().flush().unwrap();
            }
            4 => {
                // Exit the loop
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a valid option.");
            }
        }
    }
}