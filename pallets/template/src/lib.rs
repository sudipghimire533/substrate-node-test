#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

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

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_first_storage)]
	pub type FirstStorage<T> = StorageValue<_, i32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_second_storage)]
	pub type SecondStorage<T> = StorageValue<_, i32, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Nothing(),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// ??
		Null,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn change_both_failing(_origin: OriginFor<T>, value: i32) -> DispatchResult {
			// Changing first went fine
			<FirstStorage<T>>::set(value);

			// Some error occured at this point
			if true {
				return Err("Nasty eror..".into()).into();
			}

			// We intended to change second storage too
			<SecondStorage<T>>::set(value);

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn change_both_passing(_origin: OriginFor<T>, val: i32) -> DispatchResult {
			<FirstStorage<T>>::set(val);
			<SecondStorage<T>>::set(val);

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn print_both(_origin: OriginFor<T>, value: i32) -> DispatchResult {
			log::info!("\n\n===========================");
			log::info!("[FirstStorage] {}", <FirstStorage<T>>::get());
			log::info!("[SecondStorage] {}", <SecondStorage<T>>::get());
			log::info!("\n\n===========================");


			Ok(())
		}
	}
}
