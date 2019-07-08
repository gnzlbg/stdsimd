//! Runtime support needed for testing the stdsimd crate.
//!
//! This basically just disassembles the current executable and then parses the
//! output once globally and then provides the `assert` function which makes
//! assertions about the disassembly of a function.
#![feature(const_str_as_bytes)]
#![feature(const_transmute)]
#![allow(clippy::missing_docs_in_private_items, clippy::print_stdout)]

extern crate assert_instr_macro;
extern crate cc;
#[macro_use]
extern crate lazy_static;
extern crate rustc_demangle;
extern crate simd_test_macro;
#[macro_use]
extern crate cfg_if;

pub use assert_instr_macro::*;
pub use simd_test_macro::*;
use std::{collections::HashMap, env, str, sync::atomic::AtomicPtr};

// `println!` doesn't work on wasm32 right now, so shadow the compiler's `println!`
// macro with our own shim that redirects to `console.log`.
#[allow(unused)]
#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! println {
    ($($args:tt)*) => (wasm::js_console_log(&format!($($args)*)))
}

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        extern crate wasm_bindgen;
        extern crate console_error_panic_hook;
        pub mod wasm;
        use wasm::disassemble_myself;
    } else {
        mod disassembly;
        use disassembly::disassemble_myself;
    }
}

lazy_static! {
    static ref DISASSEMBLY: HashMap<String, Vec<Function>> = disassemble_myself();
}

#[derive(Debug)]
struct Function {
    addr: Option<usize>,
    instrs: Vec<Instruction>,
}

#[derive(Debug)]
struct Instruction {
    parts: Vec<String>,
}

fn normalize(mut symbol: &str) -> String {
    //eprintln!("before normalization: {}", symbol);
    // Remove trailing colon:
    if symbol.ends_with(':') {
      symbol = &symbol[..symbol.len() - 1];
    }
    //eprintln!("  after trailing colon removal: {}", symbol);
    let symbol = rustc_demangle::demangle(symbol).to_string();
    //eprintln!("  after demangling: {}", symbol);
    let mut ret = match symbol.rfind("::h") {
        Some(i) => symbol[..i].to_string(),
        None => symbol.to_string(),
    };
    //eprintln!("  after hash removal: {}", ret);

    // Remove Rust paths
    if let Some(last_colon) = ret.rfind(":") {
        ret = (&ret[last_colon+1..]).to_string();
    }
    //eprintln!("  after path removal: {}", ret);

    // Normalize to no leading underscore to handle platforms that may
    // inject extra ones in symbol names.
    while ret.starts_with('_') {
        ret.remove(0);
    }
    //eprintln!("  after normalization: {}", ret);
    ret
}


/// Main entry point for this crate, called by the `#[assert_instr]` macro.
///
/// This asserts that the function at `fnptr` contains the instruction
/// `expected` provided.
pub fn assert(_fnptr: usize, fnname: &str, expected: &str) {
    eprintln!("shim name: {}", fnname);
    let function = &DISASSEMBLY.get(&fnname.to_string()).unwrap()[0];
    eprintln!("  function: {:?}", function);

    let mut instrs = &function.instrs[..];
    while instrs.last().map_or(false, |s| s.parts == ["nop"]) {
        instrs = &instrs[..instrs.len() - 1];
    }

    // Look for `expected` as the first part of any instruction in this
    // function, returning if we do indeed find it.
    let mut found = false;
    for instr in instrs {
        // Get the first instruction, e.g., tzcntl in tzcntl %rax,%rax.
        if let Some(part) = instr.parts.get(0) {
            // Truncate the instruction with the length of the expected
            // instruction: tzcntl => tzcnt and compares that.
            if part.starts_with(expected) {
                found = true;
                break;
            }
        }
    }

    // Look for `call` instructions in the disassembly to detect whether
    // inlining failed: all intrinsics are `#[inline(always)]`, so
    // calling one intrinsic from another should not generate `call`
    // instructions.
    let mut inlining_failed = false;
    for (i, instr) in instrs.iter().enumerate() {
        let part = match instr.parts.get(0) {
            Some(part) => part,
            None => continue,
        };
        if !part.contains("call") {
            continue;
        }

        // On 32-bit x86 position independent code will call itself and be
        // immediately followed by a `pop` to learn about the current address.
        // Let's not take that into account when considering whether a function
        // failed inlining something.
        let followed_by_pop = function
            .instrs
            .get(i + 1)
            .and_then(|i| i.parts.get(0))
            .map_or(false, |s| s.contains("pop"));
        if followed_by_pop && cfg!(target_arch = "x86") {
            continue;
        }

        inlining_failed = true;
        break;
    }

    let instruction_limit = std::env::var("STDSIMD_ASSERT_INSTR_LIMIT")
        .ok()
        .map_or_else(
            || match expected {
                // `cpuid` returns a pretty big aggregate structure, so exempt
                // it from the slightly more restrictive 22 instructions below.
                "cpuid" => 30,

                // Apparently, on Windows, LLVM generates a bunch of
                // saves/restores of xmm registers around these intstructions,
                // which exceeds the limit of 20 below. As it seems dictated by
                // Windows's ABI (I believe?), we probably can't do much
                // about it.
                "vzeroall" | "vzeroupper" if cfg!(windows) => 30,

                // Intrinsics using `cvtpi2ps` are typically "composites" and
                // in some cases exceed the limit.
                "cvtpi2ps" => 25,

                // core_arch/src/acle/simd32
                "usad8" => 27,
                "qadd8" | "qsub8" | "sadd8" | "sel" | "shadd8" | "shsub8" | "usub8" | "ssub8" => 29,

                // Original limit was 20 instructions, but ARM DSP Intrinsics
                // are exactly 20 instructions long. So, bump the limit to 22
                // instead of adding here a long list of exceptions.
                _ => 22,
            },
            |v| v.parse().unwrap(),
        );
    let probably_only_one_instruction = instrs.len() < instruction_limit;

    if found && probably_only_one_instruction && !inlining_failed {
        return;
    }

    // Help debug by printing out the found disassembly, and then panic as we
    // didn't find the instruction.
    println!("disassembly for {}: ", fnname,);
    for (i, instr) in instrs.iter().enumerate() {
        let mut s = format!("\t{:2}: ", i);
        for part in &instr.parts {
            s.push_str(part);
            s.push_str(" ");
        }
        println!("{}", s);
    }

    if !found {
        panic!(
            "failed to find instruction `{}` in the disassembly",
            expected
        );
    } else if !probably_only_one_instruction {
        panic!(
            "instruction found, but the disassembly contains too many \
             instructions: #instructions = {} >= {} (limit)",
            instrs.len(),
            instruction_limit
        );
    } else if inlining_failed {
        panic!(
            "instruction found, but the disassembly contains `call` \
             instructions, which hint that inlining failed"
        );
    }
}

pub fn assert_skip_test_ok(name: &str) {
    if env::var("STDSIMD_TEST_EVERYTHING").is_err() {
        return;
    }
    panic!("skipped test `{}` when it shouldn't be skipped", name);
}

// See comment in `assert-instr-macro` crate for why this exists
pub static _DONT_DEDUP: AtomicPtr<u8>
    = AtomicPtr::new(unsafe { std::mem::transmute("".as_bytes().as_ptr()) });
