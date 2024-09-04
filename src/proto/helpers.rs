pub(crate) mod hex_to_string;
pub(crate) mod safe_vec;

// #[allow(unsafe_code)]
// fn get_stack_size() -> usize {
//     use std::arch::asm;
//     use std::thread;

//     let stack_top = thread::current().id().as_u64().get() as usize;
//     let stack_pointer: usize;
//     // x86_64
//     unsafe {
//         asm!("mov {}, rsp", out(reg) stack_pointer);
//     }

//     stack_top - stack_pointer
// }
