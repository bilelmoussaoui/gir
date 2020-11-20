use crate::{codegen::generate_single_version_file, env::Env, library::{MAIN_NAMESPACE, Type}, version::Version};
use std::collections::BTreeMap;


pub fn generate(env: &Env) {
    for x in env.library.namespace(MAIN_NAMESPACE).types.iter() {
        match x {
            Some(Type::Class(c)) =>  {

                c.virtual_methods.iter().for_each(|v| {
                    println!("[ ] {} - {:#?}", c.name, v.name);
                });
                println!("##############");
            },
            Some(Type::Interface(i)) => {
                i.virtual_methods.iter().for_each(|v| {
                    println!("[ ] {} - {:#?}", i.name, v.name);
                });
                println!("##############");
            },
            _ => (),
        }
    }
}



