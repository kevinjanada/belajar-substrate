# Create new pallet
```shell
cargo new pallet/nft-pallet --lib
```

# NFT Pallet
## NFT Struct
```rust
use sp_std::prelude::*; // Needed for Vec

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct Nft<Hash, AccountId> {
    id: Hash, // H256
    owner_id: AccountId,
    name: Vec<u8>, // Text (string)
    image: Vec<u8>, // Text (string)
}
impl<Hash, AccountId> Nft<Hash, AccountId> {
    pub fn new(id: Hash, owner_id: AccountId, name: Vec<u8>, image: Vec<u8>) -> Self {
        Self { id, owner_id, name, image }
    }
}
```

## Types
```rust
pub type HashOf<T> = <T as frame_system::Config>::Hash;
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
```

## Storage
```rust
#[pallet::storage]
#[pallet::getter(fn nft_by_id)]
pub(super) type Nfts<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, Nft<HashOf<T>, AccountIdOf<T>>>;

#[pallet::storage]
#[pallet::getter(fn nfts_by_owner)]
pub(super) type NftsByOwner<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<HashOf<T>>>;
```

## Event
```rust
#[pallet::event]
#[pallet::metadata(T::AccountId = "AccountId")]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    /// Event documentation should end with an array that provides descriptive names for event
    /// parameters. [Nft]
    NftCreated(Nft<HashOf<T>, AccountIdOf<T>>),
}
```


## Pallet Methods
```rust
use frame_support::sp_runtime::traits::Hash; // Needed for T::Hashing::hash

impl<T: Config> Pallet<T> {
    pub fn create_nft(owner_id: &T::AccountId, name: &Vec<u8>, image: &Vec<u8>) -> Nft<HashOf<T>> {
        let nfts = Self::nfts_by_owner(owner_id).unwrap_or(Vec::new());
        let nft_count: u64 = nfts.len() as u64;
        let nft_id = Self::generate_nft_id(owner_id, nft_count);
        let nft = Nft::new(nft_id, name.clone(), image.clone());
        nft
    }

    pub fn add_nft_by_owner(owner_id: &T::AccountId, nft_id: HashOf<T>) {
        match NftsByOwner::<T>::get(owner_id) {
            None => {
                let mut nfts = Vec::<HashOf<T>>::new();
                nfts.push(nft_id.clone());
                NftsByOwner::<T>::insert(owner_id, nfts);
            },
            Some(mut nfts) => {
                nfts.push(nft_id.clone());
                NftsByOwner::<T>::insert(owner_id, nfts);
            }
        }
    }

    pub fn generate_nft_id(owner_id: &T::AccountId, nft_count: u64) -> <T as frame_system::Config>::Hash {
        let mut account_id_bytes = owner_id.encode();
        let mut nft_count_bytes = nft_count.encode();
        account_id_bytes.append(&mut nft_count_bytes);

        let seed = &account_id_bytes;
        T::Hashing::hash(seed)
    }
}
```