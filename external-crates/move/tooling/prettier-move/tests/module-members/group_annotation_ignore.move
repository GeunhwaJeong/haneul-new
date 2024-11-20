// options:
// printWidth: 40
// useModuleLabel: true
// autoGroupImports: package

module prettier::group_imports;

use haneul::{
    clock::Clock,
    coin::Coin,
    dynamic_field as df,
    dynamic_object_field as dof,
    haneul::HANEUL,
    table::{Self, Table},
    table_vec::{Self, TableVec as TV}
};

#[test_only]
use std::{
    string::String as UTF8,
    ascii::String as ASCII,
    vector as vec,
    vector as haha,
    option::{Self as opt, Option},
    type_name::get as type_name_get,
};
