# cassf-tete

    Casse-tête (n.m.): Au Québec, un puzzle. 
    --- [Larousse dictionary](https://www.larousse.fr/dictionnaires/francais/casse-t%C3%AAte/13626)

`casscf-tete` is a simple program that builds input decks for US-GAMESS
CAS-SCF calculations.

## Usage

Call it with the number of electrons of the molecule and
your choice of HOMO and LUMO orbitals, and it will print out the appropriate
`$guess` and `$det` decks,
```bash
$ ./casscf-tete 60 --homo 24 28 29 30 --lumo 31 37 34 33 --norb 270
 $guess guess=moread norb=270 norder=1 $end
 $guess iorder(24)=27 iorder(27)=24 iorder(32)=37 iorder(37)=32 $end
 $det ncore=26 nact=8 nels=8 $end
```

## Installation

On linux, download the binary
```bash
wget
https://github.com/clavigne/casscf-tete/releases/download/v0.1.0/casscf-tete 
```
and run. That's it!

On other platforms, you can build it using [Cargo and rustc,](https://doc.rust-lang.org/cargo/getting-started/installation.html)
```bash
git clone https://github.com/clavigne/casscf-tete/
cd casscf-tete
cargo --build --release
```


## Features

Here is the list of features:

- It helps you write your CASSCF decks for US-GAMESS.


## License

casscf-tete is [free and unencumbered software released in the public domain.](./LICENSE)
