use clap::{crate_version, App, Arg};
use std::collections::{BTreeMap, BTreeSet};
use std::iter::FromIterator;

fn main() {
    let matches = App::new("casscf-tete")
        .author("Cyrille Lavigne <cyrille.lavigne@mail.utoronto.ca>")
        .about(
            "\
Generate CAS-SCF parameters for US-GAMESS without all the headaches.\n\
\n\
This program generates the $guess and $det groups for a US-GAMESS CAS-SCF\n\
calculation. For example,\n\n\
\tcasscf-tete 68 --homo 28 31 32 33 34 --lumo 35 36 38 43 45 \n\n\
prints the following\n\n\
\t $guess iorder(28)=30 iorder(30)=28 iorder(37)=43 iorder(39)=45 $end\n\
\t $guess iorder(43)=37 iorder(45)=39 $end\n\
\t $det ncore=29 nact=10 nels=10 $end\n",
        )
        .version(crate_version!())
        .arg(
            Arg::new("nel")
                .value_name("NEL")
                .about("Total number of electrons.")
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
        .arg(
            Arg::new("norb")
                .long("norb")
                .value_name("NORBS")
                .about("Total number of orbitals.")
                .takes_value(true),
        )
        .get_matches();

    // TODO:
    // - by hand HOMO selection (in case the HOMO is not to be included)
    // - by hand active electron

    // Get the orbital indices
    let homos = BTreeSet::from_iter(
        matches
            .values_of_t::<usize>("homo")
            .unwrap_or_else(|e| e.exit()),
    );
    let lumos = BTreeSet::from_iter(
        matches
            .values_of_t::<usize>("lumo")
            .unwrap_or_else(|e| e.exit()),
    );

    // number of total electrons, active orbitals and active electrons
    let nel = match matches.value_of_t::<usize>("nel") {
        Ok(i) => i,
        Err(e) => e.exit(),
    };

    if nel % 2 == 1 {
        panic!("Odd number of electrons is not supported (yet).")
    }

    let norb = match matches.value_of("norb") {
        Some(i) => i,
        None => "<NORBITALS>",
    };

    // let nel = matches
    //     .values_of_t::<usize>("nel")
    //     .unwrap_or_else(|e| e.exit());
    let actorb = homos.len() + lumos.len();
    let actel = homos.len() * 2;
    let start = nel / 2 - homos.len();

    if homos.iter().max() > lumos.iter().min() {
        // TODO nicer error
        panic!("overlapping homo and lumo spaces are not supported!")
    }

    let mut iorder = BTreeMap::new();
    for k in 1..actorb + 1 {
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
            iorder.insert(*element, free);
        } else {
            panic!(
                "Problem! Ran out of free spots to insert new orbitals.\n \
                 This should never happen."
            )
        };
    }

    // Print guess group
    println!(" $guess guess=moread norb={} norder=1 $end", norb);

    // Print reordering
    let start = String::from(" $guess");
    let mut s = start.clone();
    let mut last = 0;
    for (key, value) in iorder.iter() {
        if key != value {
            if *key == last + 1 {
                s = format!("{},{}", s, value);
            } else {
                s = format!("{} iorder({})={}", s, key, value);
            }
            last = *key;
            if s.len() >= 60 {
                println!("{} $end", s);
                s = start.clone();
            }
        }
    }
    if s.len() > start.len() {
        println!("{} $end", s);
    }

    // Print $det group
    println!(
        " $det ncore={} nact={} nels={} $end",
        (nel - actel) / 2,
        actorb,
        actel
    );
}
