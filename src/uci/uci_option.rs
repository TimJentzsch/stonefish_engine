#[allow(dead_code)]
pub enum UciOptionType {
    Check,
    Spin,
    Combo,
    Button,
    String,
}

impl UciOptionType {
    pub fn stringify(&self) -> &str {
        match self {
            UciOptionType::Check => "check",
            UciOptionType::Spin => "spin",
            UciOptionType::Combo => "combo",
            UciOptionType::Button => "button",
            UciOptionType::String => "string",
        }
    }
}

pub struct UciOption {
    name: String,
    option_type: UciOptionType,
    default: Option<String>,
    min: Option<String>,
    max: Option<String>,
    vars: Option<Vec<String>>,
}

impl UciOption {
    /// Create a new UCI option.
    pub fn new(name: &str, option_type: UciOptionType) -> UciOption {
        UciOption {
            name: name.to_string(),
            option_type,
            default: None,
            min: None,
            max: None,
            vars: None,
        }
    }

    /// Send the option from the engine to the GUI
    pub fn send_option(&self) {
        // Mandatory options
        let mut output = format!(
            "option name {} type {}",
            self.name,
            self.option_type.stringify()
        );

        // Default
        if let Some(default) = &self.default {
            output += format!(" default {}", default).as_str();
        }

        // Min
        if let Some(min) = &self.min {
            output += format!(" min {}", min).as_str();
        }

        // Max
        if let Some(max) = &self.max {
            output += format!(" max {}", max).as_str();
        }

        // Var
        if let Some(vars) = &self.vars {
            for var in vars {
                output += format!(" var {}", var).as_str();
            }
        }

        println!("{}", output);
    }
}
