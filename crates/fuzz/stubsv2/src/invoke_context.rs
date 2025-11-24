// new stubsV2 - invoke_context.rs
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::cell::RefCell;
use std::mem::transmute;

use solana_program_runtime::invoke_context::InvokeContext;

thread_local! {
    static INVOKE_CONTEXT: RefCell<Option<usize>> = const { RefCell::new(None) };
}
pub fn set_invoke_context(new: &mut InvokeContext) {
    INVOKE_CONTEXT.with(|invoke_context| unsafe {
        invoke_context.replace(Some(transmute::<&mut InvokeContext, usize>(new)))
    });
}
pub fn get_invoke_context<'a, 'b>() -> &'a mut InvokeContext<'b> {
    let ptr = INVOKE_CONTEXT.with(|invoke_context| match *invoke_context.borrow() {
        Some(val) => val,
        None => panic!("Invoke context not set!"),
    });
    unsafe { transmute::<usize, &mut InvokeContext>(ptr) }
}