#![cfg_attr(not(feature = "std"), no_std)]

// 编译标签
/// A module for proof of existence

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    pub type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32,
        (T::AccountId, T::BlockNumber)
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, u32),
        ClaimRevoked(T::AccountId, u32),
        ClaimTransfered(T::AccountId, T::AccountId, u32),
    }

    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyClaimed,
        NoSuchProof,
        ClaimNotExist,
        NotClaimOwner,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn create_claim(
            origin: OriginFor<T>,
            claim: u32,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyClaimed);
            let current_block = <frame_system::Pallet<T>>::block_number();
            Proofs::<T>::insert(&claim, (&sender, current_block));

            Self::deposit_event(Event::ClaimCreated(sender, claim));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            claim: u32,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::NoSuchProof);
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
            ensure!(owner == sender, Error::<T>::NotClaimOwner);
            Proofs::<T>::remove(&claim);

            Self::deposit_event(Event::ClaimRevoked(sender, claim));
            Ok(())
        }

        #[pallet::weight(1_000)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            receiver: T::AccountId,
            proof: u32,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
            let (owner, _) = Proofs::<T>::get(&proof).ok_or(Error::<T>::ClaimNotExist)?;
            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            let current_block = <frame_system::Pallet<T>>::block_number();
            Proofs::<T>::mutate(&proof, |value| {
                value.as_mut().unwrap().0 = receiver.clone();
                value.as_mut().unwrap().1 = current_block;
            });

            Self::deposit_event(Event::ClaimTransfered(sender, receiver, proof));
            Ok(())
        }
    }
}
