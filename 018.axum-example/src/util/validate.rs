use validator::{Validate, ValidationErrors};

pub fn validate_payload<T: Validate>(payload: &T) -> Result<(), ValidationErrors> {
    Ok(payload.validate()?)
}