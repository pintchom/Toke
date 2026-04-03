#[cfg(test)]
mod tests {
    use toke::analyzer::*;
    use toke::ast::*;

    fn pos() -> Position {
        Position { line: 1, col: 1 }
    }

    fn str_field(value: &str) -> StringField {
        StringField {
            value: value.to_string(),
            position: pos(),
        }
    }

    fn int_field(value: u64) -> IntField {
        IntField {
            value,
            position: pos(),
        }
    }

    fn flag_field() -> FlagField {
        FlagField { position: pos() }
    }

    // Minimal valid contract — supply=1000, decimals=18, symbol="TKN"
    fn base_contract() -> ContractNode {
        ContractNode {
            name: "MyToken".to_string(),
            name_position: pos(),
            symbol: Some(str_field("TKN")),
            decimals: Some(int_field(18)),
            supply: Some(int_field(1000)),
            mintable: None,
            burnable: None,
            capped: None,
            owner: None,
        }
    }

    // --- check_supply ---

    #[test]
    fn test_supply_missing() {
        let contract = ContractNode {
            supply: None,
            ..base_contract()
        };
        let result = analyze(&contract);
        assert!(result.errors.iter().any(|e| e.contains("supply")));
    }

    #[test]
    fn test_supply_zero() {
        let contract = ContractNode {
            supply: Some(int_field(0)),
            ..base_contract()
        };
        let result = analyze(&contract);
        assert!(result.errors.iter().any(|e| e.contains("supply")));
    }

    #[test]
    fn test_supply_valid() {
        let result = analyze(&base_contract());
        assert!(result.errors.iter().all(|e| !e.contains("supply")));
    }

    // --- check_decimals_range ---

    #[test]
    fn test_decimals_too_high() {
        let contract = ContractNode {
            decimals: Some(int_field(78)),
            ..base_contract()
        };
        let result = analyze(&contract);
        assert!(result.errors.iter().any(|e| e.contains("decimals")));
    }

    #[test]
    fn test_decimals_at_max_boundary() {
        let contract = ContractNode {
            decimals: Some(int_field(77)),
            ..base_contract()
        };
        let result = analyze(&contract);
        assert!(result.errors.iter().all(|e| !e.contains("decimals")));
    }

    #[test]
    fn test_decimals_zero_valid() {
        let contract = ContractNode {
            decimals: Some(int_field(0)),
            ..base_contract()
        };
        let result = analyze(&contract);
        assert!(result.errors.iter().all(|e| !e.contains("decimals")));
    }

    #[test]
    fn test_decimals_absent_no_error() {
        let contract = ContractNode {
            decimals: None,
            ..base_contract()
        };
        let result = analyze(&contract);
        assert!(result.errors.is_empty());
    }

    // --- check_capped_requires_mintable ---

    #[test]
    fn test_capped_without_mintable_is_error() {
        let contract = ContractNode {
            capped: Some(int_field(2000)),
            mintable: None,
            ..base_contract()
        };
        let result = analyze(&contract);
        assert!(result.errors.iter().any(|e| e.contains("mintable")));
    }

    #[test]
    fn test_capped_with_mintable_ok() {
        let contract = ContractNode {
            capped: Some(int_field(2000)),
            mintable: Some(flag_field()),
            ..base_contract()
        };
        let result = analyze(&contract);
        assert!(result.errors.iter().all(|e| !e.contains("mintable")));
    }

    #[test]
    fn test_no_capped_no_mintable_ok() {
        let result = analyze(&base_contract());
        assert!(result.errors.iter().all(|e| !e.contains("mintable")));
    }

    // --- check_capped_gte_supply ---

    #[test]
    fn test_capped_less_than_supply_is_error() {
        let contract = ContractNode {
            capped: Some(int_field(500)),
            mintable: Some(flag_field()),
            supply: Some(int_field(1000)),
            ..base_contract()
        };
        let result = analyze(&contract);
        assert!(result.errors.iter().any(|e| e.contains("capped")));
    }

    #[test]
    fn test_capped_equal_to_supply_ok() {
        let contract = ContractNode {
            capped: Some(int_field(1000)),
            mintable: Some(flag_field()),
            supply: Some(int_field(1000)),
            ..base_contract()
        };
        let result = analyze(&contract);
        assert!(result.errors.iter().all(|e| !e.contains("capped")));
    }

    #[test]
    fn test_capped_greater_than_supply_ok() {
        let contract = ContractNode {
            capped: Some(int_field(2000)),
            mintable: Some(flag_field()),
            supply: Some(int_field(1000)),
            ..base_contract()
        };
        let result = analyze(&contract);
        assert!(result.errors.iter().all(|e| !e.contains("capped")));
    }

    // --- check_decimals_unusual ---

    #[test]
    fn test_decimals_not_18_warns() {
        let contract = ContractNode {
            decimals: Some(int_field(19)),
            ..base_contract()
        };
        let result = analyze(&contract);
        assert!(result.warnings.iter().any(|w| w.contains("decimals")));
    }

    #[test]
    fn test_decimals_18_no_warning() {
        let result = analyze(&base_contract());
        assert!(result.warnings.iter().all(|w| !w.contains("decimals")));
    }

    #[test]
    fn test_decimals_absent_no_warning() {
        let contract = ContractNode {
            decimals: None,
            ..base_contract()
        };
        let result = analyze(&contract);
        assert!(result.warnings.iter().all(|w| !w.contains("decimals")));
    }

    // --- check_symbol_length ---

    #[test]
    fn test_symbol_too_long_warns() {
        let contract = ContractNode {
            symbol: Some(str_field("TOOLONG")),
            ..base_contract()
        };
        let result = analyze(&contract);
        assert!(result.warnings.iter().any(|w| w.contains("symbol")));
    }

    #[test]
    fn test_symbol_exactly_5_ok() {
        let contract = ContractNode {
            symbol: Some(str_field("USDC0")),
            ..base_contract()
        };
        let result = analyze(&contract);
        assert!(result.warnings.iter().all(|w| !w.contains("symbol")));
    }

    #[test]
    fn test_symbol_absent_no_warning() {
        let contract = ContractNode {
            symbol: None,
            ..base_contract()
        };
        let result = analyze(&contract);
        assert!(result.warnings.iter().all(|w| !w.contains("symbol")));
    }

    // --- combined ---

    #[test]
    fn test_valid_contract_no_errors_or_warnings() {
        let result = analyze(&base_contract());
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }
}
