#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated {
            who: T::AccountId,
            claim: T::Hash,
        },
        ClaimRevoked {
            who: T::AccountId,
            claim: T::Hash,
        },
        ClaimTransfered {
            from: T::AccountId,
            to: T::AccountId,
            claim: T::Hash,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        AlreadyClaimed,
        NoSuchClaim,
        NotClaimOwner,
    }

    #[pallet::storage]
    pub(super) type Claims<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, T::BlockNumber)>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn create_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(!Claims::<T>::contains_key(&claim), Error::<T>::AlreadyClaimed);

            let current_block = <frame_system::Pallet<T>>::block_number();

            Claims::<T>::insert(&claim, (&sender, current_block));

            Self::deposit_event(Event::ClaimCreated {
                who: sender,
                claim,
            });

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn revoke_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let (owner, _) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

            ensure!(sender == owner, Error::<T>::NotClaimOwner);

            Claims::<T>::remove(&claim);

            Self::deposit_event(Event::ClaimRevoked {
                who: sender,
                claim,
            });
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn transfer_claim(origin: OriginFor<T>, claim: T::Hash, to: T::AccountId) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let (owner, _) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

            ensure!(sender == owner, Error::<T>::NotClaimOwner);

            let block = Claims::<T>::get(&claim).unwrap().1;
            Claims::<T>::remove(&claim);
            Claims::<T>::insert(&claim, (&to, block));
            // Claims::<T>::mutate(&claim, |v| (&to, v.clone().unwrap().1));

            Self::deposit_event(Event::ClaimTransfered {
                from: sender,
                to,
                claim,
            });
            Ok(())
        }
    }
}