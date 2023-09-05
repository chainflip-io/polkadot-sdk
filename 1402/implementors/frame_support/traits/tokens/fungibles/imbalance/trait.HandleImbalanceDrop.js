(function() {var implementors = {
"assets_common":[["impl&lt;Assets, LocalAssetIdConverter, ForeignAssets&gt; <a class=\"trait\" href=\"frame_support/traits/tokens/fungibles/imbalance/trait.HandleImbalanceDrop.html\" title=\"trait frame_support::traits::tokens::fungibles::imbalance::HandleImbalanceDrop\">HandleImbalanceDrop</a>&lt;<a class=\"struct\" href=\"staging_xcm/v3/multilocation/struct.MultiLocation.html\" title=\"struct staging_xcm::v3::multilocation::MultiLocation\">MultiLocation</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.u128.html\">u128</a>&gt; for <a class=\"struct\" href=\"assets_common/local_and_foreign_assets/struct.CreditDropIndirection.html\" title=\"struct assets_common::local_and_foreign_assets::CreditDropIndirection\">CreditDropIndirection</a>&lt;Assets, LocalAssetIdConverter, ForeignAssets&gt;<span class=\"where fmt-newline\">where\n    Assets: <a class=\"trait\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Balanced.html\" title=\"trait frame_support::traits::tokens::fungibles::regular::Balanced\">Balanced</a>&lt;<a class=\"type\" href=\"parachains_common/types/type.AccountId.html\" title=\"type parachains_common::types::AccountId\">AccountId</a>&gt; + <a class=\"trait\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Inspect.html\" title=\"trait frame_support::traits::tokens::fungibles::regular::Inspect\">Inspect</a>&lt;<a class=\"type\" href=\"parachains_common/types/type.AccountId.html\" title=\"type parachains_common::types::AccountId\">AccountId</a>, Balance = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.u128.html\">u128</a>, AssetId = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.u32.html\">u32</a>&gt;,\n    LocalAssetIdConverter: <a class=\"trait\" href=\"sp_runtime/traits/trait.MaybeEquivalence.html\" title=\"trait sp_runtime::traits::MaybeEquivalence\">MaybeEquivalence</a>&lt;<a class=\"struct\" href=\"staging_xcm/v3/multilocation/struct.MultiLocation.html\" title=\"struct staging_xcm::v3::multilocation::MultiLocation\">MultiLocation</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.u32.html\">u32</a>&gt;,\n    ForeignAssets: <a class=\"trait\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Balanced.html\" title=\"trait frame_support::traits::tokens::fungibles::regular::Balanced\">Balanced</a>&lt;<a class=\"type\" href=\"parachains_common/types/type.AccountId.html\" title=\"type parachains_common::types::AccountId\">AccountId</a>&gt; + <a class=\"trait\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Inspect.html\" title=\"trait frame_support::traits::tokens::fungibles::regular::Inspect\">Inspect</a>&lt;<a class=\"type\" href=\"parachains_common/types/type.AccountId.html\" title=\"type parachains_common::types::AccountId\">AccountId</a>, Balance = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.u128.html\">u128</a>, AssetId = <a class=\"struct\" href=\"staging_xcm/v3/multilocation/struct.MultiLocation.html\" title=\"struct staging_xcm::v3::multilocation::MultiLocation\">MultiLocation</a>&gt;,</span>"],["impl&lt;Assets, LocalAssetIdConverter, ForeignAssets&gt; <a class=\"trait\" href=\"frame_support/traits/tokens/fungibles/imbalance/trait.HandleImbalanceDrop.html\" title=\"trait frame_support::traits::tokens::fungibles::imbalance::HandleImbalanceDrop\">HandleImbalanceDrop</a>&lt;<a class=\"struct\" href=\"staging_xcm/v3/multilocation/struct.MultiLocation.html\" title=\"struct staging_xcm::v3::multilocation::MultiLocation\">MultiLocation</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.u128.html\">u128</a>&gt; for <a class=\"struct\" href=\"assets_common/local_and_foreign_assets/struct.DebtDropIndirection.html\" title=\"struct assets_common::local_and_foreign_assets::DebtDropIndirection\">DebtDropIndirection</a>&lt;Assets, LocalAssetIdConverter, ForeignAssets&gt;<span class=\"where fmt-newline\">where\n    Assets: <a class=\"trait\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Balanced.html\" title=\"trait frame_support::traits::tokens::fungibles::regular::Balanced\">Balanced</a>&lt;<a class=\"type\" href=\"parachains_common/types/type.AccountId.html\" title=\"type parachains_common::types::AccountId\">AccountId</a>&gt; + <a class=\"trait\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Inspect.html\" title=\"trait frame_support::traits::tokens::fungibles::regular::Inspect\">Inspect</a>&lt;<a class=\"type\" href=\"parachains_common/types/type.AccountId.html\" title=\"type parachains_common::types::AccountId\">AccountId</a>, Balance = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.u128.html\">u128</a>, AssetId = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.u32.html\">u32</a>&gt;,\n    LocalAssetIdConverter: <a class=\"trait\" href=\"sp_runtime/traits/trait.MaybeEquivalence.html\" title=\"trait sp_runtime::traits::MaybeEquivalence\">MaybeEquivalence</a>&lt;<a class=\"struct\" href=\"staging_xcm/v3/multilocation/struct.MultiLocation.html\" title=\"struct staging_xcm::v3::multilocation::MultiLocation\">MultiLocation</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.u32.html\">u32</a>&gt;,\n    ForeignAssets: <a class=\"trait\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Balanced.html\" title=\"trait frame_support::traits::tokens::fungibles::regular::Balanced\">Balanced</a>&lt;<a class=\"type\" href=\"parachains_common/types/type.AccountId.html\" title=\"type parachains_common::types::AccountId\">AccountId</a>&gt; + <a class=\"trait\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Inspect.html\" title=\"trait frame_support::traits::tokens::fungibles::regular::Inspect\">Inspect</a>&lt;<a class=\"type\" href=\"parachains_common/types/type.AccountId.html\" title=\"type parachains_common::types::AccountId\">AccountId</a>, Balance = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.u128.html\">u128</a>, AssetId = <a class=\"struct\" href=\"staging_xcm/v3/multilocation/struct.MultiLocation.html\" title=\"struct staging_xcm::v3::multilocation::MultiLocation\">MultiLocation</a>&gt;,</span>"]],
"frame_support":[]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()