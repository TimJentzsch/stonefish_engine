pub enum UciOptionType {
    Check,
    Spin,
    Combo,
    Button,
    String,
}

pub struct UciOption {
    name: String,
    option_type: UciOptionType,
    default: Option<String>,
    min: Option<String>,
    max: Option<String>,
    var: Option<Vec<String>>,
}
