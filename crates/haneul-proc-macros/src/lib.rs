// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn init_static_initializers(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::ItemFn);

    let body = &input.block;
    input.block = syn::parse2(quote! {
        {
            // We have some lazily-initialized static state in the program. The initializers
            // alter the thread-local hash container state any time they create a new hash
            // container. Therefore, we need to ensure that these initializers are run in a
            // separate thread before the first test thread is launched. Otherwise, they would
            // run inside of the first test thread, but not subsequent ones.
            //
            // Note that none of this has any effect on process-level determinism. Without this
            // code, we can still get the same test results from two processes started with the
            // same seed.
            //
            // However, when using sim_test(check_determinism) or MSIM_TEST_CHECK_DETERMINISM=1,
            // we want the same test invocation to be deterministic when run twice
            // _in the same process_, so we need to take care of this. This will also
            // be very important for being able to reproduce a failure that occurs in the Nth
            // iteration of a multi-iteration test run.
            std::thread::spawn(|| {
                ::haneul_simulator::telemetry_subscribers::init_for_testing();
                ::haneul_simulator::haneul_framework::get_move_stdlib();
                ::haneul_simulator::haneul_framework::get_haneul_framework();
                ::haneul_simulator::haneul_types::gas::HaneulGasStatus::new_unmetered();

                // For reasons I can't understand, LruCache causes divergent behavior the second
                // time one is constructed and inserted into, so construct one before the first
                // test run for determinism.
                let mut cache = ::haneul_simulator::lru::LruCache::new(1.try_into().unwrap());
                cache.put(1, 1);

                {
                    // Initialize the static initializers here:
                    // https://github.com/move-language/move/blob/652badf6fd67e1d4cc2aa6dc69d63ad14083b673/language/tools/move-package/src/package_lock.rs#L12
                    use std::path::PathBuf;
                    use haneul_simulator::haneul_framework_build::compiled_package::{BuildConfig, HaneulPackageHooks};
                    use haneul_simulator::haneul_framework::build_move_package;
                    use haneul_simulator::tempfile::TempDir;
		    use haneul_simulator::move_package::package_hooks::register_package_hooks;

		    move_package::package_hooks::register_package_hooks(Box::new(HaneulPackageHooks {}));
                    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                    path.push("../../haneul_programmability/examples/basics");
                    let mut build_config = BuildConfig::default();

                    build_config.config.install_dir = Some(TempDir::new().unwrap().into_path());
                    let _all_module_bytes = build_move_package(&path, build_config)
                        .unwrap()
                        .get_package_bytes(/* with_unpublished_deps */ false);
                }


                use ::haneul_simulator::anemo_tower::callback::CallbackLayer;
                use ::haneul_simulator::anemo_tower::trace::DefaultMakeSpan;
                use ::haneul_simulator::anemo_tower::trace::DefaultOnFailure;
                use ::haneul_simulator::anemo_tower::trace::TraceLayer;
                use ::haneul_simulator::narwhal_network::metrics::MetricsMakeCallbackHandler;
                use ::haneul_simulator::narwhal_network::metrics::NetworkMetrics;

                use std::sync::Arc;
                use ::haneul_simulator::fastcrypto::traits::KeyPair;
                use rand::rngs::{StdRng, OsRng};
                use rand::SeedableRng;
                use ::haneul_simulator::tower::ServiceBuilder;

                // anemo uses x509-parser, which has many lazy static variables. start a network to
                // initialize all that static state before the first test.
                let rt = ::haneul_simulator::runtime::Runtime::new();
                rt.block_on(async move {
                    use ::haneul_simulator::anemo::{Network, Request};

                    let make_network = |port: u16| {
                        let registry = prometheus::Registry::new();
                        let inbound_network_metrics =
                            NetworkMetrics::new("haneul", "inbound", &registry);
                        let outbound_network_metrics =
                            NetworkMetrics::new("haneul", "outbound", &registry);

                        let service = ServiceBuilder::new()
                            .layer(
                                TraceLayer::new_for_server_errors()
                                    .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                                    .on_failure(DefaultOnFailure::new().level(tracing::Level::WARN)),
                            )
                            .layer(CallbackLayer::new(MetricsMakeCallbackHandler::new(
                                Arc::new(inbound_network_metrics),
                                usize::MAX,
                            )))
                            .service(::haneul_simulator::anemo::Router::new());

                        let outbound_layer = ServiceBuilder::new()
                            .layer(
                                TraceLayer::new_for_client_and_server_errors()
                                    .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                                    .on_failure(DefaultOnFailure::new().level(tracing::Level::WARN)),
                            )
                            .layer(CallbackLayer::new(MetricsMakeCallbackHandler::new(
                                Arc::new(outbound_network_metrics),
                                usize::MAX,
                            )))
                            .into_inner();


                        Network::bind(format!("127.0.0.1:{}", port))
                            .server_name("static-init-network")
                            .private_key(
                                ::haneul_simulator::fastcrypto::ed25519::Ed25519KeyPair::generate(&mut StdRng::from_rng(OsRng).unwrap())
                                    .private()
                                    .0
                                    .to_bytes(),
                            )
                            .start(service)
                            .unwrap()
                    };
                    let n1 = make_network(80);
                    let n2 = make_network(81);

                    let _peer = n1.connect(n2.local_addr()).await.unwrap();
                });
            }).join().unwrap();

            #body
        }
    })
    .expect("Parsing failure");

    let result = quote! {
        #input
    };

    result.into()
}

/// The haneul_test macro will invoke either `#[msim::test]` or `#[tokio::test]`,
/// depending on whether the simulator config var is enabled.
///
/// This should be used for tests that can meaningfully run in either environment.
#[proc_macro_attribute]
pub fn haneul_test(args: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);

    let header = if cfg!(msim) {
        quote! {
            #[::haneul_simulator::sim_test(crate = "haneul_simulator", #(#args)*)]
            #[::haneul_macros::init_static_initializers]
        }
    } else {
        quote! {
            #[::tokio::test(#(#args)*)]
            // though this is not required for tokio, we do it to get logs as well.
            #[::haneul_macros::init_static_initializers]
        }
    };

    let result = quote! {
        #header
        #input
    };

    result.into()
}

/// The sim_test macro will invoke `#[msim::test]` if the simulator config var is enabled.
///
/// Otherwise, it will emit an ignored test - if forcibly run, the ignored test will panic.
///
/// This macro must be used in order to pass any simulator-specific arguments, such as
/// `check_determinism`, which is not understood by tokio.
#[proc_macro_attribute]
pub fn sim_test(args: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);

    let result = if cfg!(msim) {
        quote! {
            #[::haneul_simulator::sim_test(crate = "haneul_simulator", #(#args)*)]
            #[::haneul_macros::init_static_initializers]
            #input
        }
    } else {
        let fn_name = &input.sig.ident;
        let sig = &input.sig;
        let body = &input.block;
        quote! {
            #[tokio::test]
            #sig {
                if std::env::var("HANEUL_SKIP_SIMTESTS").is_ok() {
                    println!("not running test {} in `cargo test`: HANEUL_SKIP_SIMTESTS is set", stringify!(#fn_name));

                    struct Ret;

                    impl From<Ret> for () {
                        fn from(_ret: Ret) -> Self {
                        }
                    }

                    impl<E> From<Ret> for Result<(), E> {
                        fn from(_ret: Ret) -> Self {
                            Ok(())
                        }
                    }

                    return Ret.into();
                }

                #body
            }
        }
    };

    result.into()
}
