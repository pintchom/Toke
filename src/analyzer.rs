use crate::ast::ContractNode;

pub struct AnalyzerResult {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn analyze(contract: &ContractNode) -> AnalyzerResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    println!("Analyzing contract {}...", &contract.name);

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
    match &contract.supply {
        None => errors.push("supply is required".to_string()),
        Some(field) if field.value == 0 => errors.push(format!(
            "supply must be > 0 (line {}, col {})",
            field.position.line, field.position.col
        )),
        _ => {}
    }
}

fn check_decimals_range(contract: &ContractNode, errors: &mut Vec<String>) {
    if let Some(field) = &contract.decimals {
        if field.value > 77 {
            errors.push(format!(
                "decimals must be 0–77, got {} (line {}, col {})",
                field.value, field.position.line, field.position.col
            ));
        }
    }
}

fn check_capped_requires_mintable(contract: &ContractNode, errors: &mut Vec<String>) {
    if contract.capped.is_some() && contract.mintable.is_none() {
        let pos = &contract.capped.as_ref().unwrap().position;
        errors.push(format!(
            "capped requires mintable to be set (line {}, col {})",
            pos.line, pos.col
        ));
    }
}

fn check_capped_gte_supply(contract: &ContractNode, errors: &mut Vec<String>) {
    if let (Some(capped), Some(supply)) = (&contract.capped, &contract.supply) {
        if capped.value < supply.value {
            errors.push(format!(
                "capped ({}) must be >= supply ({}) (line {}, col {})",
                capped.value, supply.value, capped.position.line, capped.position.col
            ));
        }
    }
}

fn check_decimals_unusual(contract: &ContractNode, warnings: &mut Vec<String>) {
    if let Some(field) = &contract.decimals {
        if field.value != 18 {
            warnings.push(format!(
                "decimals is {} (expected 18) (line {}, col {})",
                field.value, field.position.line, field.position.col
            ));
        }
    }
}

fn check_symbol_length(contract: &ContractNode, warnings: &mut Vec<String>) {
    if let Some(field) = &contract.symbol {
        if field.value.len() > 5 {
            warnings.push(format!(
                "symbol '{}' is longer than 5 characters (line {}, col {})",
                field.value, field.position.line, field.position.col
            ));
        }
    }
}
