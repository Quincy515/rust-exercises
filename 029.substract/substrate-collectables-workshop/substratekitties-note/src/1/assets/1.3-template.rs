// ACTION: Import `support::dispatch::Result` in order to use the correct
//         `Result` return type for `set_value`.
//   HINT: Again, you only need to add `dispatch::Result` to the list below.
use support::{decl_storage, decl_module, StorageValue};

pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        Value: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn set_value(origin, value: u64) -> Result {
            // ACTION: "Ensure" that the transaction is signed

            // ACTION: "Put" the value into storage
            
            Ok(())
        }
    }
}