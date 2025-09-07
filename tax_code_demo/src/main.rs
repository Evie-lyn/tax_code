use clap::Parser;
use tax_code::{TaxCalculator, FilingStatus, TaxResult};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    #[clap(short, long)]
    state: String, // State of residence (ex CA, TX, TN)
    #[clap(short, long)]
    income: f64, // Primary taxpayer's taxable income
    #[clap(long, default_value_t = 0.0)]
    secondary_income: f64, // Secondary taxpayer's taxable income (for married couples)
    #[clap(long, default_value_t = 0.0)]
    capital_gains: f64, // Primary taxpayer's capital gains
    #[clap(long, default_value_t = 0.0)]
    secondary_capital_gains: f64, // Secondary taxpayer's capital gains
    #[clap(long, default_value_t = 0.0)]
    social_security: f64, // Primary taxpayer's social security income
    #[clap(long, default_value_t = 0.0)]
    secondary_social_security: f64, // Secondary taxpayer's social security income
    #[clap(short, long, default_value = "single")]
    filing_status: String, // Filing status: single, married filing jointly, married filing separately, qualifying surviving spouse, head of household
    #[clap(short, long)]
    year: i32,
    #[clap(long, default_value_t = 40)]
    age: i32, // Primary taxpayer's age
    #[clap(long, default_value_t = 40)]
    secondary_age: i32, // Secondary taxpayer's age
}

// To run input 'cargo run -- --state "CA" --income 50000 --year 2024 --filing-status "single"'
// For married couples: 'cargo run -- --state "CA" --income 50000 --secondary-income 45000 --year 2024 --filing-status "married filing jointly"'
fn main() {
    let args = Args::parse();

    println!("Calculating taxes for:");
    println!("  State: {}", args.state);
    println!("  Primary Income: ${:.2}", args.income);
    if args.secondary_income > 0.0 {
        println!("  Secondary Income: ${:.2}", args.secondary_income);
    }
    println!("  Filing Status: {}", args.filing_status);
    println!("  Year: {}", args.year);

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

    // Create the tax calculator instance (loads all calculators once)
    let tax_calculator = match TaxCalculator::new() {
        Ok(calculator) => calculator,
        Err(e) => {
            eprintln!("Error initializing tax calculator: {}", e);
            return;
        }
    };

    // Use the pre-loaded calculator
    let tax_result = tax_calculator.calculate_income_tax(
        &args.state, 
        args.income, 
        args.secondary_income,
        args.capital_gains,
        args.secondary_capital_gains,
        args.social_security,
        args.secondary_social_security,
        filing_status_enum, 
        args.year, 
        args.age,
        args.secondary_age
    );

    match tax_result {
        TaxResult::Joint(taxes) => {
            println!("\n=== Joint Tax Calculation ===");
            println!("Federal income tax: ${:.2}", taxes.federal_income_tax);
            println!("Federal capital gains tax: ${:.2}", taxes.federal_capital_gains_tax);
            println!("State income tax: ${:.2}", taxes.state_income_tax);
            println!("State capital gains tax: ${:.2}", taxes.state_capital_gains_tax);
            println!("Social security tax: ${:.2}", taxes.social_security_tax);
            println!("FICA tax: ${:.2}", taxes.fica_tax);
            
            let total = taxes.federal_income_tax + taxes.federal_capital_gains_tax 
                      + taxes.state_income_tax + taxes.state_capital_gains_tax 
                      + taxes.social_security_tax + taxes.fica_tax;
            println!("Total taxes: ${:.2}", total);
        }
        TaxResult::Separate(taxes) => {
            println!("\n=== Primary Taxpayer ===");
            println!("Federal income tax: ${:.2}", taxes.primary.federal_income_tax);
            println!("Federal capital gains tax: ${:.2}", taxes.primary.federal_capital_gains_tax);
            println!("State income tax: ${:.2}", taxes.primary.state_income_tax);
            println!("State capital gains tax: ${:.2}", taxes.primary.state_capital_gains_tax);
            println!("Social security tax: ${:.2}", taxes.primary.social_security_tax);
            println!("FICA tax: ${:.2}", taxes.primary.fica_tax);
            
            let primary_total = taxes.primary.federal_income_tax + taxes.primary.federal_capital_gains_tax 
                              + taxes.primary.state_income_tax + taxes.primary.state_capital_gains_tax 
                              + taxes.primary.social_security_tax + taxes.primary.fica_tax;
            println!("Primary total: ${:.2}", primary_total);

            if taxes.secondary.federal_income_tax > 0.0 || taxes.secondary.state_income_tax > 0.0 || taxes.secondary.social_security_tax > 0.0 || taxes.secondary.fica_tax > 0.0 {
                println!("\n=== Secondary Taxpayer ===");
                println!("Federal income tax: ${:.2}", taxes.secondary.federal_income_tax);
                println!("Federal capital gains tax: ${:.2}", taxes.secondary.federal_capital_gains_tax);
                println!("State income tax: ${:.2}", taxes.secondary.state_income_tax);
                println!("State capital gains tax: ${:.2}", taxes.secondary.state_capital_gains_tax);
                println!("Social security tax: ${:.2}", taxes.secondary.social_security_tax);
                println!("FICA tax: ${:.2}", taxes.secondary.fica_tax);
                
                let secondary_total = taxes.secondary.federal_income_tax + taxes.secondary.federal_capital_gains_tax 
                                    + taxes.secondary.state_income_tax + taxes.secondary.state_capital_gains_tax 
                                    + taxes.secondary.social_security_tax + taxes.secondary.fica_tax;
                println!("Secondary total: ${:.2}", secondary_total);
                println!("Combined total: ${:.2}", primary_total + secondary_total);
            }
        }
    }
}