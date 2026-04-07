fn main() {
    let mut tickets: Vec<u16> = (1..=999).collect();
    print_tickets(&tickets);

    println!();
    println!("Ticket is odd");
    tickets.retain(|t| {
        vec![1,3,5,7,9].contains(&get_ones(t))
    });
    print_tickets(&tickets);

    println!();
    println!("Ticket is a multiple of 3");
    tickets.retain(|t| { t % 3 == 0 });
    print_tickets(&tickets);

    println!();
    println!("Ticket in the tens place is twice the digit in the hundreds place");
    tickets.retain(|t| {
        // filter zeroes
        get_hundreds(t) != 0 && get_tens(t) != 0 && 2*get_hundreds(t) == get_tens(t)
    });
    print_tickets(&tickets);

    println!();
    println!("The digit in the ones place is 5 more than the digit in the tens place");
    tickets.retain(|t| {
        get_ones(t) == get_tens(t) + 5
    });
    print_tickets(&tickets);
}


fn print_tickets(tickets: &Vec<u16>) {
    println!("Tickets - {} left - [{:?}]", tickets.iter().count(), format_tickets(tickets));
}

fn format_tickets(tickets: &Vec<u16>) -> Vec<String> {
    tickets.iter().map(|t| format!("{:03}", t)).collect()
}

fn get_ones(t: &u16) -> u8 { get_number_at_index(t, 0) }
fn get_tens(t: &u16) -> u8 { get_number_at_index(t, 1) }
fn get_hundreds(t: &u16) -> u8 { get_number_at_index(t, 2) }

fn get_number_at_index(number: &u16, index: u32) -> u8 {
    ((number / 10_u16.pow(index)) % 10) as u8
}