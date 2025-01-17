// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of ink!.
//
// ink! is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// ink! is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with ink!.  If not, see <http://www.gnu.org/licenses/>.

use super::*;

#[test]
fn contract_compiles() {
    assert_eq_tokenstreams(
        quote! {
            #![env = DefaultSrmlTypes]

            /// A simple contract that has a boolean value that can be flipped and be returned.
            struct Flipper {
                /// The internal value.
                value: storage::Value<bool>,
            }

            impl Deploy for Flipper {
                /// The internal boolean is initialized with `true`.
                fn deploy(&mut self) {
                    self.value.set(true)
                }
            }

            impl Flipper {
                /// Flips the internal boolean.
                pub(external) fn flip(&mut self) {
                    self.value = !(*self.value)
                }

                /// Returns the internal boolean.
                pub(external) fn get(&self) -> bool {
                    *self.value
                }
            }
        },
        quote! {
            mod types {
                use super::*;
                use ink_core::env::{ContractEnv, EnvTypes};

                pub type AccountId = <DefaultSrmlTypes as EnvTypes>::AccountId;
                pub type Balance = <DefaultSrmlTypes as EnvTypes>::Balance;
                pub type Hash = <DefaultSrmlTypes as EnvTypes>::Hash;
                pub type Moment = <DefaultSrmlTypes as EnvTypes>::Moment;
                pub type BlockNumber = <DefaultSrmlTypes as EnvTypes>::BlockNumber;
            }

            type Env = ink_core::env::ContractEnv<DefaultSrmlTypes>;
            use types::{
                AccountId,
                Balance,
                Hash,
                Moment,
                BlockNumber,
            };

            #[cfg(not(feature = "ink-as-dependency"))]
            mod normal {
                use super::*;

                ink_model::state! {
                    /// A simple contract that has a boolean value that can be flipped and be returned.
                    #[cfg_attr(
                        feature = "ink-generate-abi",
                        derive(type_metadata::Metadata, ink_abi::HasLayout,)
                    )]
                    pub struct Flipper {
                        /// The internal value.
                        value: storage::Value<bool>,
                    }
                }

                mod msg {
                    use super::*;
                    use ink_model::messages;

                    ink_model::messages! {
                        /// Flips the internal boolean.
                        970692492 => Flip();
                        /// Returns the internal boolean.
                        4266279973 => Get() -> bool;
                    }
                }

                impl Flipper {
                    /// The internal boolean is initialized with `true`.
                    pub fn deploy(&mut self, env: &mut ink_model::EnvHandler<ink_core::env::ContractEnv<DefaultSrmlTypes> >) {
                        self.value.set(true)
                    }

                    /// Flips the internal boolean.
                    pub fn flip(&mut self, env: &mut ink_model::EnvHandler<ink_core::env::ContractEnv<DefaultSrmlTypes> >) {
                        self.value = !(*self.value)
                    }

                    /// Returns the internal boolean.
                    pub fn get(&self, env: &ink_model::EnvHandler<ink_core::env::ContractEnv<DefaultSrmlTypes> >) -> bool {
                        *self.value
                    }
                }

                use ink_model::Contract as _;

                #[cfg(not(test))]
                impl Flipper {
                    pub(crate) fn instantiate() -> impl ink_model::Contract {
                        ink_model::ContractDecl::using::<Self, ink_core::env::ContractEnv<DefaultSrmlTypes>>()
                            .on_deploy(|env, ()| {
                                let (handler, state) = env.split_mut();
                                state.deploy(handler,)
                            })
                            .on_msg_mut::<msg::Flip>(|env, _| {
                                let (handler, state) = env.split_mut();
                                state.flip(handler,)
                            })
                            .on_msg::<msg::Get>(|env, _| {
                                let (handler, state) = env.split();
                                state.get(handler,)
                            })
                            .instantiate()
                    }
                }

                #[cfg(not(test))] #[no_mangle] fn deploy() -> u32 { Flipper::instantiate().deploy().to_u32() }
                #[cfg(not(test))] #[no_mangle] fn call() -> u32 { Flipper::instantiate().dispatch().to_u32() }
            }

            #[cfg(not(feature = "ink-as-dependency"))]
            use normal::*;

            #[cfg(not(feature = "ink-as-dependency"))]
            use ink_core::env::FromAccountId as _;

            #[cfg(test)]
            mod test {
                use super::*;

                pub struct TestableFlipper {
                    env: ink_model::ExecutionEnv<Flipper, ink_core::env::ContractEnv<DefaultSrmlTypes>>,
                }

                impl Flipper {
                    /// Returns a testable version of the contract.
                    pub fn deploy_mock() -> TestableFlipper {
                        let mut mock = TestableFlipper::allocate();
                        mock.deploy();
                        mock
                    }
                }

                impl TestableFlipper {
                    /// Allocates the testable contract storage.
                    fn allocate() -> Self {
                        use ink_core::storage::{
                            Key,
                            alloc::{
                                AllocateUsing as _,
                                Initialize as _,
                                BumpAlloc,
                            },
                        };
                        Self {
                            env: unsafe {
                                let mut alloc = BumpAlloc::from_raw_parts(Key([0x0; 32]));
                                ink_model::ExecutionEnv::allocate_using(&mut alloc).initialize_into(())
                            }
                        }
                    }

                    /// Deploys the testable contract by initializing it with the given values.
                    fn deploy(&mut self,) {
                        let (handler, state) = self.env.split_mut();
                        state.deploy(handler,)
                    }
                }

                impl TestableFlipper {
                    pub fn flip(&mut self) {
                        let (handler, state) = self.env.split_mut();
                        state.flip(handler,)
                    }

                    pub fn get(&self) -> bool {
                        let (handler, state) = self.env.split();
                        state.get(handler,)
                    }
                }
            }

            #[cfg(not(feature = "ink-as-dependency"))]
            #[cfg(feature = "ink-generate-abi")]
            pub fn ink_generate_abi() -> ink_abi::InkProject{
                let contract = {
                    ink_abi::ContractSpec::new("Flipper")
                        .constructors(vec![
                            ink_abi::ConstructorSpec::new("on_deploy")
                                .selector(0)
                                .args(vec![])
                                .docs(vec![
                                    "The internal boolean is initialized with `true`.",
                                ])
                                .done()
                        ])
                        .messages(vec![
                            ink_abi::MessageSpec::new("flip")
                                .selector(970692492u32)
                                .mutates(true)
                                .args(vec![])
                                .docs(vec!["Flips the internal boolean.",])
                                .returns(ink_abi::ReturnTypeSpec::new(None))
                                .done(),
                            ink_abi::MessageSpec::new("get")
                                .selector(4266279973u32)
                                .mutates(false)
                                .args(vec![])
                                .docs(vec!["Returns the internal boolean.",])
                                .returns(
                                    ink_abi::ReturnTypeSpec::new(
                                        ink_abi::TypeSpec::with_name_segs::<bool, _>(
                                            vec!["bool"].into_iter().map(AsRef::as_ref)
                                        )
                                    )
                                )
                                .done(),
                            ])
                            .events(vec![])
                            .docs(vec![])
                            .done()
                };
                let layout = {
                    unsafe {
                        use ink_core::storage::alloc::AllocateUsing as _;
                        use ink_abi::HasLayout as _;
                        Flipper::allocate_using(
                            &mut ink_core::storage::alloc::BumpAlloc::from_raw_parts(
                                ink_core::storage::Key([0x0; 32])
                            )
                        ).layout()
                    }
                };
                ink_abi::InkProject::new(layout, contract)
            }

            #[cfg(feature = "ink-as-dependency")]
            mod as_dependency {
                use super::*;

                /// A simple contract that has a boolean value that can be flipped and be returned.
                #[derive(Clone, scale::Encode, scale::Decode)]
                #[cfg_attr(feature = "ink-generate-abi", derive(type_metadata::Metadata))]
                pub struct Flipper {
                    account_id: AccountId,
                }

                impl ink_core::storage::Flush for Flipper {
                    fn flush(&mut self) {}
                }

                /// Allows to enhance calls to `&self` contract messages.
                pub struct CallEnhancer<'a> {
                    contract: &'a Flipper,
                }

                /// Allows to enhance calls to `&mut self` contract messages.
                pub struct CallEnhancerMut<'a> {
                    contract: &'a mut Flipper,
                }

                impl ink_core::env::FromAccountId<Env> for Flipper {
                    fn from_account_id(account_id: AccountId) -> Self {
                        Self { account_id }
                    }
                }

                impl Flipper {
                    /// The internal boolean is initialized with `true`.
                    pub fn new(code_hash: Hash,) -> ink_core::env::CreateBuilder<Env, Self> {
                        ink_core::env::CreateBuilder::<Env, Self>::new(code_hash)
                    }
                    /// Returns the internal account ID of the contract.
                    pub fn account_id(&self) -> AccountId {
                        self.account_id
                    }
                    /// Allows to enhance calls to `&self` contract messages.
                    pub fn call(&self) -> CallEnhancer {
                        CallEnhancer { contract : self }
                    }
                    /// Allows to enhance calls to `&mut self` contract messages.
                    pub fn call_mut(&mut self) -> CallEnhancerMut {
                        CallEnhancerMut { contract : self }
                    }
                }

                impl Flipper {
                    /// Flips the internal boolean.
                    pub fn flip(&mut self,) {
                        self
                            .call_mut()
                            .flip()
                            .fire()
                            .expect(
                                concat!(
                                    "invocation of ",
                                    stringify!(Flipper),
                                    "::",
                                    stringify!(flip),
                                    " message was invalid"
                                )
                            )
                    }

                    /// Returns the internal boolean.
                    pub fn get(&self,) -> bool {
                        self
                            .call()
                            .get()
                            .fire()
                            .expect(
                                concat!(
                                    "evaluation of ",
                                    stringify!(Flipper),
                                    "::",
                                    stringify!(get),
                                    " message was invalid"
                                )
                            )
                    }
                }

                impl<'a> CallEnhancer<'a> {
                    /// Returns the internal boolean.
                    pub fn get(self,) -> ink_core::env::CallBuilder<Env, ink_core::env::ReturnType<bool>> {
                        ink_core::env::CallBuilder::eval(
                            self.contract.account_id.clone(), 4266279973u32
                        )
                    }
                }

                impl<'a> CallEnhancerMut<'a> {
                    /// Flips the internal boolean.
                    pub fn flip(self,) -> ink_core::env::CallBuilder<Env, ()> {
                        ink_core::env::CallBuilder::<Env, ()>::invoke(
                            self.contract.account_id.clone(), 970692492u32)
                    }
                }
            }

            #[cfg(feature = "ink-as-dependency")]
            pub use as_dependency::{Flipper, CallEnhancer, CallEnhancerMut,};
        },
    )
}
