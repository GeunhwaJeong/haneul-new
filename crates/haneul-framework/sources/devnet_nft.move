// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// A minimalist example to demonstrate how to create an NFT like object
/// on Haneul. The user should be able to use the wallet command line tool
/// (https://docs.haneul.io/build/wallet) to mint an NFT. For example,
/// `wallet example-nft --name <Name> --description <Description> --url <URL>`
module haneul::devnet_nft {
    use haneul::url::{Self, Url};
    use haneul::utf8;
    use haneul::object::{Self, ID, UID};
    use haneul::event;
    use haneul::transfer;
    use haneul::tx_context::{Self, TxContext};

    /// An example NFT that can be minted by anybody
    struct DevNetNFT has key, store {
        id: UID,
        /// Name for the token
        name: utf8::String,
        /// Description of the token
        description: utf8::String,
        /// URL for the token
        url: Url,
        // TODO: allow custom attributes
    }

    struct MintNFTEvent has copy, drop {
        // The Object ID of the NFT
        object_id: ID,
        // The creator of the NFT
        creator: address,
        // The name of the NFT
        name: utf8::String,
    }

    /// Create a new devnet_nft
    public entry fun mint(
        name: vector<u8>,
        description: vector<u8>,
        url: vector<u8>,
        ctx: &mut TxContext
    ) {
        let nft = DevNetNFT {
            id: object::new(ctx),
            name: utf8::string_unsafe(name),
            description: utf8::string_unsafe(description),
            url: url::new_unsafe_from_bytes(url)
        };
        let sender = tx_context::sender(ctx);
        event::emit(MintNFTEvent {
            object_id: object::uid_to_inner(&nft.id),
            creator: sender,
            name: nft.name,
        });
        transfer::transfer(nft, sender);
    }

    /// Transfer `nft` to `recipient`
    public entry fun transfer(
        nft: DevNetNFT, recipient: address, _: &mut TxContext
    ) {
        transfer::transfer(nft, recipient)
    }

    /// Update the `description` of `nft` to `new_description`
    public entry fun update_description(
        nft: &mut DevNetNFT,
        new_description: vector<u8>,
        _: &mut TxContext
    ) {
        nft.description = utf8::string_unsafe(new_description)
    }

    /// Permanently delete `nft`
    public entry fun burn(nft: DevNetNFT, _: &mut TxContext) {
        let DevNetNFT { id, name: _, description: _, url: _ } = nft;
        object::delete(id)
    }

    /// Get the NFT's `name`
    public fun name(nft: &DevNetNFT): &utf8::String {
        &nft.name
    }

    /// Get the NFT's `description`
    public fun description(nft: &DevNetNFT): &utf8::String {
        &nft.description
    }

    /// Get the NFT's `url`
    public fun url(nft: &DevNetNFT): &Url {
        &nft.url
    }
}

#[test_only]
module haneul::devnet_nftTests {
    use haneul::devnet_nft::{Self, DevNetNFT};
    use haneul::test_scenario;
    use haneul::utf8;

    #[test]
    fun mint_transfer_update() {
        let addr1 = @0xA;
        let addr2 = @0xB;
        // create the NFT
        let scenario = test_scenario::begin(&addr1);
        {
            devnet_nft::mint(b"test", b"a test", b"https://www.haneul.io", test_scenario::ctx(&mut scenario))
        };
        // send it from A to B
        test_scenario::next_tx(&mut scenario, &addr1);
        {
            let nft = test_scenario::take_owned<DevNetNFT>(&mut scenario);
            devnet_nft::transfer(nft, addr2, test_scenario::ctx(&mut scenario));
        };
        // update its description
        test_scenario::next_tx(&mut scenario, &addr2);
        {
            let nft = test_scenario::take_owned<DevNetNFT>(&mut scenario);
            devnet_nft::update_description(&mut nft, b"a new description", test_scenario::ctx(&mut scenario)) ;
            assert!(*utf8::bytes(devnet_nft::description(&nft)) == b"a new description", 0);
            test_scenario::return_owned(&mut scenario, nft);
        };
        // burn it
        test_scenario::next_tx(&mut scenario, &addr2);
        {
            let nft = test_scenario::take_owned<DevNetNFT>(&mut scenario);
            devnet_nft::burn(nft, test_scenario::ctx(&mut scenario))
        }
    }
}
