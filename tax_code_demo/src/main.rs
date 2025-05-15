//download clap & tax_code cargos!!
use clap:: Parser;
use tax_code::{calculate_income_tax, FilingStatus};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    state: String, // State of residence (CA, TX, TN)
    income: f64, // Your taxable income
    #[clap(short, long, default_value = "single")]
    filing_status: String, // Filing status: single, married filing jointly, married filing separately, qualifying surviving spouse, head of household
}

// To run input 'cargo run -- "(State: CA,TX,TN)" (taxable income #)'
fn main() {
    let args = Args::parse();
    let tax_year: i32 = 2025;

    println!("Calculating taxes for: {}, income: ${:.2}, filing status: {}",
            args.state, args.income, args.filing_status);

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
   let taxes = calculate_income_tax(&args.state, args.income, filing_status_enum, tax_year);
   println!("Income Tax: ${:.2}", taxes)
}