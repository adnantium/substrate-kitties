use support::{decl_storage, decl_module, StorageValue, StorageMap,
    dispatch::Result, ensure};
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash};
use parity_codec::{Encode, Decode};

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Kitty<Hash, Balance> {
    id: Hash,
    dna: Hash,
    price: Balance,
    gen: u64,
}

pub trait Trait: balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        Kitties get(kitty): map T::Hash => Kitty<T::Hash, T::Balance>;
        KittyOwner get(kitty_owner): map T::Hash => Option<T::AccountId>;
        OwnedKitty get(kitty_of_owner): map T::AccountId => T::Hash;
        Nonce: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn create_kitty(origin) -> Result {
            let sender = ensure_signed(origin)?;

            let nonce = <Nonce<T>>::get();
            let random_seed = <system::Module<T>>::random_seed();
            let random_hash = (random_seed, sender.clone(), nonce).using_encoded(<T as system::Trait>::Hashing::hash);
            ensure!(!<KittyOwner<T>>::exists(random_hash), "Kitty already exists");

            let new_kitty = Kitty {
                id: random_hash,
                dna: random_hash,
                price: <T::Balance as As<u64>>::sa(0),
                gen: 0,
            };

            <Kitties<T>>::insert(new_kitty.id, &new_kitty);
            <KittyOwner<T>>::insert(new_kitty.id, &sender);
            <OwnedKitty<T>>::insert(&sender, new_kitty.id);

            <Nonce<T>>::mutate(|n| *n += 1);

            Ok(())
        }
    }
}