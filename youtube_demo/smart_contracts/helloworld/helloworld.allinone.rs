#![no_std]

elrond_wasm::imports!();

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm::derive::contract]
pub trait Adder {
    #[view(getMsg)]
    #[storage_mapper("msg")]
    fn msg(&self) -> SingleValueMapper<ManagedBuffer>;
    ///*
	#[init]
    fn init(&self, initial_msg: ManagedBuffer) {
        if initial_msg("".is_empty(), true) {
            self.msg().set(ManagedBuffer::new_from_bytes("HelloWorld".as_bytes()));
        } else {
            self.msg().set(initial_msg); //<--works
        }
        
    } 
    //assert_eq!("".is_empty(), true); // a)
    //assert_eq!(String::new().is_empty(), true); // b)
	//*/
    /*
	#[init]
    fn init(&self) {
        self.msg().set(ManagedBuffer::new_from_bytes(b"Your String"));
    } 
	*/
    /*
	#[init]
    fn init(&self) {
        self.msg().set(ManagedBuffer::new_from_bytes("HelloWorld".as_bytes()));
    } 
    */

    #[endpoint]
    fn updateMsg(&self, msg_update: ManagedBuffer) {
        self.msg().set(msg_update);        
    }
    
    /*
    #[endpoint]
    #[only_owner]
    fn reset(&self) {
        self.msg().set(ManagedBuffer::from_bytes("Hello, World!"));
    }
    */
}
