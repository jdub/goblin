//! The ELF 64-bit struct definitions and associated values

#[path="header64.rs"]
pub mod header;
#[path="sym64.rs"]
pub mod sym;
#[path="program_header64.rs"]
pub mod program_header;
#[path="section_header64.rs"]
pub mod section_header;
#[path="dyn64.rs"]
pub mod dyn;
#[path="reloc64.rs"]
pub mod reloc;

#[cfg(feature = "std")]
pub use elf::strtab;

#[cfg(feature = "std")]
pub mod gnu_hash {
    elf_gnu_hash_impl!();
}
