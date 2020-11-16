use crate::{codegen::generate_single_version_file, env::Env, version::Version};
use std::collections::BTreeMap;


pub fn generate(env: &Env) {
    env.library.show_non_bound_types(env);
}
