pub struct Position {
    pub line: usize,
    pub col: usize,
}

pub struct StringField {
    pub value: String,
    pub position: Position,
}

pub struct IntField {
    pub value: u64,
    pub position: Position,
}

pub struct AddressField {
    pub value: [u8; 20],
    pub position: Position,
}

pub struct FlagField {
    pub position: Position,
}

pub struct ContractNode {
    pub name: String,
    pub name_position: Position,
    pub symbol: Option<StringField>,
    pub decimals: Option<IntField>,
    pub supply: Option<IntField>,
    pub mintable: Option<FlagField>,
    pub burnable: Option<FlagField>,
    pub capped: Option<IntField>,
    pub owner: Option<AddressField>,
}
