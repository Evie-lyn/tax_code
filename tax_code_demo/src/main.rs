//download clap & tax_code cargos!!
use clap:: Parser;
use tax_code::calculate_income_tax;

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

    println!("Calculating taxes for: {}, income: ${:.2}, filing status: {}",
            args.state, args.income, args.filing_status);

   let taxes = calculate_income_tax(&args.state, args.income, &args.filing_status);
   println!("Income Tax: ${:.2}", taxes)
}