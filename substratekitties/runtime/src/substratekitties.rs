use parity_codec::{Decode, Encode};
use runtime_primitives::traits::{As, Hash};
use support::{decl_module, decl_storage, dispatch::Result, StorageMap};
use system::ensure_signed;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<Hash, Balance> {
    id: Hash,
    dna: Hash,
    price: Balance,
    gen: u64,
}

pub trait Trait: balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        OwnedKitty get(kitty_of_owner): map T::AccountId => Kitty<T::Hash, T::Balance>;
        Value: map T::AccountId => u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn create_kitty(origin) -> Result {
            let sender = ensure_signed(origin)?;

            let new_kitty = Kitty {
                id: <T as system::Trait>::Hashing::hash_of(&0),
                dna: <T as system::Trait>::Hashing::hash_of(&0),
                price: <T::Balance as As<u64>>::sa(0),
                gen: 0,
            };

            <OwnedKitty<T>>::insert(sender, new_kitty);

            Ok(())
        }

        fn set_value(origin, value: u64) -> Result{
            let sender = ensure_signed(origin)?;

            <Value<T>>::insert(sender, value);

            Ok(())
        }

    }
}
