pub struct Deduction {
    pub standard_deduction: f64,
}

//Alaska (no income tax, so no deduction)
pub fn ak_standard_deduction(_income: f64, _filing_status: &crate::FilingStatus) -> Deduction {
    Deduction { standard_deduction: 0.0 }
}

//Arizona 2024
pub fn az_standard_deduction_2024 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 14600.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 14600.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 29200.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 29200.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 21900.0 }, 
    }
}

//Arkansas 2024
pub fn ar_standard_deduction_2024 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 2340.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 2340.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 4680.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 2340.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 2340.0 }, 
    }
}

//California
pub fn ca_standard_deduction_2024 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 5540.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 5540.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 11080.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 11080.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 11080.0 }, 
    }
}

//Colorado (No Standard deduction)
pub fn co_standard_deduction(_income: f64, _filing_status: &crate::FilingStatus) -> Deduction {
    Deduction { standard_deduction: 0.0 }
}

//Connecticut (No Standard deduction)
pub fn ct_standard_deduction(_income: f64, _filing_status: &crate::FilingStatus) -> Deduction {
    Deduction { standard_deduction: 0.0 }
}

//Delaware
pub fn de_standard_deduction_2024 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 5700.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 5700.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 11400.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 5700.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 5700.0 }, 
    }
}

//Florida (no income tax, so no deduction)
pub fn fl_standard_deduction(_income: f64, _filing_status: &crate::FilingStatus) -> Deduction {
    Deduction { standard_deduction: 0.0 }
}

//Georgia
pub fn ga_standard_deduction_2024 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 12000.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 12000.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 24000.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 12000.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 12000.0 }, 
    }
}

//Hawaii
pub fn hi_standard_deduction_2024 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 4400.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 4400.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 8800.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 8800.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 6424.0 }, 
    }
}
pub fn hi_standard_deduction_2025 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 4400.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 4400.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 8800.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 8800.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 6424.0 }, 
    }
}

//Idaho
pub fn id_standard_deduction_2024 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 14600.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 14600.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 29200.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 29200.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 21900.0 }, 
    }
}
pub fn id_standard_deduction_2025 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 15000.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 15000.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 30000.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 30000.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 22500.0 }, 
    }
}

//Illinois (No Standard deduction)
pub fn il_standard_deduction(_income: f64, _filing_status: &crate::FilingStatus) -> Deduction {
    Deduction { standard_deduction: 0.0 }
}

//Indiana (No Standard deduction)
pub fn in_standard_deduction(_income: f64, _filing_status: &crate::FilingStatus) -> Deduction {
    Deduction { standard_deduction: 0.0 }
}

//Iowa
pub fn ia_standard_deduction_2024 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 14600.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 14600.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 29200.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 29200.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 21900.0 }, 
    }
}
pub fn ia_standard_deduction_2025 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 15000.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 15000.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 30000.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 30000.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 22500.0 }, 
    }
}

//Kansas
pub fn ks_standard_deduction_2024 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 3605.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 4120.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 8240.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 8240.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 6180.0 }, 
    }
}

//Kentucky
pub fn ky_standard_deduction_2024 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 3160.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 3160.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 3160.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 3160.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 3160.0 }, 
    }
}
pub fn ky_standard_deduction_2025 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 3270.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 3270.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 3270.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 3270.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 3270.0 }, 
    }
}

//Louisiana
pub fn la_standard_deduction_2024 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 4500.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 4500.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 9000.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 9000.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 9000.0 }, 
    }
}
pub fn la_standard_deduction_2025 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 12500.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 12500.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 25000.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 25000.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 25000.0 }, 
    }
}

//Maine
pub fn me_standard_deduction_2024 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 14600.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 14600.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 29200.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 29200.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 21900.0 }, 
    }
}
pub fn me_standard_deduction_2025 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 15000.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 15000.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 30000.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 30000.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 22500.0 }, 
    }
}

//Maryland
pub fn md_standard_deduction_2024 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 2700.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 2700.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 5450.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 5450.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 5450.0 }, 
    }
}

//Massachusetts (No Standard deduction)
pub fn ma_standard_deduction(_income: f64, _filing_status: &crate::FilingStatus) -> Deduction {
    Deduction { standard_deduction: 0.0 }
}

//Michigan (No Standard deduction)
pub fn mi_standard_deduction(_income: f64, _filing_status: &crate::FilingStatus) -> Deduction {
    Deduction { standard_deduction: 0.0 }
}

//Minnesota
pub fn mn_standard_deduction_2024 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 14575.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 14575.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 29150.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 29150.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 21900.0 }, 
    }
}
pub fn mn_standard_deduction_2025 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 14950.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 14950.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 29900.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 29900.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 22500.0 }, 
    }
}

//Mississippi
pub fn ms_standard_deduction_2024 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 2300.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 2300.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 4600.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 4600.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 3400.0 }, 
    }
}
pub fn ms_standard_deduction_2025 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 2300.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 2300.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 4600.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 4600.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 3400.0 }, 
    }
}

//Missouri
pub fn mo_standard_deduction_2024 (_income: f64, filing_status: &crate::FilingStatus) -> Deduction{
    match filing_status {
        crate::FilingStatus::Single => Deduction { standard_deduction: 14600.0 }, 
        crate::FilingStatus::MarriedFilingSeparately => Deduction { standard_deduction: 14600.0 }, 
        crate::FilingStatus::MarriedFilingJointly => Deduction { standard_deduction: 29200.0 },
        crate::FilingStatus::QualifyingSurvivingSpouse => Deduction { standard_deduction: 29200.0 },
        crate::FilingStatus::HeadOfHousehold => Deduction { standard_deduction: 21900.0 }, 
    }
}