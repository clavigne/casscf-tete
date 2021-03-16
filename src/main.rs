use clap::{App, Arg};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::iter::FromIterator;

fn main() {
    let matches = App::new("casse-tete")
        .version("1.0")
        .author("Cyrille Lavigne <cyrille.lavigne@mail.utoronto.ca>")
        .about("Generate CAS-SCF parameters for US-GAMESS without all the headaches.")
        .arg(
            Arg::new("active_el")
                .value_name("CAS_EL")
                .about("Number of electrons in the CAS space.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("homo")
                .long("homo")
                .value_name("IORBS")
                .multiple(true)
                .about("Indices of HOMO orbitals.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("lumo")
                .long("lumo")
                .value_name("IORBS")
                .multiple(true)
                .about("Indices of LUMO orbitals.")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    // Get the orbital indices
    let homos = BTreeSet::from_iter(
        matches
            .values_of_t::<usize>("homo")
            .unwrap_or_else(|e| e.exit()),
    );

    // Get the orbital indices
    let lumos = BTreeSet::from_iter(
        matches
            .values_of_t::<usize>("lumo")
            .unwrap_or_else(|e| e.exit()),
    );

    // TODO: warn about overlapping spaces
    // if homos.max() < lumos.min() {
    //     panic!("overlapping homo and lumo spaces are not supported!")
    // }

    let nactive = homos.len() + lumos.len();
    let homo = homos.iter().max();

    let start = 1;

    let mut iorder = BTreeMap::new();
    for k in 0..nactive {
        iorder.insert(start + k, start + k);
    }

    // Filter out any orbitals already in the set
    let curr_iorbs: Vec<usize> = homos
        .union(&lumos)
        .filter(|x| !iorder.contains_key(x))
        .cloned()
        .collect();

    let mut last_free: usize = 0;
    for element in curr_iorbs.iter() {
        if let Some(free) = iorder
            .keys()
            .filter(|x| x > &&last_free)
            .filter(|x| !homos.contains(x))
            .filter(|x| !lumos.contains(x))
            .next()
            .cloned()
        {
            last_free = free;
            iorder.insert(free, *element);
        } else {
            panic!(
                "Problem! Ran out of free spots to insert new orbitals.\n \
                 This should never happen."
            )
        };
    }
    println!("{:?}", iorder);
}
