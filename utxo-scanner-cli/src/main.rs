use clap::Parser;
use utxo_scanner;


/// Scan for all UTXOs and save to a CSV file.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// Path to Bitcoin data directory (e.g., ~/.bitcoin, C:\Users\YourName\AppData\Roaming\Bitcoin).
    #[clap(short, long)]
    bitcoin_path: String,

    /// Output CSV file path.
    #[clap(short='o', long)]
    csv_path: String,

    /// Include mempool (unconfirmed, pre-blockchain) transactions in the scan.
    #[clap(short, long, default_value_t = false)]
    include_mempool: bool,

    /// Use the testnet network.
    #[clap(short='t', long, default_value_t = false)]
    testnet: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("Scanning Bitcoin UTXOs in {}", cli.bitcoin_path);
    println!("Saving to {}", cli.csv_path);
    println!("Dots show the progress of the scan. Each dot represents 10,000 UTXOs.");

    let stats = utxo_scanner::scan(
        &cli.bitcoin_path,
        cli.include_mempool,
        Some(&cli.csv_path),
        cli.testnet
    );

    println!("Total Transactions: {}", stats.count);
    println!("Total Amount: {}", stats.amount);
    println!("Total Seconds: {}", stats.total_secs);
}
