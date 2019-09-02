/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs
use rstd::prelude::*;
use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result, ensure};
use system::{self, ensure_signed};

/// The module's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

/// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		// 添加总量和余额映射
		pub TotalSupply get(total_supply): u64 = 21000000;
		pub BalanceOf get(balance_of): map T::AccountId => u64;

		Init get(is_init): bool;
	}
}

/// events
decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        // token令牌转账事件
        // from, to, value
        Transfer(AccountId, AccountId, u64),
    }
);

/// The module declaration.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // 初始化默认事件
        fn deposit_event<T>() = default;

		// 初始化token令牌，并将总供应量转给调用者
		fn init(origin) -> Result {
			let sender = ensure_signed(origin)?;
			ensure!(Self::is_init() == false, "Already initialized.");

			<BalanceOf<T>>::insert(sender, Self::total_supply());

            <Init<T>>::put(true);

			Ok(())
		}
		// 转账
		fn transfer(_origin, to: T::AccountId, value: u64) -> Result {
			let sender = ensure_signed(_origin)?;
			let sender_balance = Self::balance_of(sender.clone());
			ensure!(sender_balance >= value, "Not enough balance.");

			let updated_from_balance = sender_balance.checked_sub(value).ok_or("overflow in calculating balance")?;
			let receiver_balance = Self::balance_of(to.clone());
			let updated_to_balance = receiver_balance.checked_add(value).ok_or("overflow in calculating balance")?;

			// 发送者减少余额
			<BalanceOf<T>>::insert(sender.clone(), updated_from_balance);
			// 接受者增加余额
			<BalanceOf<T>>::insert(to.clone(), updated_to_balance);

			Self::deposit_event(RawEvent::Transfer(sender, to, value));

			Ok(())
		}
	}
}