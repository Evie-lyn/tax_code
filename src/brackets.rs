pub struct Bracket(pub f64, pub f64); //(upper boundary, tax rate)

//California Brackets
//'single' or 'married filing separately'
pub fn ca_single_tax() -> Vec<Bracket> {
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

//'married filing jointly' or 'qualifying surviving spouse'
pub fn ca_joint_tax() -> Vec<Bracket> {
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

//Brackets used for 'head of household'
pub fn ca_headhouse_tax() -> Vec<Bracket> {
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

//Texas Brackets (No state income tax)
pub fn tx_single_tax() -> Vec<Bracket> {
    vec![Bracket(f64::INFINITY, 0.0)]
}

//Tennessee Brackets (No state income tax)
pub fn tn_single_tax() -> Vec<Bracket> {
    vec![Bracket(f64::INFINITY, 0.0)]
}