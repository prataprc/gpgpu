use vk_parse::{Registry, RegistryChild};

use crate::{Error, Result};

#[macro_export]
macro_rules! get_registry_variant {
    ($reg:ident, $variant:ident, $vtype:ty) => {
        $reg.0
            .iter()
            .filter_map(|item| match item {
                RegistryChild::$variant(val) => Some(val.clone()),
                _ => None,
            })
            .collect::<Vec<CommentedChildren<$vtype>>>()
    };
}

pub fn get_registry() -> Result<Registry> {
    let data = include_bytes!("../vk.xml").to_vec();
    // TODO: add Display trait for vk_parse::{FatalError, Error}, then we don't need this
    // match expression
    match vk_parse::parse_stream(data.as_slice()) {
        Err(err) => err_at!(Vk, msg: "{:?}", err),
        Ok((reg, errs)) if errs.is_empty() => Ok(reg),
        Ok((_, errs)) => {
            let ss: Vec<String> = errs.into_iter().map(|e| format!("{:?}", e)).collect();
            err_at!(Vk, msg: "{}", ss.as_slice().join("\n"))?
        }
    }
}

pub fn front_page(reg: &Registry) {
    let mut counts: [(usize, usize); 11] = [(0_usize, 0_usize); 11];

    for item in reg.0.iter() {
        let (off, children) = match item {
            RegistryChild::Comment(_) => (0, 1),
            RegistryChild::VendorIds(val) => (1, val.children.len()),
            RegistryChild::Platforms(val) => (2, val.children.len()),
            RegistryChild::Tags(val) => (3, val.children.len()),
            RegistryChild::Types(val) => (4, val.children.len()),
            RegistryChild::Enums(val) => (5, val.children.len()),
            RegistryChild::Commands(val) => (6, val.children.len()),
            RegistryChild::Feature(val) => (7, val.children.len()),
            RegistryChild::Extensions(val) => (8, val.children.len()),
            RegistryChild::SpirvExtensions(val) => (9, val.children.len()),
            RegistryChild::SpirvCapabilities(val) => (10, val.children.len()),
            _ => panic!("non-exhaustive patter for RegistryChild"),
        };
        counts[off].0 += 1;
        counts[off].1 += children;
    }

    println!(
        "Number of Comment             : {}, {}",
        counts[0].0, counts[0].1
    );
    println!(
        "Number of VendorIds           : {}, {}",
        counts[1].0, counts[1].1
    );
    println!(
        "Number of Platforms           : {}, {}",
        counts[2].0, counts[2].1
    );
    println!(
        "Number of Tags                : {}, {}",
        counts[3].0, counts[3].1
    );
    println!(
        "Number of Types               : {}, {}",
        counts[4].0, counts[4].1
    );
    println!(
        "Number of Enums               : {}, {}",
        counts[5].0, counts[5].1
    );
    println!(
        "Number of Commands            : {}, {}",
        counts[6].0, counts[6].1
    );
    println!(
        "Number of Feature             : {}, {}",
        counts[7].0, counts[7].1
    );
    println!(
        "Number of Extensions          : {}, {}",
        counts[8].0, counts[8].1
    );
    println!(
        "Number of SpirvExtensions     : {}, {}",
        counts[9].0, counts[9].1
    );
    println!(
        "Number of SpirvCapabilities   : {}, {}",
        counts[10].0, counts[10].1
    );
}
