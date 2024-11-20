// options:
// printWidth: 50
// useModuleLabel: true
// autoGroupImports: module

module prettier::use_declaration;

use haneul::coin::Coin;
use haneul::coin::Coin as C;
use haneul::coin::{Self as c, Coin as C};
use haneul::coin::very_long_function_name_very_long_function_name as short_name;
use beep::staked_haneul::StakedHaneul;

use haneul::transfer_policy::{Self as policy, TransferPolicy, TransferPolicyCap, TransferRequest};
use haneul::transfer_policy::TransferPolicyCap as cap;
use haneul::{
    transfer_policy::{TransferPolicy, TransferPolicyCap, TransferRequest, Kek as KEK},
    transfer_policy::TransferPolicyCap as cap,
};

public use fun my_custom_function_with_a_long_name as TransferPolicyCap.very_long_function_name;

friend has_been::here;

// will break before `as`
public use fun my_custom_function_with_a_long_name
    as TransferPolicyCap.very_long_function_name;
