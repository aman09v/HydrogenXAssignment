#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	type CustomerOf<T> = Customer<<T as frame_system::Config>::Hash>;
	// SCustomer is a struct for Customer
	#[derive(Encode, Decode, Copy, Clone, Default, TypeInfo, MaxEncodedLen, PartialEq, RuntimeDebug)]
	pub struct Customer<Hash> {
		pub id: u32,
		pub name: Hash,
		pub location: (u32, u32),
	}
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
	#[pallet::storage]
	#[pallet::getter(fn get_customer)]
	pub type CustomerMap<T: Config> = StorageMap<_, Blake2_128Concat, u32, CustomerOf<T>>;

	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// event emitted when customer is added.
		CustomerAdded(u32, T::AccountId),
		// event emitted when customers's location is updated.
		CustomerLocationUpdated(T::AccountId, u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		// Error emitted when Customer's id is already present in Customer StorageMap.
		CustomerAlreadyExist,

		StorageOverflow,

		CustomerDoesNotExist,

	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_new_customer(
			origin: OriginFor<T>,
			cust_id: u32,
			new_cust: CustomerOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			match <CustomerMap<T>>::get(cust_id) {
				Some(_) => Err(Error::<T>::CustomerAlreadyExist)?,
				None => {
					<CustomerMap<T>>::insert(cust_id, new_cust);
				}
			}
			// Emit an event.
			Self::deposit_event(Event::CustomerAdded(cust_id, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn update_customer_location(
			origin: OriginFor<T>,
			cust_id: u32,
			location: (u32, u32),
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin

			let who = ensure_signed(origin)?;
			ensure!(
                <CustomerMap<T>>::contains_key(&cust_id),
                Error::<T>::CustomerDoesNotExist
            );
			let cust_option = <CustomerMap<T>>::get(&cust_id);
			if let Some(mut customer) = cust_option {
				customer.location.0 = location.0;
				customer.location.1 = location.1;
				<CustomerMap<T>>::insert(&cust_id, customer);
				Self::deposit_event(Event::CustomerLocationUpdated(who, customer.id));
			}
			Ok(().into())
		}
	}
}
