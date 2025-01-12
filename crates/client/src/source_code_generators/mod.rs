pub(crate) mod anchor_29;
pub(crate) mod anchor_30;

#[derive(Default)]
pub enum AnchorVersion {
    #[default]
    Unknown,
    V29(Vec<anchor_29::types::Idl>),
    V30(Vec<anchor_lang_idl_spec::Idl>),
}

impl From<String> for AnchorVersion {
    fn from(value: String) -> Self {
        if value == "anchor-cli 0.29.0" {
            AnchorVersion::V29(vec![])
        } else if value == "anchor-cli 0.30.0" || value == "anchor-cli 0.30.1" {
            AnchorVersion::V30(vec![])
        } else {
            AnchorVersion::Unknown
        }
    }
}

impl AnchorVersion {
    pub fn test_fuzz_generator(&self) -> String {
        match self {
            AnchorVersion::Unknown => {
                panic!("Unsupported Anchor version, cannot parse IDL (probably version too old)")
            }
            AnchorVersion::V29(idls) => anchor_29::test_fuzz_generator::generate_source_code(idls),
            AnchorVersion::V30(idls) => anchor_30::test_fuzz_generator::generate_source_code(idls),
        }
    }

    pub fn fuzz_instructions_generator(&self) -> String {
        match self {
            AnchorVersion::Unknown => {
                panic!("Unsupported Anchor version, cannot parse IDL (probably version too old)")
            }
            AnchorVersion::V29(idls) => {
                anchor_29::fuzz_instructions_generator::generate_source_code(idls)
            }
            AnchorVersion::V30(idls) => {
                anchor_30::fuzz_instructions_generator::generate_source_code(idls)
            }
        }
    }
}
