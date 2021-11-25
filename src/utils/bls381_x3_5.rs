// https://github.com/webb-tools/bulletproof-gadgets/tree/main/src/crypto_constants/data/poseidon

// Parameter for:
// exponentiation = 3
// width = 5
// full rounds = 8
// partial rounds = 85
// prime field =
// 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001

// Sage script command:
// sage generate_parameters_grain.sage 1 0 255 5 8 85
// 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
use crate::poseidon::sbox::PoseidonSbox;

pub const FULL_ROUNDS: u8 = 8;
pub const PARTIAL_ROUNDS: u8 = 60;
pub const WIDTH: u8 = 5;
pub const EXPONENTIATION: u8 = 3;
pub const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(3);

use crate::utils::parse_vec;
use ark_ff::PrimeField;

pub fn get_rounds_poseidon_bls381_x3_5<F: PrimeField>() -> Vec<F> {
	parse_vec(ROUND_CONSTS.to_vec())
}
pub fn get_mds_poseidon_bls381_x3_5<F: PrimeField>() -> Vec<Vec<F>> {
	parse_matrix(MDS_ENTRIES.iter().map(|x| x.to_vec()).collect::<Vec<_>>())
}

use super::{parse_matrix, PoseidonParameters};
pub fn get_poseidon_bls381_x3_5<F: PrimeField>() -> PoseidonParameters<F> {
	let rounds = get_rounds_poseidon_bls381_x3_5();
	let mds = get_mds_poseidon_bls381_x3_5();
	PoseidonParameters::<F>::new(
		rounds,
		mds,
		FULL_ROUNDS,
		PARTIAL_ROUNDS,
		WIDTH,
		EXPONENTIATION,
		SBOX,
	)
}

pub const ROUND_CONSTS: [&str; 465] = [
	"0x2bd65fc0d8e846616ed551992c790b099757cfc693dea026617682bfec20581f",
	"0x70c252b59d870074c9229b48f5202b3240d62696a10363c6a2a6f5faad4f38f7",
	"0x102b247cb596131d2d4b6335b0424e78e2d6dbef4ba118c75433b6fcb691f883",
	"0x7291b747651e8ff03fd804ccf7ea7a51d5fe50e454088cef1033862003b37e7a",
	"0x62cfd17c0bdbb7b32119d8067d2c93ed9e9520e09ce2cc3997304f8836031a12",
	"0x2725731e06c466e4d6a95e4a95015ba0c679bdb3bb5a641b39d6b115d9d62141",
	"0x51f80ac0f575f875d9d73d7e8f1d7f2b5f6f7d4344a273cb875c0e1240b0d955",
	"0x39f9481ea5cac9dc092d304a66ed886ea6e2ce78bb983ca805e5fdc96bcbf1bd",
	"0x53b79a1ae455785c1f94d2420e8ce4a886355e8ed20ade8ef5bfdc366ff79cc4",
	"0x0d2ef5628ac1a368e28b60b7836051d6fec4c722253856a7c4b046e5423b0620",
	"0x266aa4fdbcc69ec4b7a664d702936bb7c92ba2ef990c9e7a2ae6526fcdf86036",
	"0x3ab96943d5dfc7cbaad6288fbb36023592aa278dd896bf8b3e2caee32a3df713",
	"0x48e100cbc0b7f738b73ef5067bb8a7ea1ac4b7302f0e60e0103885ffb4c643eb",
	"0x008767cac6885a1da8dd73cf5ec5f5020db77d36f39e3ff815689369a9ee915b",
	"0x13a121eac704ce372afbef99c17113b367ac9638ebaa848572794c96638b576b",
	"0x47ece3dcd0bd4fc02d537794d2bf0d9dea5c1381f8adcb65ee95ff9bb5c78dd4",
	"0x6c7dd9953ff225698ece5ee65cb287ec455079ecd618142bfa63b04db1c718f1",
	"0x4fc9bcaf5a68a507faddbc227847b28ff36fbc5173dfb354b8d4ab11355159d2",
	"0x3ebc4a8051f075a152ca1a807268c6aef3222c6cd0dfde33c83476a3bb18a368",
	"0x4f5649120daf03e2ad5488a6553aa4822c1a7ebbc7adbbf8caee711bf73998f3",
	"0x6deea3ecb0c4b0d6232991aa745f88bb7162eaf6aef32ebcd20000cb45b62177",
	"0x22621264efbe5f8e5f3bb4312867a60bf61ecfdc9510246dce238cf47a97c720",
	"0x1db9cdf5fd6a9867ea8d3ec039c781e19e7a34bc80f21756fa145edbf72289ad",
	"0x411dc113442f5af16db1436bdf29443edfb03243ed11d58e3a4732e9603f85a0",
	"0x1df7fa3f038b35b80b16f569422c72f3c1a2a9057b3da40c97aa8c37a1ba58d7",
	"0x3f0ba69b8a2aef69788eb7b8eff02fc11e0f151c0545c8ae4a84e77ab5956700",
	"0x1d4d9bc61ae19a641c398ed73cd632d2af2038b950656c4e785457ad8d6c59d2",
	"0x5a208ee2cac6b5a8c159d61a7193f19a81cae39d6dd3fe9a72e3c68c80e0721a",
	"0x4c5d72a825996215a73c09e57deab8948637f03b2a77dd78ab2f4a45cca8e83e",
	"0x6bad1a98bae5ad67bf4eeab17796cde4fa61d637370e3e6dd64d1c86624e6128",
	"0x104f09cffc2db1ab8fddfae64f63db27bce725987b77769d3e9d3dd046c1f05e",
	"0x1b07e97ba618f6f29472814cd12aa2fea49cc75c7f32a8034d4098d59e5387d0",
	"0x62d058a2e0faee2a5d9ab6c86fce9eacff7031e0ed22df957a76668a5b14880d",
	"0x186f07147c7bf96ea9503edd582a018fffd7ad517dcd9ac22982302ff7eb4d29",
	"0x449e17eea5c2580aa3318c269474e0a0e0f0a665df8cbc8751b601c3277795ef",
	"0x150f59e75e967424f8e995fdea004ebe96fa363160e600b3b06ac59ee5d38115",
	"0x145c2c9e7e24f9885aaefe45fc38f28e52341223d12724652208e27a5e1675d4",
	"0x61be793a56eb1748a74b8bfecc249606de5650069158ddf2bd0b8bf37ef14f4e",
	"0x289f8628f227ab24d9803a31a53980fe80768daec6680bcbe778f41012ba4f29",
	"0x0bf7b4b705fd7090a9060a6a1d3bf3a5ad4c82e7235220555309a624d4bd76e5",
	"0x1466e3fb0078feda77d0a9b04327a8c3455bc2c8597f68510216a673b7477655",
	"0x5469a15c55aae076593fff38fcc19f3abe32e735510af774a9cc1b830a5b128b",
	"0x564b4e0288ff6134bb45bb51b329b58f6d36b0bcb654163b438fff6507a0e8a7",
	"0x0b8d3851a465b9ce60c8c823359da542c2680750644e9fd7c6b113a2551edb8f",
	"0x68914b5c0ba27cffb79a57d205023e0d28002877a207cc1888fa3863114136f4",
	"0x62fb4cfb53bff7ddd4270765d345b2f415f061401626caa82e25526bd77e12e7",
	"0x419c64d1260f7571e18c0ebaae76ac88efc3b83999439f8fcd781cf5947d81dd",
	"0x18a14ccfd932835973387e77bfa4d724f3e35495ad679862291f778e0a0b78cc",
	"0x0d3b2ca69c8612bbfda3c1646d7b0d7f8bb85c1e9903246001344ea2f7a53086",
	"0x160c250b7ce4a48441e6fc6b6e49b88b495da2453a45455018369432c907baba",
	"0x00364ac55d18385ee9a58b179ba922733c9054d1032c33ed61fd5b2594f6d5bc",
	"0x638af1536bab3fcae9e348566a5c34d8ae22ede5b1794d94f0e4d87fd17f8022",
	"0x6cddb8e1665df40d305708f4674a86361fae66e05b7b6e299f4209def05667ad",
	"0x306bc82a4cb74c9c476aae13318fd1352b3aebb4e872df8264efa70e80e70930",
	"0x4b36d5074a684f262e8919268b6b7d1373168d592c5b950fc617a848c7a74206",
	"0x2db2b3e3c8fe442fa1a360941814531f1d0f4eb1b98c993f6b7f2d4f531d32f9",
	"0x5fa3b9503ad522ac6d5ca6890876ca5eb0a77fc05f7c1a1334acf0fff8221508",
	"0x13f79226abb50cbebbf551e7a21f29b6408754055ae0774e623258839180cf14",
	"0x30352d89f5ce1294f4a0579e6298d3e931851ce399c6a0ba4479f27973f04040",
	"0x5908b022fdcf793dfb90323e032e1418789c60f0bf289125eca972764c7d1824",
	"0x7335b1a113d3346112c63b43dc22ea76e77af5b1347d0a564fb4a3cd513c28b7",
	"0x48dc7fd715ac5bf31e7d05093bee6c15e968911eca1fe0f42d7254ca34f95ac5",
	"0x3a09e7ee4c04ba7d3e5e408ce17d1786ddff4d3cf8216a637eba8fc57109bbff",
	"0x5f5e5d82e7cd4891d3be125c634c87a55aeec1796d10c3515f54090be93a2a10",
	"0x2f702911beafc04444e6bff1707070b03a55114e9658e93eac8bebe8f3b8e369",
	"0x51208002ce807e80a20ebd4a2a7207a5e8e9948151afbc680e505e25a7f4e890",
	"0x4983fb4d257c2119a95adc86438e313008d52415d96499323a68775a4b27de94",
	"0x4acc8a156b3c8388e3ad1f9d4c993ee46b51e2be8e0e9bcb9af67d19c2328449",
	"0x512b31b79afad26e09a660ba612d1b6f1c9f2c7f479ea23f61e8f1fe8d26ee84",
	"0x64e240efbdecfef17cd9f7226ea323266666728c88d582acf5ab73a9a3cf5485",
	"0x58d1dcd6d124f491f3c088a8b627d29c6ab13562c55b7bd07e2a4b9f43e4a4c2",
	"0x208a1f7fe591fed5dceefb729d17d4da271d2ff5b6ac42ed1013d23cc191e33e",
	"0x08a607c352dc59eba41c73981bb8fd1c2ca20b5cefffbfb09afc40620ce8fdc9",
	"0x6d40d1e6ef6017dabbcc048daea7bc2c206fa2176c38a3a75a76e882a035757c",
	"0x3ae1ed3432676a4246e21434709d369b688df4bfb879d6083d044fdabb1d7439",
	"0x303a2b2c2ea5928407ece774f2abd2c524cc8f3b724910f4c196b83453cc5105",
	"0x3512fbc014dae2f345859b0e890427a42ae96d36a18f1f38803f854454f1d8ed",
	"0x73a27513f4af5602f82c19c6b367f114a2211641e2f5bea1cbb8013aa93e8838",
	"0x39bb9f760cbb55e593fb9d07b8a4b88f9fa0c2e97e5e0b7b602301eb747c67af",
	"0x62da2df6fc5e712ed5bec8e71526990adfdeaa95d38f5b8c2ceeffe89699a5c9",
	"0x1e38619aef714fe4c2351f46b7ab725493586d45392dbf860580da216bf0af91",
	"0x3dcc0d46194bdc4e2faf13ab011f35b0aa833a21301efbe98936b33e6c53eb44",
	"0x00745b14093a2dd41c15e41b7497faa15e84e328ce3510f51cde3f5cf45a1a29",
	"0x6f834f755c5f5792992b4addc12fb1affa9512541650a16136af5c111007bd3e",
	"0x480b129b8d0d02b1f06928d60cf8a7e0910c6afe1965f35e0c951d2693f6cced",
	"0x5af55fd91026faf7bafb1ce5cf1423765d5f41c621a3ee2153747e19c7a4b1dc",
	"0x30c204df98e5d48a8702d58f374aa0d591da44d6282a5d5e5a8b86e9f2e95ee9",
	"0x6f6e385c32899934dd0e1dc35acf607f5d838219c109be6ac84467e66e9deaaf",
	"0x1294a05acf82c1b3406f9dfb6eadadf270d9d901c3af227a6540fd81c31031e0",
	"0x384f67ac3e4edb064e1fd6572b9fe17abfe35afd8836a8e50e4e2a1464eb6d90",
	"0x182d12834a1ea35692ef6f1b77d7981542b9d37335a579462f9003f91119bad3",
	"0x5ce2478d5df48b73d20dbf1497e609227aa7ab29be0b65d95ffd1687fcf289b2",
	"0x3ff690f927a1064a48d0e5c1e6cf599c00c6940368b1909fcd13da59d4eaf6fe",
	"0x40f8b592af1add3250c3429bf415d5282a0121c4b437595cf4654097505a7eae",
	"0x4c632691f1afcfa897394f91c9ef744cbbcee7f6d5ef20e5c6f9b6ec29f9b39d",
	"0x4f384df411664dd5786a3ae1cfd0683923ada6be403ea8f8464d9433f7a0b520",
	"0x0858ab64e72f1af7cf1378183137df89db2c7ac167683ac01ce7f9c620b6b322",
	"0x3b0b490ff8564124b197b589ed0152eb2a348ef9ac2636617ff94529a1956f43",
	"0x66e82c6a93032209f03285972d273f781c481e498d594a5cc240e162de1e35ea",
	"0x12c39b3b769d98087db512e1e18abc9f6b72d9738afb0502be6447af5b01ca06",
	"0x37338da7dd12f4b5bfd6888b58cddeba7e7a99118d08187f78dc3f2b87575e5f",
	"0x7203634583701a84d58e3c34b7891e7295a4c7c5004a39d06c9aa6fddc66782f",
	"0x2c99904ec9640ffbc52b890c4dbaab4ecdb2a7da351fc92fa6439e6b6886d96d",
	"0x41f1bb81cc96d00b043f51e6fa89caf0bf4d2d0acb9953f74299dbceadbfc5b2",
	"0x2a62ac02d7c1b4cba4bd33aa15938f4aa861d18403f953a9d1580c1ceef37134",
	"0x5e4f0c3d39ea53aaa37047b2e9209fe2c245ec09cd37acd82790e37ef5c869cb",
	"0x6d7f51a67c9993fc421c5961b6ca209596180ddeb5fcbd5b64e36556c138a30b",
	"0x4fd81c799dc57ffb7af2a0d430cccbab9bbc2c5ed80b908d86039251e974dd76",
	"0x2994461455d2165e2fcbc5ab26115ee5e02d669e4f96e49fe5161b94c91c738a",
	"0x42273f38216c9f73cd9b0dfa07584271e7000de094ce3721edc665d60268f672",
	"0x7385a0c9f73fe6699e3289d89fd3de17c4f5c72481df51c5bf024edcee0b26bb",
	"0x2472c7f40500b40de5c9b0467894df0361e9c653feadb71e9b9172944b2cbab9",
	"0x23e1a282601303b8b7b0ae196da35fd449cbdeb316e3e7ca5799c7afb1b6470f",
	"0x5672676cb39fc929bf02bc53fee47d70d27cbd7cddb54824093d9e54bec954cd",
	"0x5b28d168567b8129efd980f45e6c8d0f0a4c756fbe550cb54207e47702546a8b",
	"0x689a7d2867e77a2c2a03d0a9760caf27fde3ba831a6784e49db25ee81b25affa",
	"0x3213e3c35a2f51873ca9c09660c8cd03b7f86ae1aff81ac5e254231551baf589",
	"0x39903917f204d0bcd1030ef5141a3297e0dba4d158c6f01758b15e2de0f78cf1",
	"0x6183f1d0c727bdeebe9aebecb65eba56345c9d2e9a45005c51cfd87f64f685bd",
	"0x4ad3aaadfbcec018d188a39fee218e6c95336aedf44c0980afb1b3e4f241427c",
	"0x4f36c5a05bef1b6f1b34e690b6219e6e76c2f7696958fd4fa553348fe9b148b1",
	"0x5e1a1d03bd703952f60542ba1783b98a92379fbb369ea1ac2ad237e9d67d29ed",
	"0x49142f727d8832815fffbb45f57bc389e941f9533a21d1982ab46e5fffbfe37a",
	"0x205a99a4ad6b9775e284edabe78295249e1da5d42025102626270a79f3b7bf4c",
	"0x0a22b7d241aea40d0111fd3b8008abd536adab90faa12ba550b4393fde37439e",
	"0x71cf523fdcbd17450a1cb19c6ed8647faa804ea3b73437d49f7ed9cbf0b6b775",
	"0x0c63ad20de489409cafd9fbc2b796b91b62a32476730f6dae457fc6d766aa125",
	"0x4e3f72c6a9a70002f0992e04fd03ca32a751661d6c59c9e7c2d1be47550d44ba",
	"0x5362523cbebfeb765181272f741ac728ebe5da576bdb2785ad2638ff73579733",
	"0x51408dfd617b3fe3bcc87100b60c18564a747b612b97a7aff93d3db6ee3ed2e7",
	"0x4a6a0eeecbb165d4b98ebad84fcb919e961fda11e48570c14c5e68a35b60fb15",
	"0x40cf317ab9d59535fc0be8308f2cb5becb64268c041e03c257b8990d58f3b1ef",
	"0x6a6758dd57da67564bff3ad2fe2da734b68e178d714d0d118eb079d64fe4674b",
	"0x294a294195fc075ff152df04c3d529fcaec6c4eac87e06a635950e2548605c18",
	"0x43767a0d0b437c651164988c50d464c1a783a965fe78eecbe16365d6c6f4a9d0",
	"0x53337528f183db9a1233028794f904303901062e68b23705d3fe0cf88a90b721",
	"0x33010b01b0214a10954933194b3dd9e4b43b40406c1cf0af58b0ad4fc18d0c72",
	"0x073970c4a17f6f1758894cf76978353d63109271d80d61491774fc9fd053b56d",
	"0x005be56ace131356d0294ecfb821a65a6a775a709c95cc85be09a92c38302d66",
	"0x6ddd7739e9455abd3e16c08f5493af73a335c7e8d1f48ad6ef83f580b9b2dcbb",
	"0x5579af75abf1e76bdd5c5e2ab0a4fc536db382d2e0aacd82dbcf6a3730bf5a86",
	"0x51860f8067317f3724b84fcfc33a9a5df85b66070391056b8637e3985262613f",
	"0x097bd08c079537af2bf986b42de7eb5fa84971e879224a3750fd305d6aecda80",
	"0x556d50d69c6f6a44527fc06ca308cb84eb8574e60efc68464120e2e9161733c3",
	"0x5eeb78b84b79a3419a2328b93b7407a8cf35eada1a4a980acd2ddf915d561079",
	"0x1606318a15ee66e491a0006acb9ef1c817f778249084019d3a04427c1414652d",
	"0x30ef19e81bef0502d25f4617598a3ffbc58aa89104ee9406f6c0675d26b9977f",
	"0x2966ce199d3d1f047aa703280a00985d01d0076fcd66f906b43fef9ddf87949b",
	"0x4308749a5c545fd988701a3370a1e1fd2f4760494e518f4b0e9d72e43a567d95",
	"0x4abd1841b60947c7e2456d7caa41e8d59a13845d0c84a8278da7e0a5639f79fa",
	"0x617e65cd69568c3964f1e5a2217a11c78409725a2aa08d25ef4f43c68be65e42",
	"0x08d8c167f7f3ec79b8029432207005fe1ec4cc477d0acc12a2e07cf1006e1ef4",
	"0x124d95c0d2dae3bcb14bd85c706edd96a01075e675d0693fa31cdf7f58e56634",
	"0x478d7de06f6ef3fa7b9d4c13c5c342958cbc696c5f5715089829bb12bab54b06",
	"0x1624d0a36fa59750c7556ea2f6c79cb80425ed19821c75a535f69860036295de",
	"0x1da5ae4c1eac8bcc12ae39715128cf16f23a1dc2c1d0bc25117f82250aebe3b8",
	"0x21931c26e3037f160548487227137738cd7c979779154dec50c0f4109f2abe3b",
	"0x09aa9ffb8df9caae15f71431c6040622ae1faa48d91684eff07f54edc18c416d",
	"0x630010f56c5b85c5117c8c038d6ec2d59c97ddbe3106e61c166a42a65130baaa",
	"0x5c139272e0f5699088389f200a29ea43f91e94d81442850448b18183623516c3",
	"0x5f2e319f909a5c0b18004e7a55805210d0aa2fe205d61f425abf0d6017de4dd8",
	"0x432482492c8f28ffe55637b40ac54022d97b2fd6113b5eb1cd54c4129604fcfa",
	"0x0ca7b9fea17dad66cf02d2cd047e71b73b5a03ace47bccee246c39c2653a8571",
	"0x2cdf7820d554f0f2ba3058d9a48c9f9fb9420938adb17bdb70556ccbb0d5adb9",
	"0x43966974c89d3e7778dd54bcafd8fb379344f50257f505b312be8b3c7203c0fc",
	"0x340f00c7b2df26fb4031c37e436a51b1b2c418b1c6785ccd35a345a5c6a08bef",
	"0x6454561ed048f93f134125b9b5ce36d40c7294480d1224db44e6f30f821921ca",
	"0x0855a96254c676e0aa16d095d07a81c41832329f33bb5fd8427d017a220a436a",
	"0x67315d286d25ea41e5a290ff6c434cb727a082cc64072f2f52edd656c3ddef3e",
	"0x3badd9485dfb64786fe9c258896847699d1ce20f7bf9f4d0988ee35b19ec9e90",
	"0x59c184839f187e9a38c281018776dd057a9548e636683c3da580b8b17be058dd",
	"0x3d24313b063c3f35896aece2f7dbca620bc475c952ab03f8612ab645df97d573",
	"0x733e0454617dd539b70dc4bd4ffb2d4faffc0c4d66e8e6a40083df332149e41c",
	"0x42d4cbce901151ab5129b46e8960941c087de7ac7a47ba6396cef5bf2880c87c",
	"0x5c307062ef7b4f03da79cdb99e939585ec5c1b848473bce6203a83aee24c653e",
	"0x5085c1c6c9144ed9e8b7b68ce997f0168b0d648cea7acb56ff551f8e25a3821c",
	"0x6290f0a3260cad8730444addd6769e3eb46f2265e302e9f5350cf5e304edf795",
	"0x2dc687c21fd646ea999f87587a774e68fad6abe2c1df7dc4b9e7fb4a8dc1c5a2",
	"0x546dc9d96caab19cc03685771f5b05ab6e3ac5343fa6255d650967166f73b271",
	"0x6ea62e5a86179c1cc36569151a7dbb76d3f8c2b6e6b805f5e461bda9afad6f81",
	"0x144f6e14ecdc1d410fb39aa0029da07ed48f6f89b0b82efba7fcc666fbd24ba1",
	"0x6e2269ae0564ea908dac2a0c68d3a8181fed9b6d1db579bf4494241800131125",
	"0x2d5481a405428a3be594a8ee3c4e45ad6b069dc1ce095756de7c3eef2c9e8133",
	"0x296f36fdf6ba3f1b7f2a5cfe3843004d65653caf571e1cb2e2c2a2d48e41216a",
	"0x6db789a084973f2c08f04810805992ea3be6ff71217a2e66a2e0c04acb576361",
	"0x36ff1ad137206849daad3405ed265e564ce3fae42dd33dfc2c171fe32b7c1b28",
	"0x65c3ac19cd61c1145f96311cecdf3566165704c1a52b74aa8a9b7e30432930de",
	"0x6cacdd52d59702a7fc2fe41de3061df9b39adb76d924f0312f4a11a73547cd67",
	"0x6ee735fd5e2911f4afef66041c2128e5adc970dc871731f6e68f076ff4078628",
	"0x38d6215e9789accc0591eaab75a5e0c86e9c1f72103a1b3ded563085ee4c8189",
	"0x0be9badb18cd6aa862322dc6e585c5f59d2d5cdd6d4c8267cb28b82df422db8d",
	"0x4e35138c8a289d49a3cfebdf9b3212cc0dab7e18b13cc18fbcd01fa529b29617",
	"0x169ab041839a6f6376a807eaec39957ac78b914721ea46c08808541d334fce1b",
	"0x52a996591f4648e5abb8fcb2cd3e69b55c9e95a80aac31274641cbf8ea662796",
	"0x19fdf91955f0e909bdaba37e37aba13f9223b7c5380058fa5e0a43fec542b592",
	"0x16a6e2154286f9a85f1eadb28fe2774803b0a4c9ba83acf6126e3708eeb418b5",
	"0x3ee271017488bcedb6248d700ea9219d6707c1a7149574e781485f613e1ec028",
	"0x1828175655d8e782430741e143e312031a1e32cedfbb194dcdde70c4281c336a",
	"0x5447aa9a934dbb559cbe96fd055875f4b6ab1f1d50a144bf8a2e2832171e8d36",
	"0x6b8a72d52e3b94058fb05ed3060427442102e2731534a6ad7348071e1316ff5c",
	"0x1a0ed439dbd925bf9ccfaddeb9a790ae038a696effc01d9ec11cd6d8bd7f7105",
	"0x2b431ded9c2824fd78e8a770473b5b26f5f82b4bd4d191f81ff47d7920337b63",
	"0x04332fb1566cd3fa9c7bb8ccb123cbc2a08409ee57fdf39e02ba8724daa7f88f",
	"0x17f043df2f1d71e5dcd5c0abe50ff58319afa486ac3ef0ff9e3779d8afa89379",
	"0x3e1afcaa378fbb54e30ae42a8c5021e532089ef5e1570e5d45af119dc438e6e9",
	"0x26e0cf6665ac55bca9d0884fb6733d441fe9c67691128303a8fcbbc6dfd653c0",
	"0x6e4c877b121772aa0ce618e014baaf9815c6ff32e5801e951247ebef7312f355",
	"0x0b1655a19837e1ce2e4e53a35cbb501eb6656587bf8fad01a088f70faaf36400",
	"0x5503d07ea5d90f8d1c5512206092b749266db861d500c96afcf6285563d2571d",
	"0x0af6a68b7ba0d447054704148a4099de5115593a5a598d55e8105bb4ce0675f2",
	"0x071a8c2dbb902104da1b51869bcd00cc407c7e9cf503d90bd7a5104a2311bc1f",
	"0x6e7285f8f5aff8321cb66dc60ef289fc771abbd00d6aa48666946a65cb9ce96a",
	"0x42150781fa9fc6924fb1fbc8b6342151cf13d20a75e00e9c04f4b7064d5e1156",
	"0x724b84f01642d3bd03b2d4460afac3a29a3a88ce9b56f8ad6b7bfa5d8051131d",
	"0x3a44c402a0ffd67b790c499e73ae823e677958edd7a06b444665f6dc69d9a24c",
	"0x6dcb7fc548db7848807a0da131a93bdca11c27e00a91a933d452ab944cd3015b",
	"0x016565b5a3f9034237bcb353807556503ef404d5602a696f5056b60cb3142394",
	"0x5d2574bfcdfecb0556dec262d5f563d553c9aa29b981c62f6932d91fe44f9888",
	"0x0680a00aa7d7f3fc5a234ba2d6c9a138575bb6381098365b6a1bb22f47971bcd",
	"0x3f34358dd2b7c3175624285302b88c9fdb60bc573039567b2daabf665673f727",
	"0x5696aef7a312186bff741e9e5ca72896d9ee19f66af695f886f0c93533d62efd",
	"0x63c590a346fed9f413d59b940bfdf4a90dc2b679175729ab6301de203f928872",
	"0x1f21e04bbaf3e8f096445f33ee3a2ad72f5c703aef6f925808534256383564ce",
	"0x42b0d77faebc1234a87884d8bb8e2875f8bc6eb2acc9d79cc7d0fa0743456871",
	"0x4324ac0e76c14562aa1b4ef3fb215e4be3f63f6133aba183058b0854284de817",
	"0x49cdbfb08132b1ff7eda552e8217f727b3b1069933abd69fb63bef5def056017",
	"0x50ed85d43d513ac0e42c978af7f9678ee4b2f6d00a6ce828bd680e8051ce7301",
	"0x34b91bc632d42d8a6dec660e2f3d1df75606709d2e85720cff09790a9e88bfc4",
	"0x4bb836a6e19d3fff4f5bef060a82ccf2f3b209c7fad396a3c8e5ad381c3cbb07",
	"0x276f2e13d4a4bfbb3b3402ddb47ffa41575db0a737715df456675ffd3ee023ed",
	"0x3dfe1f14ded08d89bcb8918be2e852b2865b16afde671043277fb1880b71212f",
	"0x0b1d24a7853a3df2db39f9743278b04241fbc50590bb81e928439015e3770cd5",
	"0x0ecc35f30f2a942da18c0215ec12f2c2e720b58c1f1156068b10274bff9ef103",
	"0x2813e7d51712d9708dd9c225a9bc7da8ab6cbf394fb9e3b6c5f719e16944c6bc",
	"0x6c40fab57d374b2dda22559b68d5a7de3a959a82cd976facfc6d7717641d1b96",
	"0x1875264f0c2e96ed1966e68e76aa360c974ea38a95c8288138f0e3f9c7e43b6a",
	"0x04c573d8a3a6ca6c63cb0fc64360dcbbe2231455f9dd7dcd95312a65781a639f",
	"0x366e0df9685a5eb018bdd51b9684c1747b0015aff6801202e420d86c1503cebb",
	"0x6673491689d9a92193c562c1dd5f3e87099866c5f0b7d9e8b9c96baab1b1e1fc",
	"0x32700d81d1264402e8e810e10fd83a66de225297616dbd6984711a1be7d5a741",
	"0x5a166abb12094b7e4f7fba2fde7f89fa2d7be4e996da165a3e704b880d9536d1",
	"0x2f4ac2dc06097dda4114b0c78ef44bd16963e1137677d1309100fe586bddf801",
	"0x509a9a9d9ad9bb87af5d275fdc5543124ac7b0c7cb8a629a8dae9fffda44317b",
	"0x44048d6e48e6a63633734043e0f6f70e35b68f5ebe3f68bf8438e034e2f8d246",
	"0x45013a8d9682eea7b3d3a74b228bcb7197ba652674a81df254a9e091737932c7",
	"0x11da56b10378b2fdc1e22fb0760c8dae8f9555a8dd78fa6f60ab9a5cf7e51333",
	"0x6ed5452d70cb412edb05db985c47cc6e61fe4ea3232cecb386871b20d1f7a20a",
	"0x5f03bdfce30df7f7d8572ba31a38022ae0bc6d1eab50cbb9e378b17629967a90",
	"0x2e8d22870333af399544e42d809c8af298069e934169ed621fa227f599ecf029",
	"0x0b41c4ccc37409292bb85c7dea4fc4fffefcc8c364080cf6bb0ba30399878da9",
	"0x22d134e1635595f8f099235634b187462e74f691f4510e2eb79f6db5f53fe182",
	"0x29d3198b1cc17814cd1183b3a165552e0f8977d9097e527418602e5e278c68e3",
	"0x673d0d40da5ee168dd3835ac2736edf0369ad73a4b025bfd979abd3c30d38024",
	"0x2fa3a80b9eb83cab31d0f6fb8109d308e136fad46024a7720310ab4200e69093",
	"0x3a550027db31aef402c483a94f3f92bc886f4de21c62b233e7c4e9eb2ea1d6e2",
	"0x334cb5cd97b08059c5913e3e590ef30b77ef8a525abc2707c012cef370a15bd0",
	"0x5a152317754a4805e64803a65e8f4f30008df0d2a4102573a7d259a7af52253a",
	"0x3536a84f9a6471a61fffb4e42a65f5b67fae207916d70b6c94f5ad874bee5f7e",
	"0x002f2ab39384153acb6931ffee51d01fc7475a27d4a1eae9f6def2462d6313a0",
	"0x346056a0b4cfb001552ae8be9a130f24e29bcbb55b3abdfab900a6c849fd821f",
	"0x069e38ee05f6b0196eed1ffda52a73273630f7ab281781303649b8ce2e8a3939",
	"0x45fd90db063ee5373dfd06cd1d73cfd5496f9db2ca67b5391dd4150ad22c18d4",
	"0x64908d40f223dad70f1b8dd17eca9dd9ad358e08fe3defffdc3634e97e795af7",
	"0x6de097da94a5cdb896f3bb12fd3708ae3048da50122a529b9f0c50a244f896a9",
	"0x14ff87be57ffae5315bd69aff287f7ec2ecc982b6002c049984af7160d1e1182",
	"0x19ee2968276ce2805786be31cd5b299560e12c6f426fca87d1ffaeb6fac22c2b",
	"0x566e9319c70ba7c57dc8a728c9495b46cab29e5f0ac19dbdfef5a4c5ad92f2f3",
	"0x492b6a8af96d174e33b292ba3a2f6a39beaf10c8e3f4218833ff836660da51d1",
	"0x42a264c34061e2e29dfd45f6662cb32da2f7c55cc329a8f0c2fea99c4ec999cb",
	"0x08ab960a103675667c47c7a437a30aeac87f8ddce03965d5697b0bc2b0119f82",
	"0x466755e9315ee0e8aafcf35209ac06e41537f604ec98fae02592edc5c73e5497",
	"0x54c297dad28919aeb276d29dfc127ba7173dbba7e8a8376655d457d4b5860797",
	"0x044e879ee4dbbc36c513fc85e432636fcaab8efef038542d724dfe4c74667d08",
	"0x09b0bd247f0df5af8cf6b4bd568f4594f2e0b181c20c41576492251e188896d5",
	"0x14590b54334bea5efd266e1a47c8d7f576cbb8fc5154d7ba3891d5f3c12baf1b",
	"0x47c9a848eebd2f50e889d6de2c6f6016dc46d31480874b86d7b3b7b636fff33e",
	"0x0f48c0298b7f9e9042f954e067082e0f570086e3136e9b41296cdbcee033e6c0",
	"0x6ce9421529ae6bc95a138510485ec3b7e7bbf34fa57b7b1d68610d819f7be555",
	"0x6d30d1f468f02c878da04350d85924794e1ac3f49e167b54963b41a6f5e4bd92",
	"0x2c92e611e78f6d1ba6e32a3006623a01ee79cccd6b13dbe66da3563dc09b1101",
	"0x5f3b5e8844fcc12cd7e43354e0fd0df910d92db418cca12b25516ebdba9cc119",
	"0x1a66b26afca21d0d7fbab8b414627bead70182341bb9e5a52ae5e6842fa901bf",
	"0x380993cc9d7df6ae795a1c0363f90cf938f07b9817c0daf031112bef700d6e1b",
	"0x24aba0c0afee9c0301e821f14b64a07cfc41a8f766448b8691bd69dc03c5a090",
	"0x09a4eb7a24e76be9f91a8dcbca40314f6fcf7f6a8a9ea5b78ef7a39f6ddb1964",
	"0x0cc16b201a5920f4e883690213a6823694224207e2c074337352fdc189e7ee0c",
	"0x4c654cef5a446a0d9bda191585fff95bcf59c1c2429d1811bffe3778be22e538",
	"0x58172a75e9756f6ac208df785d9d16a033365196b85ce224c88bcda1e980586f",
	"0x2ff8c86fcdc08e9da2f59b2f68a3d4ff326ebe322492b9bf34dfd845357b57e1",
	"0x5c4b5a4ee34f6bc3b35a0feb7e5cc9d7407ccc5dcba74e6fc1ecc305ac51a4e3",
	"0x06426d2c08653e4fecf649cf2168dc9b1e71634fb240bf14c1bb0d2a7093c716",
	"0x302868e57bef1c4a03df9becb1db2c8d18df0fecb7ce8f0d40a1651d6456bd87",
	"0x5fe994cc46ea4d1d4e59493beb04f26090a9f9e5db7f10519b4f8a8cde128fbb",
	"0x293eff80810d2535e31faaedeb9ba80bac7310c00671143ddd6059602e37b063",
	"0x3192a2d041bbe182a759101f39d14d3391afdf4016eb09743fdb27b46b693319",
	"0x001cd729876361e6f7765d2cda542ab2502cb9c41f80f85fd3cb8c83881aee91",
	"0x11a7eebbe6bb8952be2d29f6218d4583f04ad7d412e5da44308cdaaf245afa58",
	"0x1104dcd595ad8abd7255b6347a74e2bdda3f904b9022820b1da1c169f5cc0a9b",
	"0x26fbadc46c949c621a32a2b4a9fdf632a9e4d3ed36fc791a84e3bba030263686",
	"0x5b75e92f15fcbef9667026b4188ee751a869aeb097424a08cb26ef65f21cd5a2",
	"0x6037a1ddcf25482e69e1a67394013e4c47cdc9887b4434c71beac58fee0c3b5c",
	"0x1ac78d034110f35b021bd9769e482f45557ea0d0c26bd3205eae386206009d1d",
	"0x0df89706cdf1957e26edb0cca54418b19f99a1fc703bc7fda8aa8eb79470c340",
	"0x59126836d0f013be982f73aa9342ea2e7abb87f7729f3b7bb3a31db2771fe666",
	"0x62cf4acbf413682dec4eedd81d6c5b100b6ad221c92d59d3ff2196c169fa04ab",
	"0x11c1ffa204282d40273e42eaadb9a019acf298542f2ee2d675661b9a9d8301cf",
	"0x331a1f7cee6dc3da6384cde97487d9e3137122927a09635b35203484133d2df5",
	"0x33dc13867c2ed6865ec3bb3085392f6931aab2dd58a12f6ae266386ddee11fa7",
	"0x614cd1609c7f8b04d4463777c90a1eefadc969266967b0de2d1dcdcc0557c271",
	"0x04f389f75cf5e0be10dfa1b9b8fa2d9d97759115f71f4f63bfbe894f422ee4bf",
	"0x24f492336080a81430aa983c23b544ee8f84c1612fe97b19b76ec74e558d71a9",
	"0x6f0cfa1fa27ca0cff9920c11c64f57bede4afaf9b01f7f689c3e607e5ac48ffd",
	"0x2003444fd1de65815da8294c8e24e9377c2436e770cca49d4a81435c51e6a70b",
	"0x355937557a6e724820288621f760b14b83118b6ef4218536f4aa660a495c0750",
	"0x2443f1848f726fe4c39f2f83c35f83c1789894f2497eb8fbe9fc1e8423ac3b81",
	"0x3832e0838354d55adeb45fc96cb3aca6b68b75dc73b0af058ed4e67b143c148a",
	"0x0c453b897be1e29c3f5cafadcd1ef1819cba3c6fe4b57ba4f8ca886fdbe9fbce",
	"0x582912ceab20e93d6eec33ae9d6ccc65320893188328d536b3054d98599022ce",
	"0x2b60acb59d3bab447826470bb157652a8f74a021618a916422e5c265f6a98317",
	"0x6276f4c3d020306a58568839b6968f0f62a6edf491ac70fc49b739ef9437ec13",
	"0x6cf6b9b50bb68fc8082bc63a1b8eeee1e8a17b06f51166f7ee4af49894c2daeb",
	"0x45f3bf8fd8fe24bf8733ce4dca041f95bf2d7996764f7c34412c15d2bdba6c32",
	"0x043c36500e423d7e09a0b2cc15db43a2b8f1daf2655dd85a054307f53db53058",
	"0x68131364ddaa8b2b2367aaac162c87e9a336bc79d006944900370607ea23f1bc",
	"0x6b0ba53a362d76fea12003820ac457d6f8d326f9bab5cd75194298518f3ce4ba",
	"0x2740abfb4d0d43fbd3171c9d0ea743097e6674726ed9b96d53886037e14e6f64",
	"0x2aa7d5046ae9a47d97d27be3c7711fc0a25b4e56fcafabe8b3f23b880424e1f2",
	"0x5897b24501b4547609c601b28d879ec890d41734338d60e00560d0f13184a504",
	"0x0041374f626bc58c61b4703df407e8815623db0d8ab76786fc8fa3319d91961c",
	"0x4aba691463f7c38b688e9809201421eb6c89417a970840cb649d1a8019f3d6af",
	"0x1811d3185e6d3b413ffe7f052c80b871ea28d9e94a2ddedb11d84a36b9ac9467",
	"0x0c2ec5395a3b2a9a78cd40b9a1154d781e742e8a1005ce70451b707774775039",
	"0x58f9271b8c4e67978ec5990bd586c9c374110194631e5695117e0274aec30430",
	"0x1a3140afdec03191fbdc5d331a3080d0aad0e910061a2f60fd4071dd61e5e738",
	"0x3c57a18359bd8d6fc812b80cacc2acf59835203b2b6b5f6eba43caad314b5404",
	"0x078a5bbc7b38cb1903cf35312f93b8d1e1c27200078f02fc435851b9ff0a7f7c",
	"0x05ca6644c444d22a6ffde6eebe5d6cb2ef64f7c6aa1d1ddcb2d098afbfa56ef7",
	"0x6570b38d76fc96226608e0bae29b875086f3a509eb1bfc92b8dbd4582328fd3e",
	"0x63a3bf6801474abad00e70ad911df262310e26157ae72311012b3fdc595295e8",
	"0x49500f22d57d71cb80fb4002d936ecad43ebd7248636778dc9d0c6c9761fe494",
	"0x1aeb78af6a5ba49f74f8c6ec358f5b3ac1d7ed1db1f801671a2f53e6855f65db",
	"0x373bbc7d1906ed391982c6a43a4f73288326bc5a8eb6842f5ab74f8824248c7c",
	"0x61f3dc1d95f5adeedc0c615e16b354dd294d2389d020f8e674c0b27c3037bc79",
	"0x16c35f0a954c8395ab683e69c181b063b0a0d68959f629e36a96b9689efc08fc",
	"0x44ef302667cbf168b33ba001a17d9de1bb3c95d3c385c5effd8daa0db329e72c",
	"0x57ec2b9498440d5396b53c8aae076231985e5cba001bdd76bedb9598bcf26d9c",
	"0x22d633f7a61eae208aa7f5e02283f4256c28af7edd2a5bfa429f9a4c54f7f15d",
	"0x4ddba042420f44a23d01f14fd9b2631adcbc606ce2caa396e85754258be5619c",
	"0x2ea86f4e4d5f93ab74cb2a11655383de4a5b677c46fc518e72e59b0f523fd4a0",
	"0x07f3e3c26debdc05809db830b1378601e9e93cde53ae4aeba9d1102f05aa70a3",
	"0x0ec1b3a9f9f55179b3c062e71d15be83d34ccac429a3043849faaddf922b412a",
	"0x07d05b592adca58697d122c27af913a3bfa1c35f99463f67e6f2db268404c68d",
	"0x09974dc0793231212432752c5f036bde26cd13a423ce9cb8f562e6693ed73086",
	"0x154b08bc21c67dd0589fa5fc0183737e0466ec99963178d5f3936a37e99e9cfc",
	"0x6fdbbcc8437087e49c95df89a178f5f23f6fb8db8c2b3736180c39525310a348",
	"0x057bdea53b1c0c2d5ad9ad88f107c8c15ee75f03bb730a4dff5fbf8ce67652ae",
	"0x1a091cccc61bb4af82fc3480adc13be725e287f49cc206c2b4c7c04478671ed6",
	"0x0f31384bac76394a954b8a0c9a15c18ebb77d6fc1a0eabd4ad0f2b5e25e51620",
	"0x30f166d396e62664c1bed4c13a71e3460e9e6917b685acd39ad004d062ace28b",
	"0x6c1515e66a13226c4c0a181df4e16dba44228f7ae969a212f0d7fa86f2a94fca",
	"0x4d9808a806fce69533c9ce7c1059b4070dca6fed7434ebfc5ccba03ca86575a8",
	"0x0a524bac82d1187ff976fc4bc746712a8c076fd59c68a9269000a1f9432a9673",
	"0x0c383c8e8b53eb5d1a54cc315afceac108249efed128d767626633202b2ff922",
	"0x6130921fc4fb6c230c43d6484515765463df71b33fa43db17a5dc562019ad306",
	"0x33b4300c8d4ba7fffbadebc3d1b28353c61c2013bad493e9a99c0ea89991a431",
	"0x6ae613fd4578cf6c5c7af5bfa9787579ee4ec002d75bcc094e7fe8414f6c7803",
	"0x096ac274f6ebe0a2f5ed23ec1db2d5d73b9f437cc6ce814a5bc427ae1bfbbc1e",
	"0x2cf537f135704eb8b49baf534f3cc31a48eaae16ae7800fe47796fa6829a37f2",
	"0x6378198fa987a501db1b3453463f048dc41986baea5f6fb5364fa6ad7c409dc6",
	"0x02f2630679701dbe78752f76d8ea5122210d087ded131368251438e6e5568df5",
	"0x4408e042608b1c9921b2a85a1bb1eb699f52e0b245f4c5b68704fb3dfeb67e80",
	"0x514fdc109649fc9293e2b03d5e28ba26248a1d716e4714f39a9fdb50f78bfc3a",
	"0x163ac4822cfaef41514d3fe96ec6bec211876e880d6025666bf2216c0e4a0bc4",
	"0x038a3570826befa17ebe7c740944168fb39b7336fc3360e63dfc2ab49a052249",
	"0x0cc0b82ddaf3a9800e74d6c27029808c7bd509842d3ac4266ae774e9a353aec8",
	"0x0852e09db4e92a6e597a50af53402ceb5430d02e5a0ab6f045af8c21e76e9cc4",
	"0x312c1af7817e1f4197ebdae605d54267f590ff08751af8f9b83f5c93837549a3",
	"0x006adc866ec5126ee8eba9b30c550d4a75ceec55b943cf4d4fd829463d48f5cf",
	"0x05fabb3baca6aa771df1d4d1cfb5d6a7ad92f96d5f427f83b3fa039d7b8fee73",
	"0x28931bd7befa87e210233f10b4edaa95ce816b7d0553eb1acecd7593d5222154",
	"0x69e49220d16814d860a1c5ca943ad1396988c9613db2070eb847ce407bfaf148",
	"0x58514766d45cab8a3242fd6ac8a9e07d5d16091abf5bd14eb8bc53c34bc6fac8",
	"0x6d444a1ab45f991c0920edab35abf76f450dbb0748eee3fe8f3592132aa8c7b2",
	"0x424e7aa8e624d9f462b179ab4faa503779783dd89c5cc3ee8483deb588c8252a",
	"0x1313d25d4ee4545b32d25314a0abdd6440fdb8c9d1a271a9e5b31d1b540b0b2d",
	"0x44b589688661cc2d8350e728c8c708576e4a29e4124f34ef92f9d5fdf24b2fbd",
	"0x03415b5a40e145ca65522f4cdfb13a7dc09fae5be9dc974edcd9ba93cf5d2e3c",
	"0x2e2fcf93b436e990c5e7755b0eb7f5765dc2485635265d9e0fe151f1acfdcb7d",
	"0x70fafc62f3439457bc17c521fda2829115dd6ccb8c075b9c97089b257598e045",
	"0x455713c14e8a1e30b80c321bd99d38a875c865d26c899bc091465dc7af11687c",
	"0x25d87e7499395fa918e99bfef6b252b574f8f7808278d641b5d7a78a96b8c07b",
	"0x648f59c3104104b3a70c773ce5872ab433c21afd8077c1f5944835373412616e",
	"0x56d697ffbc88b70646fdce3c8fd87f38768ea12b7cf9ee1f57b2068d106190b4",
	"0x22d2911213722d16fd9d16a027b55914ccb0e1cb0c9ed1c073413ad2a441e1e5",
	"0x3393a1ea6cdc6666b9ae12dda74accfe95ec939eed05f5870ef47aec5381e5cf",
	"0x50fe6da5831bffac277c58743ceca82ebb1a5cfcdaa32ed95bbcfa118b5900a4",
	"0x057be43862bf518dd337409f82d37b47d3d604642170d062277e0eb946c1a520",
	"0x1aed68602838f6fde03c7c80d651b8a3abe93c208ce8f7daaab3f7edab34e239",
	"0x3a9bc544e0fe626d5c02b62d29633cc9fbf6843b3c2a0d0cd7f439c52181f4b3",
	"0x01a9be10d9be337ad657f79b72a0f6d68bc3698e4ca47948709b1533dcfa6bc8",
	"0x2fd101e6c5d462f51a114e0d8d419effff0a8011bd97faae6b144a256e69a9c2",
	"0x18fd3e0ae9fc2d916245b125525b6abb791f5b54ac67775ebc5864ff29c2b7a9",
	"0x1bdabdfd6df32eecb6c5e6652ec2eea3de5cd81329d8242f473bc794e468ffd6",
	"0x739d2d153f07aeb94a1029122b4be38bd49699548c94519d5a3686001ce11da9",
	"0x59e775a9ce25a4369fff1b88b78912d421b4d4c6b99d795626d545ba08b1e553",
	"0x0bc287cc537765ab12f755b37c710d9a25ca25b18a1898c4c242b8486bf2ddb9",
	"0x04cfe959d50c5c6014759185aa8a0d271c6eda4b57a9824b2cfd8f3773cc6256",
	"0x6a9ab1bf23630fcaa434ca2dc892b802aa8df698ca0f9c77c03483b1375e9af8",
	"0x71f8bb13e1be7d1916bd1e5769096d04a01e431223f5e11a08b526e660de8426",
	"0x44145278e6b70ca9c5def02872fcaf693997cca646bdd8884fd776a0ddf48eea",
	"0x0da0c5d9f35c539c636043fdffaa4bc6416937609b495ab27cb74ecb4a15a7f3",
	"0x3ac3801af1ea44ab127a5dac0ea85c4b475a5878960afbd817c2b1ccbddcfd6b",
	"0x14ecca88a7fa626a66864156ec220ab1a9bcbd553cab34c71a8cd539be15f44d",
	"0x1b2553381ef71b723d9acdac824f44fb0d376f9d499e2bd4352e042a92a34114",
	"0x484d6dd247fc6ab53825718c4d1c361c360aadce182ce664316276fc9fccaec5",
	"0x61938f88250d985b7566afd17f1e1b05e1aea7257f70926f256646709e49b27a",
	"0x3d567918a51ca139af5db1e7a873588a913c0809756ec1af5d70309982c36a2a",
	"0x35027ae9dbfd6876fb00ea41d0f4747ab9ad35e22bf826438e12986b056a7439",
	"0x6c818c519467717e28fd2f55271e3883b0be305d80969edec5d0f6371d3f7620",
	"0x120b61ea1058ff247398621f2abb413da776caa940837dde15423700190f9061",
	"0x385631fbee200a50340bbc4fa3f57bca6a8aaeaa954010ae6f10d0c6cd8f0587",
	"0x23cb55bf327501c1cde19dba4a96ac01ffbec6be8b96e0c03aea0078acec983c",
	"0x4fef1840b65739547bbd8bd253d386529c5246924af18ec33b94af51fd911984",
	"0x161c3262045b6c259ca657a14ec9f41b4fd0363969ccb729ddcf9a1ce8d992a6",
	"0x19fea81c9dab515594ee12f645e1d11dae1932bd3735828e83348b854f2757e0",
	"0x4cf370fa01feac739f25deed564a611ca72de44b1dadf5d78b16fc68702bfb86",
	"0x0acf927a21df9a663cda4ab81e333c28f9fe7754607288d028420a4445acfbe8",
	"0x3f05e5fec2941b0456cdfb3469a846b8981419c84d2e9d3cb9c26148edf895bd",
	"0x160bd9d2a16ef8102c1fe936412f5748fb867c82babb06e8ea14610294d6a227",
	"0x5c2ea3205ba69daf1ba9768df81ac81edfc1d987de259298f8a7cc407edbc557",
	"0x39c9ccdb91fd7ab517f480a05e4a0a9a0aa6c2a5325ce7267347a4e6c0424d20",
	"0x4caedbab22a58deaa12b8d0e164f3b44f5ad0ed4661b5e84fe51ad637590c3d8",
	"0x104e41bccd8be1bcfecd5a0f939811059f79977118fee949a5dea56c9c38096a",
	"0x5a561d098805cd3df15cbbe5c8c5815fe5d49ff4177d4ed2a597b45332deb31c",
	"0x15a9b2e654ceb5f708d54d80eac65e47abd8af93d095de556a9e15381e0d9f74",
	"0x2f2a460a7c0c18b58636bb40e7d958f01fca896d36445cf5c292b8b12d55f1e3",
	"0x571b455e24f06bc3491f87fe139ad5479154c1ff9b3fa6a29074e95ecf94ef63",
	"0x64e2a20e5435d5db089e60bac5c1dda0963ecd96be924ccdc7ce35c8deb82421",
	"0x5999339ebc0a6fb381bfc658f05b06d2c8b3db29262b689338f3d6e7c9e2cb44",
	"0x016a1eec7061397f97be73471b7f8bff58aa494cce94154f7ee421c6457caf27",
	"0x45a8889f4768523c8c87e6b90e6063c2d4af2fe6369f1c0180b9cfbd856963fb",
	"0x4ccf2da2f8edd2824cc60b2bbcc76fe197d3e4c91e1e2b41ea56ecc039a76075",
	"0x49f25bc0d465aaebe8f5f2aab79a4ce6b13839f50c69b8d0996e3390b4d85631",
	"0x63261d5ecac90c2485d2d1509970b389b9a902cadb74b720954cef911feebaf6",
	"0x5f253650c389ecbfb19f0f305d6a2e0665c5369f1140646bb5be09722c93d8fb",
	"0x68a105185e284bd187421ec31a87aef3c7a8afe9b597600193d8eded24ac402a",
	"0x57be420402cc35847cc66688bef48684e8015feab2c08c605da323b469884e62",
	"0x38b772bd57ef3ae6bac6d428ce405f8e0eb7837a06a06d9438a80c28c3855b6b",
	"0x69de31808bc25fb4d08c5cf82d79beb26c64d757ca62e280a7863a6ecffc7312",
	"0x1bf0388e44a3dc4780a27a8c8124405684d89a0542fee875819b0ff15f756f52",
	"0x0fb7087d292e3ff3d929f45c63d5c178b3aab9aa9efdbbbc526e2d8c2e10f0e9",
	"0x2b6d7871a65445dbadf1ea7846c9a97a5a028c36c0d5f472bdfb8e90f20d1f22",
	"0x2f371fc25cde32a8e6fb2595352b7191e1eb036d015ebad11b4e85230a688b83",
	"0x2ce2b75ebc237ddd462c16f492797f80e7fc79c1481b6e608ff2a2efb9e87a2b",
	"0x03c6473e7a7339443ddc2f0f1d136a577e2d7fda363ef5e44667049f5477ac08",
	"0x185fbf3c25ad6534ed43f5a404b6efa12471063e3e1dde460ec59dbe3f0c3172",
	"0x2d8de87fb693816b3adcb67f9cd4025c12cd647b35e6f252da6b8531c081ee95",
	"0x32a6321e27d60f2ad3615fa98aea9cab30f04b05946babfd83d85bb2ca0069c8",
	"0x002edf4a42a3e2eb5961e3af203ff2008bc3c53012c69b1e539f14653ce492f0",
	"0x595de2f307ce1943be51fe0b569b3804fc490d05d21e22e6b160284d7a8c6dc3",
	"0x59932700c7e82f172287d20a3baca002f5c9a9a65177294b9caa33861d001f0c",
	"0x4237b49a5e08693121f672f81e6505ae08d52f1de5a4f3c879cc70cda808d86e",
	"0x03cec89010c3435a48df2595f4e9e03f31fd475e02d7a628f67a59e53c589187",
	"0x3671d5693cea23e0861f68ed612f8882d762e41005dac28de7c9c4b74430a86e",
	"0x6643f9c302ec9d41a23bcc39d14fd87a7951e61b512c68fa1e4402493c6aeb8d",
];
pub const MDS_ENTRIES: [[&str; 5]; 5] = [
	[
		"0x14785ec147265a351623675ee09e6e4e6ab96d4b7504abe5e36bba99cb18d9c2",
		"0x054691d2ec3d13f116a771c56cd2126c43ec6d5a19ab11b0bc1324e2b5d7dabf",
		"0x467ad8cc7a9250b6fbc698f07cf158fd60a1d3170f96a9c3ac97fa1e28bdb2dc",
		"0x62feb51539ed985923a6438d6edc4bcab710c443bb1141486285453ce1ae6075",
		"0x6d72d28d6cf8d6d2870c0c09583cf2ccda2f35139519e3daa752a37d74fcd588",
	],
	[
		"0x254f1f66a5122817e515db9710c2ee900602aaa13e01a385fc37d21866115a8b",
		"0x0b76ddb36981c03f2d1f06bd93bff23ea875a789169e09f38090936e1ccc54eb",
		"0x13ef1c602cebedbfa02096a94f7684ba6746107f105ff04592fa4b8ee3d3e227",
		"0x3e1450c789b1a01552c69dd7b9ecf226e060ac97332c7de316e7a6f35e71d7d6",
		"0x260af6392ad4ff2a7466f6966149b604f5f948642c97e547461bf0021372f9f7",
	],
	[
		"0x650fc44d930cb27cc3641628694d5f7a7b3435625330c9e2d99bb74024627e7c",
		"0x3ea7714b9642d6a4dc03b05ddb610b91353f115081304b97a380d0865726073a",
		"0x15391f132ab4976cae2c9da263ec21fce55ef0933a5b54ca167ab8bfab70eb30",
		"0x715e204a23c0148d4a97bef8d928e0e28c8b577abcd2a458e5668f327b527864",
		"0x4149bfb6d68a6b18f539708ecd03de42f858fcd471f73b7afd0dd78f04a871ec",
	],
	[
		"0x16f0840ab3bbb7cb4ed56b2f69470fcc3df9efe476f804cd3c8933bd0dea91c2",
		"0x168dc763cdb02dacf2f465469b56836291e5c22378dbef7cf2ae32d2fe5bddf2",
		"0x58c7af1a8d7011747cc651dd678e2f6d5e65915dea9b2e36823c0779be3a198e",
		"0x2d44671a4a33edb38753ccf390355af0b72c6624ec43a1b7c5725cde8e6a2049",
		"0x37f8a3fd91b2d1e482937a2f7b456a8a614df90f23fe07aa9d1306d92d5a9452",
	],
	[
		"0x18f6db9044ed52d1f1c8928810fa24a2338b8a052a92b578b14364daed5b1146",
		"0x66caa72e44436d0b3e445beb199fafca76d81f5da052821ab8d108b1fb686f80",
		"0x2bfe0b9cd954e1cf0d9079c0a2ed44adafe22b39dc394c3b89e1b75c44b44cad",
		"0x3d803f6dbacf30682102d24c4e01a5db79d3a78be078e572f357b1d229693934",
		"0x0a32693b303d8117fa53c89d15ca4dd82e8e8600d4ca2ba0f5b5b754cbabdba3",
	],
];
