use crate::ast::ContractNode;

pub struct AnalyzerResult {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn analyze(contract: &ContractNode) -> AnalyzerResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // supply present and positive required
    check_supply(contract, &mut errors);

    // check decimal range
    check_decimals_range(contract, &mut errors);

    // check cross-dependent fields
    check_capped_requires_mintable(contract, &mut errors);
    check_capped_gte_supply(contract, &mut errors);

    // check strange values
    check_decimals_unusual(contract, &mut warnings);
    check_symbol_length(contract, &mut warnings);

    AnalyzerResult { errors, warnings }
}

fn check_supply(contract: &ContractNode, errors: &mut Vec<String>) {
    // supply must be present and > 0
    todo!()
}

fn check_decimals_range(contract: &ContractNode, errors: &mut Vec<String>) {
    // decimals must be 0–77 (if provided)
    todo!()
}

fn check_capped_requires_mintable(contract: &ContractNode, errors: &mut Vec<String>) {
    // capped without mintable is an error
    todo!()
}

fn check_capped_gte_supply(contract: &ContractNode, errors: &mut Vec<String>) {
    // capped value must be >= supply value
    todo!()
}

fn check_decimals_unusual(contract: &ContractNode, warnings: &mut Vec<String>) {
    // warn if decimals is set but not 18
    todo!()
}

fn check_symbol_length(contract: &ContractNode, warnings: &mut Vec<String>) {
    // warn if symbol is longer than 5 characters
    todo!()
}
