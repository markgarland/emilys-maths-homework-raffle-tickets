fn main() {
    let mut tickets: Vec<u16> = (1..=999).collect();
    print_tickets(&tickets);

    println!();
    println!("Ticket is odd");
    tickets.retain(|t| { t % 2 == 1 });
    print_tickets(&tickets);

    println!();
    println!("Ticket is a multiple of 3");
    tickets.retain(|t| { t % 3 == 0 });
    print_tickets(&tickets);

    println!();
    println!("Ticket in the tens place is twice the digit in the hundreds place");
    tickets.retain(|t| {
        let hundreds = get_number_at_index(t, 2);
        let tens = get_number_at_index(t, 1);

        // filter zeroes
        hundreds != 0 && tens != 0 && 2*hundreds == tens
    });
    print_tickets(&tickets);

    println!();
    println!("The digit in the ones place is 5 more than the digit in the tens place");
    tickets.retain(|t| {
        let ones = get_number_at_index(t, 0);
        let tens = get_number_at_index(t, 1);
        ones == tens + 5
    });
    print_tickets(&tickets);
}

fn print_tickets(tickets: &Vec<u16>) {
    println!("Tickets - {} left - [{:?}]", tickets.iter().count(), format_tickets(tickets));
}

fn format_tickets(tickets: &Vec<u16>) -> Vec<String> {
    tickets.iter().map(|t| format!("{:03}", t)).collect()
}

fn get_number_at_index(number: &u16, index: u32) -> u8 {
    ((number / 10_u16.pow(index)) % 10) as u8
}