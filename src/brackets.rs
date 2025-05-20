//For states with no income tax
macro_rules! no_income_tax {
    () => {
        vec![Bracket(f64::INFINITY, 0.0)]
    };
}

pub struct Bracket(pub f64, pub f64); //(upper boundary, tax rate)

//Alabama Brackets for 2024 (All filing statuses use same brackets)
pub fn al_single_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(500.0, 0.02),
	Bracket(3000.0, 0.04),
	Bracket(f64::INFINITY, 0.05),
    ]
}
pub fn al_joint_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(1000.0, 0.02),
	Bracket(6000.0, 0.04),
	Bracket(f64::INFINITY, 0.05),
    ]
}

pub fn al_headhouse_tax_2024() -> Vec<Bracket> {
    vec![
        Bracket(500.0, 0.02),
        Bracket(3000.0, 0.04),
        Bracket(f64::INFINITY, 0.05),
    ]
}

//Alaska Brackets (No income tax)
pub fn ak_single_tax() -> Vec<Bracket> {
    no_income_tax!()
}

//Arizona Brackets for 2024 & 2025 (Flat tax)
pub fn az_single_tax() -> Vec<Bracket> {
    vec![
    Bracket (f64::INFINITY, 0.025)
    ]
}

//Arkansas Brackets for 2024 (All filing statuses use same brackets)
pub fn ar_single_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(5499.0, 0.0),
	Bracket(10899.0, 0.02),
    Bracket(15599.0, 0.03),
    Bracket(25699.0, 0.034),
	Bracket(f64::INFINITY, 0.039),
    ]
}

//California Brackets for 2024
pub fn ca_single_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(10756.0, 0.01),
	Bracket(25499.0, 0.02),
    Bracket(40245.0, 0.04),
    Bracket(55866.0, 0.06),
    Bracket(70606.0, 0.08),
    Bracket(360659.0, 0.093),
    Bracket(432787.0, 0.103),
    Bracket(721314.0, 0.113),
	Bracket(f64::INFINITY, 0.123),
    ]
}

pub fn ca_joint_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(21512.0, 0.01),
	Bracket(50998.0, 0.02),
    Bracket(80490.0, 0.04),
    Bracket(111732.0, 0.06),
    Bracket(141212.0, 0.08),
    Bracket(721318.0, 0.093),
    Bracket(865574.0, 0.103),
    Bracket(1442628.0, 0.113),
	Bracket(f64::INFINITY, 0.123),
    ]
}

pub fn ca_headhouse_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(21527.0, 0.01),
	Bracket(51000.0, 0.02),
    Bracket(65744.0, 0.04),
    Bracket(81364.0, 0.06),
    Bracket(96107.0, 0.08),
    Bracket(490493.0, 0.093),
    Bracket(588593.0, 0.103),
    Bracket(980987.0, 0.113),
	Bracket(f64::INFINITY, 0.123),
    ]
}

//California Brackets for 2025 NOT YET AVAILABLE
//PLS UPDATE BRACKETS
pub fn ca_single_tax_2025() -> Vec<Bracket> {
    vec![
        Bracket(10756.0, 0.01),
        Bracket(25499.0, 0.02),
        Bracket(40245.0, 0.04),
        Bracket(55866.0, 0.06),
        Bracket(70606.0, 0.08),
        Bracket(360659.0, 0.093),
        Bracket(432787.0, 0.103),
        Bracket(721314.0, 0.113),
        Bracket(f64::INFINITY, 0.123),
    ]
}

pub fn ca_joint_tax_2025() -> Vec<Bracket> {
    vec![
        Bracket(21512.0, 0.01),
        Bracket(50998.0, 0.02),
        Bracket(80490.0, 0.04),
        Bracket(111732.0, 0.06),
        Bracket(141212.0, 0.08),
        Bracket(721318.0, 0.093),
        Bracket(865574.0, 0.103),
        Bracket(1442628.0, 0.113),
        Bracket(f64::INFINITY, 0.123),
    ]
}

pub fn ca_headhouse_tax_2025() -> Vec<Bracket> {
    vec![
        Bracket(21527.0, 0.01),
        Bracket(51000.0, 0.02),
        Bracket(65744.0, 0.04),
        Bracket(81364.0, 0.06),
        Bracket(96107.0, 0.08),
        Bracket(490493.0, 0.093),
        Bracket(588593.0, 0.103),
        Bracket(980987.0, 0.113),
        Bracket(f64::INFINITY, 0.123),
    ]
}

//Colorado Brackets for 2024 (Flat tax)
pub fn co_single_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(f64::INFINITY, 0.0425),
    ]
}

//Colorado Brackets for 2025 (Flat tax)
pub fn co_single_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(f64::INFINITY, 0.044),
    ]
}

//Connecticut Brackets for 2024
pub fn ct_single_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(10000.0, 0.02),
	Bracket(50000.0, 0.045),
    Bracket(100000.0, 0.055),
    Bracket(200000.0, 0.06),
    Bracket(250000.0, 0.065),
    Bracket(500000.0, 0.069),
	Bracket(f64::INFINITY, 0.0699),
    ]
}

pub fn ct_joint_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(20000.0, 0.02),
	Bracket(100000.0, 0.045),
    Bracket(200000.0, 0.055),
    Bracket(400000.0, 0.06),
    Bracket(500000.0, 0.065),
    Bracket(1000000.0, 0.069),
	Bracket(f64::INFINITY, 0.0699),
    ]
}

pub fn ct_headhouse_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(16000.0, 0.02),
	Bracket(80000.0, 0.045),
    Bracket(160000.0, 0.055),
    Bracket(320000.0, 0.06),
    Bracket(400000.0, 0.065),
    Bracket(800000.0, 0.069),
	Bracket(f64::INFINITY, 0.0699),
    ]
}

//Connecticut Brackets for 2025
pub fn ct_single_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(10000.0, 0.02),
	Bracket(50000.0, 0.045),
    Bracket(100000.0, 0.055),
    Bracket(200000.0, 0.06),
    Bracket(250000.0, 0.065),
    Bracket(500000.0, 0.069),
	Bracket(f64::INFINITY, 0.0699),
    ]
}

pub fn ct_joint_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(20000.0, 0.02),
	Bracket(100000.0, 0.045),
    Bracket(200000.0, 0.055),
    Bracket(400000.0, 0.06),
    Bracket(500000.0, 0.065),
    Bracket(1000000.0, 0.069),
	Bracket(f64::INFINITY, 0.0699),
    ]
}

pub fn ct_headhouse_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(16000.0, 0.02),
	Bracket(80000.0, 0.045),
    Bracket(160000.0, 0.055),
    Bracket(320000.0, 0.06),
    Bracket(400000.0, 0.065),
    Bracket(800000.0, 0.069),
	Bracket(f64::INFINITY, 0.0699),
    ]
}

//Delaware Brackets for 2024
pub fn de_single_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(2000.0, 0.0),
	Bracket(5000.0, 0.022),
    Bracket(10000.0, 0.039),
    Bracket(20000.0, 0.048),
    Bracket(25000.0, 0.052),
    Bracket(60000.0, 0.0555),
	Bracket(f64::INFINITY, 0.066),
    ]
}

//Delaware Brackets for 2025
pub fn de_single_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(2000.0, 0.0),
	Bracket(5000.0, 0.022),
    Bracket(10000.0, 0.039),
    Bracket(20000.0, 0.048),
    Bracket(25000.0, 0.052),
    Bracket(60000.0, 0.0555),
	Bracket(f64::INFINITY, 0.066),
    ]
}

//Florida Brackets (No state income tax)
pub fn fl_single_tax() -> Vec<Bracket> {
    no_income_tax!()
}

//Georgia Brackets for 2024 (Flat tax)
pub fn ga_single_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(f64::INFINITY, 0.0539),
    ]
}

//Georgia Brackets for 2025 (Flat tax)
pub fn ga_single_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(f64::INFINITY, 0.0519),
    ]
}

//Texas Brackets (No state income tax)
pub fn tx_single_tax() -> Vec<Bracket> {
    no_income_tax!()
}

//Tennessee Brackets (No state income tax)
pub fn tn_single_tax() -> Vec<Bracket> {
    no_income_tax!()
}
