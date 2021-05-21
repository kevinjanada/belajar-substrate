#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_support::sp_runtime::traits::Hash;
	use sp_std::prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	#[pallet::storage]
	#[pallet::getter(fn nfts_by_owner)]
	pub(super) type NftsByOwner<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<HashOf<T>>>;

	#[pallet::storage]
	#[pallet::getter(fn nft_by_id)]
	pub(super) type Nfts<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, Nft<HashOf<T>, AccountIdOf<T>>>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [Nft]
		NftCreated(Nft<T::Hash, T::AccountId>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn mint_nft(origin: OriginFor<T>, name: Vec<u8>, image: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let nft = Self::create_nft(&who, &name, &image);
			Self::deposit_event(Event::NftCreated(nft));
			Ok(())
		}
	}

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
	pub struct Nft<Hash, AccountId> {
		id: Hash, // H256
		owner_id: AccountId,
		name: Vec<u8>, // Text
		image: Vec<u8>, // Text
	}
	impl<Hash, AccountId> Nft<Hash, AccountId> {
		pub fn new(id: Hash, owner_id: AccountId, name: Vec<u8>, image: Vec<u8>) -> Self {
			Self { id, owner_id, name, image }
		}
	}

	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	impl<T: Config> Pallet<T> {
		pub fn create_nft(owner_id: &T::AccountId, name: &Vec<u8>, image: &Vec<u8>) -> Nft<HashOf<T>, AccountIdOf<T>> {
			let nfts = Self::nfts_by_owner(owner_id).unwrap_or(Vec::new());
			let nft_count: u64 = nfts.len() as u64;

			let nft_id = Self::generate_nft_id(owner_id, nft_count);

			let nft = Nft::new(nft_id.clone(), owner_id.clone(), name.clone(), image.clone());
			Nfts::<T>::insert(nft_id.clone(), nft.clone());
			Self::add_nft_by_owner(owner_id, nft_id.clone());

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
}
