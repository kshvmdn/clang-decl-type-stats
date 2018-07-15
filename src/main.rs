extern crate clang;

use clang::*;
use std::env;
use std::process;

fn usage(arg0: String) {
    println!("usage: {} /path/to/file ...", arg0);
}

fn vardecl(typ: Option<Type>, prevtyp: Option<Type>) -> bool {
    if let (Some(typ), Some(prevtyp)) = (typ, prevtyp) {
        return typ.get_kind() == prevtyp.get_kind();
    }
    false
}

fn main() {
    if env::args().len() <= 1 {
        usage(env::args().collect());
        process::exit(1);
    }

    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, true, false);

    let paths: Vec<String> = env::args().skip(1).collect();
    let mut total_commas = 0u64;
    let mut total_semicolons = 0u64;

    macro_rules! print_result {
        ($n:expr) => {
            print_result!($n, total_commas, total_semicolons)
        };
        ($n:expr, $c:expr, $s:expr) => {{
            print!("{}", $n);
            println!("comma-delimited decls     : {:?}", $c);
            println!("semicolon-delimited decls : {:?}", $s);
        }};
    }

    for path in &paths {
        // TODO: Figure out how to parallelize these parses.
        let tu = index.parser(&path).parse().unwrap();

        let mut prevtyp: Option<Type> = None;
        let mut n_commas = 0u64;
        let mut n_semicolons = 0u64;

        // Filter out any entities that are from a file that isn't |path|. These are generally
        // header files included via #include (which must be parsed since they may expose relevant
        // type information).
        let entities: Vec<Entity> = tu.get_entity()
            .get_children()
            .into_iter()
            .filter(|e| {
                let source_fp = e.get_location()
                    .map(|source| source.get_file_location())
                    .and_then(|location| location.file)
                    .and_then(|file| Some(file.get_path()));
                source_fp != None && source_fp.unwrap() == std::path::PathBuf::from(&path)
            })
            .collect();

        entities.into_iter().for_each(|e| {
            e.visit_children(|c, _| match c.get_kind() {
                EntityKind::VarDecl => {
                    if vardecl(c.get_type(), prevtyp) {
                        n_semicolons += 1;
                    }
                    prevtyp = c.get_type();
                    EntityVisitResult::Continue
                }

                EntityKind::DeclStmt => {
                    if c.get_children().len() == 1 {
                        // A DeclStmt with a single child is a "semicolon declaration". We recurse
                        // to check that the type of this declaration matches that of the previous
                        // declaration.
                        return EntityVisitResult::Recurse;
                    }
                    n_commas += c.get_children().len() as u64 - 1;
                    // XXX: Group comma decls with non-comma decls?
                    prevtyp = None;
                    EntityVisitResult::Continue
                }

                // XXX: Recurse only for certain EntityKinds?
                _ => {
                    prevtyp = None;
                    EntityVisitResult::Recurse
                }
            });
        });

        total_commas += n_commas;
        total_semicolons += n_semicolons;
        if paths.len() > 1 {
            print_result!(format!("{}\n", path), n_commas, n_semicolons);
        }
    }

    print_result!(if paths.len() > 1 { "total\n" } else { "" });
}
