#[test]
fn no_decl() {
    consts! {}
}

#[test]
fn single_decl() {
    consts! {
        KEY: "VALUE";
    }

    assert_eq!(KEY, "VALUE");
}

#[test]
fn empty_level_decl() {
    consts! {
        mod_name {}
    }
}

#[test]
fn some_levels_decl() {
    consts! {
        three {
            two {
                one {
                    GO: "SHOOT!";
                }
            }
        }
    }

    assert_eq!(three::two::one::GO, "SHOOT!");
}

#[test]
fn literal_level_literal_decl() {
    consts! {
        LITERAL1: "1";
        level {}
        LITERAL2: "2";
    }

    assert_eq!(LITERAL1, "1");
    assert_eq!(LITERAL2, "2");
}

#[test]
fn level_literal_level_decl() {
    consts! {
        level1 {}
        LITERAL: "0";
        level2 {}
    }

    assert_eq!(LITERAL, "0");
}

#[test]
fn complicated_decl() {
    consts! {
        level1 {
            VALUE: "1";
        }

        level2 {
            VALUE: "2";
            level21 {
                VALUE1: "3";
                VALUE2: "4";
            }
        }

        VALUE: "5";

        level3 {
            level31 {
                VALUE: "6";
            }
            VALUE: "7";
            level32 {
                VALUE1: "8";
                level321 {
                    VALUE: "9";
                }
                VALUE2: "10";
            }
            level33 {
                level331 {
                    level3311 {
                        level33111 {}
                    }
                }
            }
        }

        END: "11";
    }

    assert_eq!(level1::VALUE, "1");
    assert_eq!(level2::VALUE, "2");
    assert_eq!(level2::level21::VALUE1, "3");
    assert_eq!(level2::level21::VALUE2, "4");
    assert_eq!(VALUE, "5");
    assert_eq!(level3::level31::VALUE, "6");
    assert_eq!(level3::VALUE, "7");
    assert_eq!(level3::level32::VALUE1, "8");
    assert_eq!(level3::level32::level321::VALUE, "9");
    assert_eq!(level3::level32::VALUE2, "10");
    assert_eq!(END, "11");
}
