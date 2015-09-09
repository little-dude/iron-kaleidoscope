//< ch-0
#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;

extern crate iron_kaleidoscope;

use iron_kaleidoscope::driver::{main_loop,
//> ch-0
                                AST,
                                IR,
                                Exec,
//< ch-0
                                Tokens
};

//< parser-main
docopt!(Args, "
Usage: iron_kaleidoscope [(-l | -p | -i)]

Options:
    -l  Run only lexer and show its output.
    -p  Run only parser and show its output.
    -i  Run only IR builder and show its output.
");

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

//> ch-0
    let stage = if args.flag_l {
        Tokens
//> parser-main
    } else if args.flag_p {
        AST
    } else if args.flag_i {
        IR
//< parser-main
    } else {
//> parser-main
/*
//< ch-0
    if args.flag_p || args.flag_i {
        panic!("not implemented");
    }
    let stage = Tokens;
//> ch-0
//< parser-main
        AST
//> parser-main
*/
        //IR
        Exec
//< parser-main
    };
//< ch-0

    main_loop(stage);
}
//> ch-0 parser-main
