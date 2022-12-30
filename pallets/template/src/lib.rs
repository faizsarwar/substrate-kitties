#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
mod types;
pub use types::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;



#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::traits::{Randomness,Currency,ExistenceRequirement};
	use frame_support::transactional;
	use sp_core::H256;
	use frame_support::{
		sp_runtime::traits::Hash
	};
	use frame_support::sp_runtime::traits::Zero;
	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> =
	  <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
		// Struct for holding Kitty information.
	#[derive(Clone, Encode, Decode, Default, PartialEq,MaxEncodedLen,TypeInfo)]
	pub struct Kitty<Hash, Balance> {
		id: Hash,
		dna: Hash,
		price: Balance,
		gender: Gender,
	}

	#[derive(Encode, Decode, Debug, Clone, PartialEq,MaxEncodedLen,TypeInfo)]
	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender {
		fn default() -> Self {
			Gender::Male
		}
	}


	impl<T: Config> Kitty<T, T> {
		pub fn gender(dna: T::Hash) -> Gender {
			if dna.as_ref()[0] % 2 == 0 {
				Gender::Male
			} else {
				Gender::Female
			}
		}
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: pallet_balances::Config + frame_system::Config  {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type KittyRandomness: Randomness<H256, u32>;
		/// The Currency handler for the Kitties pallet.
		type Currency: Currency<Self::AccountId>;

	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	
	#[pallet::storage]
    #[pallet::getter(fn all_kitties_count)]
    pub(super) type AllKittiesCount<T: Config> = StorageValue<
        _, 
        u64, 
        ValueQuery
        >;

	#[pallet::storage]
    #[pallet::getter(fn get_nonce)]
    pub(super) type Nonce<T: Config> = StorageValue<
        _, 
        u64, 
        ValueQuery
        >;
	
	#[pallet::storage]
	#[pallet::getter(fn kitty)]
	pub(super) type Kitties<T: Config> = StorageMap<
		_, 
		Twox64Concat, 
		T::Hash, 
		Kitty<T::Hash, T::Balance>, 
		ValueQuery
		>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_array)]
	pub(super) type AllKittiesArray<T: Config> = StorageMap<
		_, 
		Twox64Concat, 
		u64, 
		T::Hash, 
		ValueQuery,
		>;
	
	#[pallet::storage]
	#[pallet::getter(fn owned_kitty_count)]
	pub(super) type OwnedKittiesCount<T: Config> = StorageMap<
		_, 
		Twox64Concat, 
		T::AccountId, 
		u32, 
		ValueQuery,
		>;
		
	
	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub(super) type KittyOwner<T: Config> = StorageMap<
		_, 
		Twox64Concat, 
		T::Hash, 
		Option<T::AccountId>, 
		ValueQuery,
		>;
		
	
	
	#[pallet::storage]
	#[pallet::getter(fn all_kitties_index)]
	pub(super) type AllKittiesIndex<T: Config> = StorageMap<
		_, 
		Twox64Concat, 
		T::Hash, 
		u64, 
		ValueQuery,
		>;
		
	#[pallet::storage]
	#[pallet::getter(fn owned_kitties_index)]
	pub(super) type OwnedKittiesIndex<T: Config> = StorageMap<
		_, 
		Twox64Concat, 
		T::Hash, 
		u32, 
		ValueQuery,
		>;
		
	#[pallet::storage]
	#[pallet::getter(fn owned_kitties_array)]
	pub(super) type OwnedKittiesArray<T: Config> = StorageMap<
		_, 
		Twox64Concat, 
		(T::AccountId,u32), 
		T::Hash, 
		ValueQuery,
		>;
		
		// Our pallet's genesis configuration.
		#[pallet::genesis_config]
		pub struct GenesisConfig<T: Config> {
			pub kitties: Vec<(T::AccountId, T::Hash, T::Balance)>,
		}

		// Required to implement default for GenesisConfig.
		#[cfg(feature = "std")]
		impl<T: Config> Default for GenesisConfig<T> {
			fn default() -> GenesisConfig<T> {
				GenesisConfig { kitties: vec![] }
			}
		}

		#[pallet::genesis_build]
		impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
			fn build(&self) {
				for &(ref acct, hash, balance) in &self.kitties {
					let k = Kitty {
						id: hash,
						dna: hash,
						price: balance,
						gender: Gender::Male,
					};

					let _ = <Pallet<T>>::mint(acct.clone(), hash, k);
				}
			}
		}
	

		// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
		Created(T::AccountId, T::Hash),
		PriceSet(T::AccountId, T::Hash, T::Balance),
		Transferred(T::AccountId, T::AccountId, T::Hash),
		Bought(T::AccountId, T::AccountId, T::Hash, T::Balance),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		KittyNotExist,
		NotKittyOwner,
		BuyerIsKittyOwner,
		NonceOverflow
	}


	impl<T: Config> Pallet<T> {
		fn increment_nonce() -> DispatchResult {
			<Nonce<T>>::try_mutate(|nonce| {
				let next = nonce.checked_add(1).ok_or(Error::<T>::NonceOverflow)?;
				*nonce = next;
				Ok(().into())
			})
		}

		// Helper to handle transferring a Kitty from one account to another.
		fn transfer_from(
			from: T::AccountId,
			to: T::AccountId,
			kitty_id: T::Hash,
		) -> DispatchResult {
			// Verify that the owner is the rightful owner of this Kitty.
			let owner = KittyOwner::<T>::get(kitty_id).ok_or("No owner for this kitty")?;
			ensure!(owner == from, "'from' account does not own this kitty");

			// Address to send from.
			let owned_kitty_count_from = Self::owned_kitty_count(&from);

			// Address to send to.
			let owned_kitty_count_to = Self::owned_kitty_count(&to);

			// Increment the amount of owned Kitties by 1.
			let new_owned_kitty_count_to = owned_kitty_count_to
				.checked_add(1)
				.ok_or("Transfer causes overflow of 'to' kitty balance")?;

			// Increment the amount of owned Kitties by 1.
			let new_owned_kitty_count_from = owned_kitty_count_from
				.checked_sub(1)
				.ok_or("Transfer causes underflow of 'from' kitty balance")?;

			// Get current Kitty index.
			let kitty_index = <OwnedKittiesIndex<T>>::get(kitty_id);

			// Update storage items that require updated index.
			if kitty_index != new_owned_kitty_count_from {
				let last_kitty_id =
					<OwnedKittiesArray<T>>::get((from.clone(), new_owned_kitty_count_from));
				<OwnedKittiesArray<T>>::insert((from.clone(), kitty_index), last_kitty_id);
				<OwnedKittiesIndex<T>>::insert(last_kitty_id, kitty_index);
			}

			// Write new Kitty ownership to storage items.
			<KittyOwner<T>>::insert(&kitty_id, Some(&to));
			<OwnedKittiesIndex<T>>::insert(kitty_id, owned_kitty_count_to);

			<OwnedKittiesArray<T>>::remove((from.clone(), new_owned_kitty_count_from));
			<OwnedKittiesArray<T>>::insert((to.clone(), owned_kitty_count_to), kitty_id);

			<OwnedKittiesCount<T>>::insert(&from, new_owned_kitty_count_from);
			<OwnedKittiesCount<T>>::insert(&to, new_owned_kitty_count_to);

			Self::deposit_event(Event::Transferred(from, to, kitty_id));

			Ok(())
		}

		// Helper to mint a Kitty.
		fn mint(
			to: T::AccountId,
			kitty_id: T::Hash,
			new_kitty: Kitty<T::Hash, T::Balance>,
		) -> DispatchResult {
				ensure!(
					!<KittyOwner<T>>::contains_key(kitty_id),
					"Kitty already contains_key"
				);
				// Update total Kitty counts.
				let owned_kitty_count = Self::owned_kitty_count(&to);
				let new_owned_kitty_count = owned_kitty_count
					.checked_add(1)
					.ok_or("Overflow adding a new kitty to account balance")?;

				let all_kitties_count = Self::all_kitties_count();
				let new_all_kitties_count = all_kitties_count
					.checked_add(1)
					.ok_or("Overflow adding a new kitty to total supply")?;

				// Update storage with new Kitty.
				<Kitties<T>>::insert(kitty_id, new_kitty);
				<KittyOwner<T>>::insert(kitty_id, Some(&to));

				// Write Kitty counting information to storage.
				<AllKittiesArray<T>>::insert(new_all_kitties_count, kitty_id);
				<AllKittiesCount<T>>::put(new_all_kitties_count);
				<AllKittiesIndex<T>>::insert(kitty_id, new_all_kitties_count);

				// Write Kitty counting information to storage.
				<OwnedKittiesArray<T>>::insert((to.clone(), new_owned_kitty_count), kitty_id);
				<OwnedKittiesCount<T>>::insert(&to, new_owned_kitty_count);
				<OwnedKittiesIndex<T>>::insert(kitty_id, new_owned_kitty_count);
				Self::deposit_event(Event::Created(to, kitty_id));
				Ok(())
			}


		fn random_hash(sender: &T::AccountId) -> T::Hash {
			let nonce = <Nonce<T>>::get();
			let seed = T::KittyRandomness::random_seed();

			T::Hashing::hash_of(&(seed, &sender, nonce))
		}
		
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {


		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Set the price for a Kitty.
		///
		/// Updates Kitty price and updates storage.
		#[pallet::weight(100)]
		pub fn set_price(
		origin: OriginFor<T>,
		kitty_id: T::Hash,
		new_price: Option<T::Balance>
		) -> DispatchResult {
		let sender = ensure_signed(origin)?;

		// ACTION #1: Checking Kitty owner
		let owner = KittyOwner::<T>::get(kitty_id).ok_or("No owner for this object")?;

		ensure!(owner == sender, "You are not the owner");

		ensure!(Kitties::<T>::contains_key(&kitty_id),<Error<T>>::KittyNotExist);
		let mut kitty = Kitties::<T>::get(&kitty_id);

		// ACTION #2: Set the Kitty price and update new Kitty infomation to storage.
		// Set the Kitty price.
		let mut kitty = Self::kitty(kitty_id);
		kitty.price = new_price.unwrap();

		// Update new Kitty infomation to storage.
		<Kitties<T>>::insert(kitty_id, kitty);

		// ACTION #3: Deposit a "PriceSet" event.
		Self::deposit_event(Event::PriceSet(sender, kitty_id, new_price.unwrap()));

		Ok(())
		}


		/// Breed a Kitty.
		/// Breed two kitties to create a new generation
		/// of Kitties.
		#[pallet::weight(100)]
		pub fn breed_kitty(
		origin: OriginFor<T>,
		kid1: T::Hash,
		kid2: T::Hash
		) -> DispatchResult {
		let sender = ensure_signed(origin)?;
		let owner_of_kid1 = KittyOwner::<T>::get(&kid1).ok_or("No owner for this object")?;
		let owner_of_kid2 = KittyOwner::<T>::get(&kid2).ok_or("No owner for this object")?;
		// Check: Verify `sender` owns both kitties (and both kitties exist).
		ensure!(owner_of_kid1 == sender, <Error<T>>::NotKittyOwner);
		ensure!(owner_of_kid2 == sender, <Error<T>>::NotKittyOwner);

		// ACTION #10: Breed two Kitties using unique DNA
		let random_hash = Self::random_hash(&sender);
		let kitty_1 = Kitties::<T>::get(&kid1);
		let kitty_2 = Kitties::<T>::get(&kid1);
	
		let mut final_dna = kitty_1.dna;
		for (i, (dna_2_element, r)) in kitty_2
			.dna
			.as_ref()
			.iter()
			.zip(random_hash.as_ref().iter())
			.enumerate()
		{
			if r % 2 == 0 {
				final_dna.as_mut()[i] = *dna_2_element;
			}
		}
		// ACTION #11: Mint new Kitty using new DNA
		let new_kitty = Kitty {
			id: random_hash,
			dna: final_dna,
			price: 0u8.into(),
			gender: Kitty::<T, T>::gender(final_dna),
		};
		
		Self::mint(sender, random_hash, new_kitty)?;
		Self::increment_nonce()?;

		Ok(())
		}
		

		#[pallet::weight(100)]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			kitty_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			// Verify Kitty owner: must be the account invoking this transaction.
			let owner = KittyOwner::<T>::get(kitty_id).ok_or("No owner for this object")?;
			ensure!(owner == sender, "You do not own this kitty");

			// Transfer.
			Self::transfer_from(sender, to, kitty_id)?;


			Ok(().into())
		}

		// buy_kitty
		#[transactional]
		#[pallet::weight(100)]
		pub fn buy_kitty(
		  origin: OriginFor<T>,
		  kitty_id: T::Hash,
		  bid_price: T::Balance
		) -> DispatchResult {
		  let buyer = ensure_signed(origin)?;
	
		  // Check the kitty exists and buyer is not the current kitty owner
		  let mut kitty = Self::kitty(&kitty_id);
		  let owner = KittyOwner::<T>::get(kitty_id);

		  ensure!(owner != Some(buyer.clone()), <Error<T>>::BuyerIsKittyOwner);
	
		  // ACTION #7: Check if the Kitty is for sale.
		  // Check if the Kitty is for sale.
		  ensure!(!kitty.price.is_zero(), "This Kitty is not for sale!");
		  ensure!(
		  	kitty.price <= bid_price,
		  	"This Kitty is out of your budget!"
		  );
	
		  // ACTION #8: Update Balances using the Currency trait.
		  <pallet_balances::Pallet<T> as Currency<_>>::transfer(
				&buyer.clone(),
				&owner.clone().unwrap(),
				kitty.price,
				ExistenceRequirement::KeepAlive,
			)?;

			// Transfer ownership of Kitty.
			Self::transfer_from(owner.unwrap(), buyer.clone(), kitty_id).expect(
				"`owner` is shown to own the kitty; \
				`owner` must have greater than 0 kitties, so transfer cannot cause underflow; \
				`all_kitty_count` shares the same type as `owned_kitty_count` \
				and minting ensure there won't ever be more than `max()` kitties, \
				which means transfer cannot cause an overflow; \
				qed",
			);

			// Set the price of the Kitty to the new price it was sold at.
			kitty.price = bid_price.into();
			<Kitties<T>>::insert(kitty_id, kitty);

		  Ok(())
		}


		#[pallet::weight(100)]
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let random_hash = Self::random_hash(&sender);

			let new_kitty = Kitty {
				id: random_hash,
				dna: random_hash,
				price: 0u8.into(),
				gender: Kitty::<T, T>::gender(random_hash),
			};

			Self::mint(sender, random_hash, new_kitty)?;
			Self::increment_nonce()?;

			Ok(().into())
		}


	}
}


// uncomment kitties