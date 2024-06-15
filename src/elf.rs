use object::{Object, ObjectSection};
use std::error::Error;
use std::fs;
use object::elf::*;
use object::read::elf::*;
use object::read::{SectionIndex, StringTable};

fn read(file_name: &str) -> Result<(), Box<dyn Error>> {
    let bin_data = fs::read(file_name)?;
    let obj_file: ElfFile64 = ElfFile::parse(&*bin_data)?;
    let endian = obj_file.endian();
    println!("endian {:?}", endian);

    if let Some(section) = obj_file.section_by_name(".text") {
        //println!("{:#x?}", String::from_utf8_lossy(section.data()?));
    } else {
        println!("section not available");
    }

    Ok(())
}


#[test]
fn test_read_elf() {
    read("./target/debug/diablo");
    assert_eq!(1, 2);
}

