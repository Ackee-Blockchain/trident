use trident_idl_spec::Idl;

pub(crate) fn process_program_id(idl: &Idl) -> String {
    // if program ID is present, use it
    if !idl.address.is_empty() {
        idl.address.clone()
    } else {
        // if program ID is not present, use placeholder
        // We might be able to parse it form program, but it
        // might not be necesarry as newer versions of IDL will contain it
        "fill corresponding program ID here".to_string()
    }
}
