use std::fs::File;
use std::io::prelude::*;

#[allow(non_snake_case)]
const fn KiB(n: usize) -> usize {
    n * 1024
}
#[allow(non_snake_case)]
const fn MiB(n: usize) -> usize {
    n * KiB(1024)
}
const fn align(n: usize) -> usize {
    (n + 3) & !3
}

const WORKER_NAME_VAR: &'static str = "CRATE";

/// Reserved memory
const MAX_MEORY: usize = MiB(512);

/// The first page of memory is reserved
const STACK_ALIGNMENT: usize = 1024 * 63;

/// The size of the stack. Its start is at address 0
const GRAPHICS_STACK_SIZE: usize = MiB(4);
const GRAPHICS_HEAP_SIZE: usize = MiB(1);

/// The size of the Allocator structures
const ALLOCATOR_SIZE: usize = MiB(1);

/// Size of the internal resource library.
/// This determines the highest available id.
const RESOURCE_TABLE_SIZE: usize = KiB(1);

/// Size of the message queue used to communicate between main.js and the logic thread
/// Its address must be exorted to javascript.
const MESSAGE_QUEUE_SIZE: usize = 64;

/// The address memory synchronization area.
/// It contains data needed for synchronization between main thread and logic thread.
/// This address must be exorted to javascript.
const SYNCHRONIZATION_MEMORY_SIZE: usize = 32;

/// Number of sprites to store in the double buffer
const BUFFER_SIZE: usize = KiB(1);

fn main() -> std::io::Result<()> {
    println!("{:#?}", std::env::vars().collect::<Vec<_>>());
    let name = std::env::var(WORKER_NAME_VAR);
    let is_logic = match name {
        Ok(worker) if &worker == "logic" => true,
        Ok(worker) if &worker == "graphics" => false,
        Ok(key) => panic!(
            "{} is no valid value. Possibel values are logic and graphics",
            key
        ),
        Err(std::env::VarError::NotPresent) => {
            panic!("{} is not defined in the environment.", WORKER_NAME_VAR)
        }
        Err(err) => panic!("env var parsing failed (\"{:?}\")", err),
    };

    let graphics_stack = align(STACK_ALIGNMENT + GRAPHICS_STACK_SIZE);
    let alloc = align(graphics_stack);
    let graphics_heap = align(alloc + ALLOCATOR_SIZE);
    let sync = align(alloc + GRAPHICS_HEAP_SIZE);
    let table = align(sync + SYNCHRONIZATION_MEMORY_SIZE);
    let buffer = align(table + RESOURCE_TABLE_SIZE);
    let queue = align(buffer + BUFFER_SIZE);
    let logic_heap = align(queue + MESSAGE_QUEUE_SIZE);

    println!("cargo:rustc-env=GRAPHICS_STACK={}", graphics_stack);
    println!("cargo:rustc-env=ALLOCATOR={}", alloc);
    println!("cargo:rustc-env=GRAPHICS_HEAP={}", graphics_heap);
    println!("cargo:rustc-env=SYNCHRONIZATION_MEMORY={}", sync);
    println!("cargo:rustc-env=RESOURCE_TABLE={}", table);
    println!("cargo:rustc-env=RESOURCE_TABLE_SIZE={}", buffer - table);
    println!("cargo:rustc-env=DOUBLE_BUFFER={}", buffer);
    println!("cargo:rustc-env=DOUBLE_BUFFER_SIZE={}", queue - buffer);
    println!("cargo:rustc-env=MESSAGE_QUEUE={}", queue);
    println!("cargo:rustc-env=MESSAGE_QUEUE_SIZE={}", logic_heap - queue);
    println!("cargo:rustc-env=LOGIC_HEAP={}", logic_heap);

    if !is_logic {
        println!("cargo:rustc-cdylib-link-arg=--stack-first");
        println!(
            "cargo:rustc-cdylib-link-arg=-zstack-size={}",
            graphics_stack
        );
    };
    println!("cargo:rustc-cdylib-link-arg=--max-memory={}", MAX_MEORY);

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut file = File::create(format!("{}/mem.json", out_dir))?;
    file.write_all(
        format!(
            "{{max_memory:{},queue_start:{},sync_area:{}}}",
            MAX_MEORY, queue, sync
        )
        .as_bytes(),
    )?;
    Ok(())
}