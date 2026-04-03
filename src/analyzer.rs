use crate::ast::ContractNode;
use crate::errors::{get_source_line, CompileError};

pub struct AnalyzerResult {
    pub errors: Vec<CompileError>,
    pub warnings: Vec<CompileError>,
}

pub fn analyze(contract: &ContractNode, source: &str) -> AnalyzerResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    check_supply(contract, source, &mut errors);
    check_decimals_range(contract, source, &mut errors);
    check_capped_requires_mintable(contract, source, &mut errors);
    check_capped_gte_supply(contract, source, &mut errors);
    check_decimals_unusual(contract, source, &mut warnings);
    check_symbol_length(contract, source, &mut warnings);

    AnalyzerResult { errors, warnings }
}

fn check_supply(contract: &ContractNode, source: &str, errors: &mut Vec<CompileError>) {
    match &contract.supply {
        None => errors.push(CompileError::semantic(
            "Missing required field 'supply'",
            contract.name_position.line,
            contract.name_position.col,
            get_source_line(source, contract.name_position.line),
        )),
        Some(field) if field.value == 0 => errors.push(CompileError::semantic(
            "Supply must be greater than 0",
            field.position.line,
            field.position.col,
            get_source_line(source, field.position.line),
        )),
        _ => {}
    }
}

fn check_decimals_range(contract: &ContractNode, source: &str, errors: &mut Vec<CompileError>) {
    if let Some(field) = &contract.decimals {
        if field.value > 77 {
            errors.push(CompileError::semantic(
                format!("Decimals must be between 0 and 77, got {}", field.value),
                field.position.line,
                field.position.col,
                get_source_line(source, field.position.line),
            ));
        }
    }
}

fn check_capped_requires_mintable(
    contract: &ContractNode,
    source: &str,
    errors: &mut Vec<CompileError>,
) {
    if contract.capped.is_some() && contract.mintable.is_none() {
        let pos = &contract.capped.as_ref().unwrap().position;
        errors.push(CompileError::semantic(
            "'capped' requires 'mintable' — a cap only applies if new tokens can be minted",
            pos.line,
            pos.col,
            get_source_line(source, pos.line),
        ));
    }
}

fn check_capped_gte_supply(
    contract: &ContractNode,
    source: &str,
    errors: &mut Vec<CompileError>,
) {
    if let (Some(capped), Some(supply)) = (&contract.capped, &contract.supply) {
        if capped.value < supply.value {
            errors.push(CompileError::semantic(
                format!(
                    "Cap ({}) is less than initial supply ({})",
                    capped.value, supply.value
                ),
                capped.position.line,
                capped.position.col,
                get_source_line(source, capped.position.line),
            ));
        }
    }
}

fn check_decimals_unusual(
    contract: &ContractNode,
    source: &str,
    warnings: &mut Vec<CompileError>,
) {
    if let Some(field) = &contract.decimals {
        if field.value != 18 {
            warnings.push(CompileError::warning(
                format!("Decimals is set to {}. Most tokens use 18", field.value),
                field.position.line,
                field.position.col,
                get_source_line(source, field.position.line),
            ));
        }
    }
}

fn check_symbol_length(contract: &ContractNode, source: &str, warnings: &mut Vec<CompileError>) {
    if let Some(field) = &contract.symbol {
        if field.value.len() > 5 || field.value.is_empty() {
            warnings.push(CompileError::warning(
                format!(
                    "Symbol '{}' is unusually long. Most tokens use 3-5 characters",
                    field.value
                ),
                field.position.line,
                field.position.col,
                get_source_line(source, field.position.line),
            ));
        }
    }
}
