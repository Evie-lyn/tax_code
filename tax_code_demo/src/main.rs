use clap:: Parser;
use tax_code::{calculate_income_tax, FilingStatus};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    #[clap(short, long)]
    state: String, // State of residence (ex CA, TX, TN)
    #[clap(short, long)]
    income: f64, // Your taxable income
    #[clap(short, long, default_value = "single")]
    filing_status: String, // Filing status: single, married filing jointly, married filing separately, qualifying surviving spouse, head of household
    #[clap(short, long)]
    year: i32,
}

// To run input 'cargo run -- --state "CA" --income 50000 --year 2024 --filing-status "single"'
fn main() {
    let args = Args::parse();

    println!("Calculating taxes for: State: {}, Income: ${:.2}, Filing Status: {}, Year: {}",
            args.state, args.income, args.filing_status, args.year);

     let filing_status_enum = match args.filing_status.to_lowercase().as_str() {
        "single" => FilingStatus::Single,
        "married filing jointly" => FilingStatus::MarriedFilingJointly,
        "married filing separately" => FilingStatus::MarriedFilingSeparately,
        "qualifying surviving spouse" => FilingStatus::QualifyingSurvivingSpouse,
        "head of household" => FilingStatus::HeadOfHousehold,
        _ => {
            eprintln!("Warning: Invalid filing status '{}', defaulting to Single.", args.filing_status);
            FilingStatus::Single
        }
    };
   let taxes = calculate_income_tax(&args.state, args.income, filing_status_enum, args.year);
   println!("Income Tax: ${:.2}", taxes)
}