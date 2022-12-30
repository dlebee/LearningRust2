enum Transaction
{
    Transfer(String, u128),
    Approve(String, u128),
    Mint(u128),
    Burn(u128)
}

fn print_transaction(transaction: &Transaction)
{
    match transaction {
        Transaction::Transfer(destination, amount) => {
            println!("transfer: {} {}", destination, amount);
        },
        Transaction::Approve(destination, amount) => {
            println!("approved: {} {}", destination, amount);
        },
        Transaction::Mint(amount) => {
            println!("mint: {}", amount);
        },
        Transaction::Burn(amount) => {
            println!("burn: {}", amount);
        }
    }
}

fn main() {
    let transfer = Transaction::Transfer(String::from("0x00"), 100_u128);
    let approve = Transaction::Approve(String::from("0x01"), 120_u128);
    let mint = Transaction::Mint(100_u128);
    let burn = Transaction::Burn(120_u128);

    print_transaction(&transfer);
    print_transaction(&approve);
    print_transaction(&mint);
    print_transaction(&burn);
}
