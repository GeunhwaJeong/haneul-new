// options:
// printWidth: 40
// useModuleLabel: true
// autoGroupImports: module

module prettier::group_imports;

use a::b as c;
use a::b::e as f;
use a::{b::g as f, b::h as i};

use haneul::balance::{Self, Balance};
use haneul::coin::{Self, Coin};
use haneul::dynamic_field as df;
use haneul::dynamic_object_field as dof;
use haneul::event;
use haneul::haneul::HANEUL;
use haneul::transfer_policy::{Self, TransferPolicy, TransferRequest};

use std::{
    string::String,
    ascii::String as ASCII,
    vector as vec,
    option::{Self as opt, Option},
    type_name::get as type_name_get,
};

public fun do_something() {}
