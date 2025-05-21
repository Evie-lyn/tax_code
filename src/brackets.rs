//For states with no income tax
macro_rules! no_income_tax {
    () => {
        vec![Bracket(f64::INFINITY, 0.0)]
    };
}

pub struct Bracket(pub f64, pub f64); //(upper boundary, tax rate)

//Alabama Brackets for 2024
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

//Arizona Brackets for 2024(Flat tax)
pub fn az_single_tax_2024() -> Vec<Bracket> {
    vec![
    Bracket (f64::INFINITY, 0.025)
    ]
}

//Arizona Brackets for 2025(Flat tax)
pub fn az_single_tax_2025() -> Vec<Bracket> {
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

//Arkansas Brackets for 2025 (All filing statuses use same brackets)
pub fn ar_single_tax_2025() -> Vec<Bracket> {
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

//Hawaii Brackets for 2024
pub fn hi_single_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(2400.0, 0.014),
	Bracket(4800.0, 0.032),
    Bracket(9600.0, 0.055),
    Bracket(14400.0, 0.064),
    Bracket(19200.0, 0.068),
    Bracket(24000.0, 0.072),
    Bracket(36000.0, 0.076),
	Bracket(48000.0, 0.079),
    Bracket(150000.0, 0.0825),
    Bracket(175000.0, 0.09),
    Bracket(200000.0, 0.10),
	Bracket(f64::INFINITY, 0.11),
    ]
}

pub fn hi_joint_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(4800.0, 0.014),
	Bracket(9600.0, 0.032),
    Bracket(19200.0, 0.055),
    Bracket(28800.0, 0.064),
    Bracket(38400.0, 0.068),
    Bracket(48000.0, 0.072),
    Bracket(72000.0, 0.076),
    Bracket(96000.0, 0.079),
    Bracket(300000.0, 0.0825),
    Bracket(350000.0, 0.09),
    Bracket(400000.0, 0.10),
	Bracket(f64::INFINITY, 0.11),
    ]
}

pub fn hi_headhouse_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(3600.0, 0.014),
	Bracket(7200.0, 0.032),
    Bracket(14400.0, 0.055),
    Bracket(21600.0, 0.064),
    Bracket(28800.0, 0.068),
    Bracket(36000.0, 0.072),
    Bracket(54000.0, 0.076),
    Bracket(72000.0, 0.079),
    Bracket(225000.0, 0.0825),
    Bracket(262500.0, 0.09),
    Bracket(300000.0, 0.10),
	Bracket(f64::INFINITY, 0.11),
    ]
}

//Hawaii Brackets for 2025
pub fn hi_single_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(9600.0, 0.014),
	Bracket(14400.0, 0.032),
    Bracket(19200.0, 0.055),
    Bracket(24000.0, 0.064),
    Bracket(36000.0, 0.068),
    Bracket(48000.0, 0.072),
    Bracket(125000.0, 0.076),
	Bracket(175000.0, 0.079),
    Bracket(225000.0, 0.0825),
    Bracket(275000.0, 0.09),
    Bracket(325000.0, 0.10),
	Bracket(f64::INFINITY, 0.11),
    ]
}

pub fn hi_joint_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(19200.0, 0.014),
	Bracket(28800.0, 0.032),
    Bracket(38400.0, 0.055),
    Bracket(48000.0, 0.064),
    Bracket(72000.0, 0.068),
    Bracket(96000.0, 0.072),
    Bracket(250000.0, 0.076),
    Bracket(350000.0, 0.079),
    Bracket(450000.0, 0.0825),
    Bracket(550000.0, 0.09),
    Bracket(650000.0, 0.10),
	Bracket(f64::INFINITY, 0.11),
    ]
}

pub fn hi_headhouse_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(14400.0, 0.014),
	Bracket(21600.0, 0.032),
    Bracket(28800.0, 0.055),
    Bracket(36000.0, 0.064),
    Bracket(54000.0, 0.068),
    Bracket(72000.0, 0.072),
    Bracket(187500.0, 0.076),
    Bracket(262500.0, 0.079),
    Bracket(337600.0, 0.0825),
    Bracket(412500.0, 0.09),
    Bracket(487500.0, 0.10),
	Bracket(f64::INFINITY, 0.11),
    ]
}

//Idaho Brackets for 2024
pub fn id_single_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(4673.0, 0.0),
	Bracket(f64::INFINITY, 0.05695),
    ]
}

pub fn id_joint_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(9346.0, 0.0),
	Bracket(f64::INFINITY, 0.05695),
    ]
}

pub fn id_headhouse_tax_2024() -> Vec<Bracket> {
    vec![
        Bracket(9346.0, 0.0),
        Bracket(f64::INFINITY, 0.05695),
    ]
}

//Illinois Brackets for 2024 (Flat tax)
pub fn il_single_tax_2024() -> Vec<Bracket> {
    vec![
    Bracket (f64::INFINITY, 0.0495)
    ]
}

//Illinois Brackets for 2025 (Flat tax)
pub fn il_single_tax_2025() -> Vec<Bracket> {
    vec![
    Bracket (f64::INFINITY, 0.0495)
    ]
}

//Indiana Brackets for 2024 (Flat tax)
pub fn in_single_tax_2024() -> Vec<Bracket> {
    vec![
    Bracket (f64::INFINITY, 0.031)
    ]
}

//Indiana Brackets for 2025 (Flat tax)
pub fn in_single_tax_2025() -> Vec<Bracket> {
    vec![
    Bracket (f64::INFINITY, 0.03)
    ]
}

//Iowa Brackets for 2024
pub fn ia_single_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(6210.0, 0.044),
    Bracket(31050.0, 0.482),
	Bracket(f64::INFINITY, 0.057),
    ]
}

pub fn ia_joint_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(12420.0, 0.044),
    Bracket(62100.0, 0.0482),
	Bracket(f64::INFINITY, 0.057),
    ]
}

pub fn ia_headhouse_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(6210.0, 0.0440),
    Bracket(31050.0, 0.482),
	Bracket(f64::INFINITY, 0.057),
    ]
}

//Iowa Brackets for 2025 (Flat tax)
pub fn ia_single_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(f64::INFINITY, 0.038),
    ]
}

//Kansas Brackets for 2024
pub fn ks_single_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(23000.0, 0.052),
	Bracket(f64::INFINITY, 0.0558),
    ]
}

pub fn ks_joint_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(46000.0, 0.052),
	Bracket(f64::INFINITY, 0.0558),
    ]
}

pub fn ks_headhouse_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(23000.0, 0.052),
	Bracket(f64::INFINITY, 0.0558),
    ]
}

//Kentucky Brackets for 2024 (Flat tax)
pub fn ky_single_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(f64::INFINITY, 0.04),
    ]
}

//Kentucky Brackets for 2025 (Flat tax)
pub fn ky_single_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(f64::INFINITY, 0.04),
    ]
}

//Louisiana Brackets for 2024
pub fn la_single_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(12500.0, 0.0185),
    Bracket(50000.0, 0.035),
	Bracket(f64::INFINITY, 0.0425),
    ]
}

pub fn la_joint_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(25000.0, 0.0185),
    Bracket(100000.0, 0.035),
	Bracket(f64::INFINITY, 0.0425),
    ]
}

pub fn la_headhouse_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(12500.0, 0.0185),
    Bracket(50000.0, 0.035),
	Bracket(f64::INFINITY, 0.0425),
    ]
}

//Louisiana Brackets for 2025 (Flat tax)
pub fn la_single_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(f64::INFINITY, 0.03),
    ]
}

//Maine Brackets for 2024
pub fn me_single_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(26050.0, 0.058),
    Bracket(61600.0, 0.0675),
	Bracket(f64::INFINITY, 0.0715),
    ]
}

pub fn me_joint_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(52100.0, 0.058),
    Bracket(123250.0, 0.0675),
	Bracket(f64::INFINITY, 0.0715),
    ]
}

pub fn me_headhouse_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(39050.0, 0.058),
    Bracket(92450.0, 0.0675),
	Bracket(f64::INFINITY, 0.0715),
    ]
}

//Maine Brackets for 2025
pub fn me_single_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(26800.0, 0.058),
    Bracket(63450.0, 0.0675),
	Bracket(f64::INFINITY, 0.0715),
    ]
}

pub fn me_joint_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(53600.0, 0.058),
    Bracket(126900.0, 0.0675),
	Bracket(f64::INFINITY, 0.0715),
    ]
}

pub fn me_headhouse_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(40200.0, 0.058),
    Bracket(95150.0, 0.0675),
	Bracket(f64::INFINITY, 0.0715),
    ]
}

//Maryland Brackets for 2024
pub fn md_single_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(1000.0, 0.02),
	Bracket(2000.0, 0.03),
    Bracket(3000.0, 0.04),
    Bracket(100000.0, 0.0475),
    Bracket(125000.0, 0.05),
    Bracket(150000.0, 0.0525),
    Bracket(250000.0, 0.055),
	Bracket(f64::INFINITY, 0.0575),
    ]
}

pub fn md_joint_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(1000.0, 0.02),
	Bracket(2000.0, 0.03),
    Bracket(3000.0, 0.04),
    Bracket(150000.0, 0.0475),
    Bracket(175000.0, 0.05),
    Bracket(225000.0, 0.0525),
    Bracket(300000.0, 0.055),
	Bracket(f64::INFINITY, 0.0575),
    ]
}

pub fn md_headhouse_tax_2024() -> Vec<Bracket> {
    vec![
	Bracket(1000.0, 0.02),
	Bracket(2000.0, 0.03),
    Bracket(3000.0, 0.04),
    Bracket(150000.0, 0.0475),
    Bracket(175000.0, 0.05),
    Bracket(225000.0, 0.0525),
    Bracket(300000.0, 0.055),
	Bracket(f64::INFINITY, 0.0575),
    ]
}

//Maryland Brackets for 2025
pub fn md_single_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(1000.0, 0.02),
	Bracket(2000.0, 0.03),
    Bracket(3000.0, 0.04),
    Bracket(100000.0, 0.0475),
    Bracket(125000.0, 0.05),
    Bracket(150000.0, 0.0525),
    Bracket(250000.0, 0.055),
	Bracket(f64::INFINITY, 0.0575),
    ]
}

pub fn md_joint_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(1000.0, 0.02),
	Bracket(2000.0, 0.03),
    Bracket(3000.0, 0.04),
    Bracket(150000.0, 0.0475),
    Bracket(175000.0, 0.05),
    Bracket(225000.0, 0.0525),
    Bracket(300000.0, 0.055),
	Bracket(f64::INFINITY, 0.0575),
    ]
}

pub fn md_headhouse_tax_2025() -> Vec<Bracket> {
    vec![
	Bracket(1000.0, 0.02),
	Bracket(2000.0, 0.03),
    Bracket(3000.0, 0.04),
    Bracket(150000.0, 0.0475),
    Bracket(175000.0, 0.05),
    Bracket(225000.0, 0.0525),
    Bracket(300000.0, 0.055),
	Bracket(f64::INFINITY, 0.0575),
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