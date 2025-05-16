//For states with no income tax
macro_rules! no_income_tax {
    () => {
        vec! [Bracket(f64::INFINITY, 0.0)]
    };
}

pub struct Bracket(pub f64, pub f64); //(upper boundary, tax rate)

//California Brackets for 2024
pub fn ca_single_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(10412.0, 0.01),
	Bracket(24684.0, 0.02),
    Bracket(38959.0, 0.04),
    Bracket(54081.0, 0.06),
    Bracket(68350.0, 0.08),
    Bracket(349137.0, 0.093),
    Bracket(418961.0, 0.103),
    Bracket(698271.0, 0.113),
	Bracket(f64::INFINITY, 0.123),
    ]
}

pub fn ca_joint_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(20824.0, 0.01),
	Bracket(49368.0, 0.02),
    Bracket(77918.0, 0.04),
    Bracket(108162.0, 0.06),
    Bracket(136700.0, 0.08),
    Bracket(698274.0, 0.093),
    Bracket(837922.0, 0.103),
    Bracket(1396542.0, 0.113),
	Bracket(f64::INFINITY, 0.123),
    ]
}

pub fn ca_headhouse_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(20839.0, 0.01),
	Bracket(49371.0, 0.02),
    Bracket(63644.0, 0.04),
    Bracket(78765.0, 0.06),
    Bracket(93037.0, 0.08),
    Bracket(474824.0, 0.093),
    Bracket(569790.0, 0.103),
    Bracket(949649.0, 0.113),
	Bracket(f64::INFINITY, 0.123),
    ]
}

//California Brackets for 2025
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

//Texas Brackets (No state income tax)
pub fn tx_single_tax() -> Vec<Bracket> {
    no_income_tax!()
}

//Tennessee Brackets (No state income tax)
pub fn tn_single_tax() -> Vec<Bracket> {
    no_income_tax!()
}