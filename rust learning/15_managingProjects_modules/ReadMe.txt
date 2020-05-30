1. Binary file -> main.rs
        Binary files(main.rs) can be many in bin folder
            Project    => Package
                |
                |-Cargo.toml
                |-Cargo.lock
                |-.gitignore
                ----> src
                            |
                            |-lib.rs   (library file can be 0 or 1 but it's not executable)
                            |-main.rs   (main.rs can be multiple)
                            ------> bin
                                          |
                                          |-a.rs
                                          |-b.rs
                                        etc 

2. Create lib.rs in Terminal -> cargo new --lib <name>
3. Public -> pub (to show functionality to use other developer)
4. Private-> (Confidential code not to show others) 
                    by default definition(enum, struct, function) is private.
               
5. Module constain specific definition of code.
    Parent module -> Cant access child module defnitions
    Child module -> Can access Parent module definitions
6. Absolute/Relative
	Absolute -> Path start form root folder
	Relative => path start from current working directory
7. Delimiter 
          |
          -> Windows = /
          -> Linux = \
          -> Rust = : :
8.  #![allow(dead_code)] -> Code not to use 
 