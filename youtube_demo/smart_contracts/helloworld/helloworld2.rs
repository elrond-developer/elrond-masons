#![no_std]

elrond_wasm::imports!();

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm::derive::contract]
pub trait HelloWorld {
    #[view(getMsg)]
    #[storage_mapper("msg")]
    fn msg(&self) -> SingleValueMapper<ManagedBuffer>;

	#[init]
    fn init(&self) {
        self.msg().set(ManagedBuffer::new_from_bytes(b"Hello, World!"));
    } 

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn updateMsg(&self, msg_update: ManagedBuffer) {
        self.msg().set(msg_update);
    }
}
