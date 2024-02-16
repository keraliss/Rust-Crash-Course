// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

pub fn run() {
    let tickets = vec![
        TicketInfo {
            name: String::from("John"),
            price: 10.0,
            TicketType: TicketType::Standard,
        },
        TicketInfo {
            name: String::from("Jane"),
            price: 20.0,
            TicketType: TicketType::Backstage,
        },
        TicketInfo {
            name: String::from("Joe"),
            price: 30.0,
            TicketType: TicketType::Vip,
        },
    ];

    for ticket in tickets {
        match ticket.TicketType {
            TicketType::Standard => println!("Standard,Price:{}", ticket.price),
            TicketType::Vip => println!("Vip, Name:{}, Price:{}", ticket.name, ticket.price),
            TicketType::Backstage => {
                println!("Backstage, Name:{}, Price:{}", ticket.name, ticket.price)
            }
            _ => (),
        }
    }

    let ticket = vec![
        Ticket::Standard(10.0),
        Ticket::Vip {
            name: String::from("John"),
            price: 20.0,
        },
        Ticket::Backstage {
            name: String::from("John"),
            price: 30.0,
        },
    ];

    for tic in ticket {
        match tic {
            Ticket::Standard(price) => println!("Standard,Price:{}", price),
            Ticket::Vip { name, price } => println!("Vip, Name:{}, Price:{}", name, price),
            Ticket::Backstage { name, price } => {
                println!("Backstage, Name:{}, Price:{}", name, price)
            }
            _ => (),
        }
    }
}

enum TicketType {
    Standard,
    Vip,
    Backstage,
}

struct TicketInfo {
    name: String,
    price: f32,
    TicketType: TicketType,
}

// Alternative Solution

enum Ticket {
    Standard(f32),
    Vip { name: String, price: f32 },
    Backstage { name: String, price: f32 },
}
