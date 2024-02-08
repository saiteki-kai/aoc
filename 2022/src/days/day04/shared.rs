use std::collections::HashSet;

fn get_sections(minmax: &str) -> HashSet<u8> {
    let mm = minmax.split('-').collect::<Vec<&str>>();

    let min: u8 = mm[0].parse().unwrap();
    let max: u8 = mm[1].parse().unwrap();
    let range = min..=max;

    range.collect::<HashSet<u8>>()
}

pub enum OverlapType {
    Full,
    Partial,
}

pub fn find_overlap(pair: &str, overlap_type: OverlapType) -> bool {
    let elf_sections = pair.split(',').collect::<Vec<&str>>();

    let elf1_sections = get_sections(elf_sections[0]);
    let elf2_sections = get_sections(elf_sections[1]);

    match overlap_type {
        OverlapType::Partial => !elf1_sections.is_disjoint(&elf2_sections),
        OverlapType::Full => {
            elf1_sections.is_subset(&elf2_sections) || elf2_sections.is_subset(&elf1_sections)
        }
    }
}
