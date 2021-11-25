use crate::poseidon::sbox::PoseidonSbox;

pub const FULL_ROUNDS: u8 = 8;
pub const PARTIAL_ROUNDS: u8 = 60;
pub const WIDTH: u8 = 5;
pub const EXPONENTIATION: u8 = 17;
pub const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(17);

use crate::utils::parse_vec;
use ark_ff::PrimeField;

pub fn get_rounds_poseidon_bn254_x17_5<F: PrimeField>() -> Vec<F> {
	parse_vec(ROUND_CONSTS.to_vec())
}

pub fn get_mds_poseidon_bn254_x17_5<F: PrimeField>() -> Vec<Vec<F>> {
	parse_matrix(MDS_ENTRIES.iter().map(|x| x.to_vec()).collect::<Vec<_>>())
}

use super::{parse_matrix, PoseidonParameters};
pub fn get_poseidon_bn254_x17_5<F: PrimeField>() -> PoseidonParameters<F> {
	let rounds = get_rounds_poseidon_bn254_x17_5();
	let mds = get_mds_poseidon_bn254_x17_5();
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

pub const ROUND_CONSTS: [&str; 215] = [
	"0x2f519d236ff3908aeed8bc12b6e3204820c18780cda2aa9bf32a675722a943e7",
	"0x0d463eca09295bbf17ecbd6c0f1a2ae22e9e971abf332b50f6dc652948130ac9",
	"0x04f7b92e174673f12ad94305e211dbc74505cea331c8270bbc0436cfd3f9203d",
	"0x0b9ce1d7a1d5702d9cce533fed0eb4efa2d3d5adeb056a0bcfaa8df560e39d65",
	"0x05fd9a940ebf8c4f6365146a71cc4d60792afcfe5edfb289ef3a5e0ee7171df5",
	"0x0b5644601dcb7f359904a3be22a6726d88c91c83462e2a96eefcb2e0d3aac3b2",
	"0x05270c934663d155aa71e7bb2c9bcc6d8dc181ae02c779a1bde958eda5b61398",
	"0x1ca3acc7430ca47799891a4c459af74a965256eba9d6940381c1d0bf0a9739a5",
	"0x03db72aade768b78c7a6a9ecda0f0f2c6733a0651cf2212e174ea97c01a59c70",
	"0x1ad0aa205be150c5621b3fea98b78db2e4b20fecf80034c52e9ae558950dc780",
	"0x041dddc69336c714ee7cb998b68d94c5232d4e8d1deeca05ce83b0ed0834cd3f",
	"0x2b3e015174c3a6939f08df076b3ba73bbd4f2452100c1821492321175d162b1f",
	"0x238e6389e5fb2584c06ed2f534d4d9e2d5d62bb99dfe8add413d809fbdc754d3",
	"0x15f4f175fe2ef1101a4f0a34fb73acf4397c3379b1c2924dd94b1dd4d2c6b27a",
	"0x04fa37d0ac02626ecd1f192e28acc2ad80f3de4354aa8ebe87471757dbabc9e7",
	"0x24deab96a4e96719adf665e11c38a3334d9b47643171a7d814b9dc6fe7382d02",
	"0x0a2c7eb8738a61719ce05c72547cfdce9f6ac8770cd64e6556a06c7ae86610c0",
	"0x199482b63e5fdf185628c3046f0bf2f7be5d0f256ecd035bc212fc01eea8ffa6",
	"0x296cfc7f739a3cf4e9b75994056e2568c6aa52b8333520627bd6b28cf484de9d",
	"0x06fdb5a0e6f4e2c7e546bad6b1669db01d9d4e3bff4ee772507563da1ed92dca",
	"0x1216f1b2f749f561078eeaf1fc60590a71e33157d0227ac8e796a4042e945ba8",
	"0x193fa4aa12acc79be80e6a2424afeb3f4d249566e8a0908c908761758437929e",
	"0x12ad33b11810144865ae5391c8f266adc3fa0adb622b90c2ada406253e09eb2b",
	"0x1a90d5e1c0578c066529c54f8f37e23109de9159e298cb6c9102c6ccf581a07d",
	"0x15af9b019146d04fd7db3dcc25ce29d163fb604f5ce11b8fd507089a8660095b",
	"0x124aa9d20b2a15ec73fc6b6eb84b544edc13e9a72309ab080e362563881eb8ff",
	"0x183cf3432c6354e3bef6170a32fd653b560748e59b65052512125db3b542ca7a",
	"0x263159492b9a836f8d477e6ccf6f2dad9d4bc43eea7241f224314cd356fb07eb",
	"0x2ef6717639f7e6a2a0c60da01ce73721b83f6f9fca63ad632a27c5e0d561dc26",
	"0x1b85e4a891411168c803d411048f946a167f45edd0b98aea0e96fcf9796a5415",
	"0x1712b6d3ab45b45a1a43bbb23ac830235f8fa4c6c057be35b84e77ef366fd89b",
	"0x2e97c862eb257de5ef4f71b56bab12b449b3186cb3f4c44ff32c931bd9c3108a",
	"0x0500ca19f7690358b983978382d969729008b41a902b03c8254e8a4b863ebe2d",
	"0x25ecf7468c3d34ab9aced95fd7f966c7e7fe7ed74ac02c2f33cbf59d182c169a",
	"0x061493f05adc4813cb62ea6269e2672c241b4ce0e3658542e4bedc660cb832c8",
	"0x077d4ee2b3292e73eb891b51f63b0fc897ddf1095208c65a291f470391eccc8a",
	"0x2a1ddaac93b561971f9174611a06273671f80dcf4eb355636577a623f2267239",
	"0x28c9d37bc63119f5b846f969452d93c78743ed7c769590d1e2eaff2f7d51b636",
	"0x1c12d6672a7c9c525e1ad06e91be238291b6a452298c3f0907b3bbeeee33a75b",
	"0x295813176a99d708f66de0dd6a79790c1049113c1a9589f58e29aef2431dc3d0",
	"0x11cf1b0b429cdde5235a2fa4044a75fa29d464d422bcec4cd5a77b2a70996952",
	"0x26cd23abd3084eb97652a0bdfa0675107c27d021a797ffd85c3935035e82c708",
	"0x0c98e951acef5ecdf322fd11adf84e1237fd7a2856f5d611185677e064e59a7c",
	"0x0f65e0d787c7a80daffa6fcec8e4333baf1b7e536dc26da1617bf8d33fd571ea",
	"0x0a6af46169c405b7aefa2369aea2437e83a5f936e21b2c5bdd40c4c229691035",
	"0x2c5d17722f1e857bc53fb954a1c617be89f79f666e7d42e5cd289bd1f6c2c253",
	"0x05ed375b0a7340fdd67b5a6e02e9d7f8cee95207cc1f021a98c9f026b6b8263a",
	"0x05c43b6a0449ae8aba7f7ef495f81f9b1172e2e33d6b45641c85ac438f867072",
	"0x0632e9aaccb645ff09e9a71c0bdc98bb8525d16dc0808089288084dc6a37239d",
	"0x020115cdd5958aee185af3281eb02700cb22c680fea57e0e24ddcfea04e4ce44",
	"0x130ed2b41ae4fa1bcb39907d9213b816fb5be6f78c8b33f81037c6c82351ad6e",
	"0x198b2b80ff15fe2af3c414b89c4fab453017cadce1b42986a9064069b91e9d1a",
	"0x26a02e2e22afd030083fac312c3ea5ef2b8261a789a3cdb14e0f59344710c7d6",
	"0x1e972689e841a22940b918fb25a4fd47bc016bb627359e0f8efb982516900250",
	"0x1719a91dcbaa2591741e3c6e8c813071dbb249caed013b1a49a09c6337588388",
	"0x07ae0bd9a15c6093e83d203cee422cabaac2b1480b199a5a01f3b6bae8943c22",
	"0x0335138c841a80c3374731753ec839f6240bd2a965d10971c20a5573c6700a61",
	"0x0bccc625cafac996fc66d3d723e2efcbc7291c5c2ee94ec4308719f0f682bae9",
	"0x1aca016e7c2b5f0fa4a3c8466822be48e461a2f96daa05a7e6bd65447ceedc20",
	"0x228ca2efdfb5fc0371bef36dae53fb3e6743f7f5ac32773eb9daa1fff9ad5280",
	"0x0abd2713ba22c5a0284319c09fdb52831253c15a3dd8bec25ef562dfd902ffb3",
	"0x0c20a2af2b35bffc6c6b267df78c0ea1e9382f9508fa665f8fdc0836a4d5da0a",
	"0x0da1ed20c40a98057ee6df9f8e55d6e0936ec8b1ded7ae68ee97c9329310c346",
	"0x2b662b17eae86aef2137d9f669f08960d262f221708c49fe793884326c92aae8",
	"0x00892d01420c44d4b5266e98d3963b613ee8b005a2b1c9cd20082a3d49f595ff",
	"0x1e7068ac5f13d9a2e8b46d99b5d5f35f2d3de6de8bd81c8e78668ef467f05160",
	"0x04c946cb36ed8b5935362ac33029dedd33cd3d62f638eb74f1834b1fb83060a2",
	"0x0bfaea8cdbcbab42a9e8bf92141c329651a604055f555663fe983c1b0646d051",
	"0x0ed5e5e518343fbbc0fb70386a6df1945679b165d1a65afc6c825315775694fb",
	"0x266d7fa0ddc680255cb75a0cbaf4d586f8edee107e2d3d5ad5ce90b142b9812d",
	"0x038632d442ca8a88fb33a37ffde668cc8c499b6eadc40af5b3f237f2ab62da06",
	"0x18e36f3104718c8e25493c8d0410aec2779af54c23014de0feb6f96f34ee52bc",
	"0x231745182598bb764367a8490e1d61c3d3db1699a7a54212144ffafddd37712e",
	"0x298d6012d765ff5ca0e313106417ffe9a7f08185d7623b37abacf91ab0aebfb9",
	"0x19f2674561197e8fe58d8547d3926ba2702999f9f5147fe77445ad75c336d683",
	"0x2b15c22e56345b557175c1eca4279b909af4e965d941cc1b5352fe2d3229ab0a",
	"0x289ad5b36e4dd22b2c92a95b1e3ef574601e8117ffd22ac0a0389b478f80572f",
	"0x214626bdeae25c53e26eb6f7f65fa6bb4c83469735f03166061c245d00ca86d5",
	"0x24f3525dabc6b7f53d021138eb9dc49133d046b851d4781ebbc94b05dec248d7",
	"0x2791a40b5946f478a90d4e5efb36d8bba14f53e401f87056af2d55a6b7df5cf8",
	"0x09318d2f819b522b0a847e5038118e65718f361f8947580cfa8b8b361ce5e8ee",
	"0x219d8daacb4cef1375b06392f9220f1d204f0e88499c108c961de46fdb5d8fe5",
	"0x268a3e49958e2d7a588b7276b41a2c7f18989d599e80dc85e39c7308d5e92f69",
	"0x10d8226869f3e198f804bc9d51901e5bd24d824e03458fc549a6a16e5c62125d",
	"0x0660a3ff70a9db2da72081518d7a9b473b054a2508b047a5c363f97931dfea83",
	"0x031271a704e1a00bc2b860da159913bcaad3c8acf27c1fa3b28ce2c33841ffab",
	"0x07eddfe02b81044a908a5f7d73f6f461aad59a29f1b55b41a2dfadb7968b4a07",
	"0x1f93630d8bedc406368b348a7006ec4806b4dbadef3e7a022ac8e51f779f2828",
	"0x085a2c147a95c4414b1b67df4fd75753f44d02cc54148ddbd6d771d3084f4cd9",
	"0x02b256807b01a9aa040f02c771397590bad8d20df2520d6ff24401663a3d5f8d",
	"0x1dc83ce1042b12070a2998b52f2d88a4fc2a7d324d6d00664a3bd617bbb4ae85",
	"0x18233d96215c73f726760194809c1a4dd405a32ebe67620ae85f2f2b96862eaa",
	"0x0c7add4ec9ce9e2fea0dbba3833f9e1260249cc855df6a2a1557740b9c477cd9",
	"0x0eb7a2b2438ca0b3459f24bf4bfa719f08272a6f27c747cc151a482ad8fb3be0",
	"0x0167e7ef87785ac6c577e395e475209462b7cfa832cebf6c2add446ecec58878",
	"0x1e52ca68bd85803046e5036b236a6886aaf8fdfeb2bf41ab82c1e0eedf045a81",
	"0x0875d6ad908a0e1b77b24422a99aba983c8eaee3a2ed63fe5d476d58890bf06b",
	"0x1bd4fdd412c233e4ad6558382c336ce03b154baf81846aeb4977c1213c8d5618",
	"0x13ad247e3e1eefb651b3ec25c7dc4aa9df5448c73adb7b683d96a6d66bfa3aa6",
	"0x02bdda4b91162dfd2c210bae56f9252170f555fe3bb6b21a177cfd04eb660803",
	"0x00e8b60d2341985c55753fb8e4ccbfa8b99692463b4e0a11a1b475b688480c4f",
	"0x19cf84ff8a5184368bb6a6f20b3fe1380600b45758ad3cf7b88bfdee2ec61b49",
	"0x25abb85753668de0457dfe38099de2bd1c47978b344ca58f7c2c0c02201997e5",
	"0x24710d523c762410a2d4924464743478feb594b14f12851a541cd3d3ba75b247",
	"0x29ffbe3a4a3087d01b5acb88009abde722a89296b420c9f52449128bc9fdc34c",
	"0x0ec3577cf043961d009493d183d50720d25e49fbc8f9adf62ba72aecd781dc1a",
	"0x26c113117e795ef7b8d7773f81912c80aabc19116464aa8fb12241ceb5dcc2ee",
	"0x07d5f46533b2a2e88682c6fda0bc7cf8a0c70b160a83a967618c65b59c9001a1",
	"0x23d54f9c4c3e67d924d26da36276acbfe02cb4f9ced76f4fe12e0a73ba803343",
	"0x10cd6323749cc45b68d78b9d749dc4a3faf38bc329b4b29e4f80eb3dfe3e039f",
	"0x29f2a6f05e471ef11fc76dfaf7732a9b03a69ebe58720dc54d95e8d66aac9601",
	"0x188cf3ecdef093b77624ab20d47ae582fd0d9dae59987f7fd4c173900d50196e",
	"0x27a03aefe417b4f88a4a811b2ef281b1b8eddd2ac9ea62560a86af54eb5f5f31",
	"0x0990e3d736045f5fc126258b0bae3710d70a9a7fdd4a03834d6ba3e1c41645e1",
	"0x01d0c03377b6c3e03a1cadb8f00aa6b3e856a5b12e9ffacd829a2e17eb1e57ff",
	"0x0b3551f6faa579f6fec4b813a862f196a14f15357371499f98eeb9cfd9970acb",
	"0x0f9444b6c7eafdb309da46679dcbab14a65268c58757e5ca9f76143205985949",
	"0x14db8cfa9979850abc02c0b49b33e22dc4ba8d4557c37a4bbce9a2645a553934",
	"0x08d995c609e1701dbc84e53f3bf3889beca275d0bf20de975ab7bd11f29168c1",
	"0x22ee92f4ec09f2174537985f561d785c4942a28cefe4b1a6f2d736579b4306d5",
	"0x0deac3e417c3e702add7e11e9a9076d2b12d6cd4c432b7bf199c492feaac78cf",
	"0x215d0c99a7fb3227054ecbb04b39dd2e85a33be4b2a78455322b9a8209a7839a",
	"0x144dfd27491018d95745bfa263ab11b6f6865c050700c8bc38ef6196973d4e82",
	"0x2e4c9e84e7b07e659bfc709a3de211b454d028e4a74120b07f130f461210548b",
	"0x260666b80f1d865b7ad6d98dca26cf2c1a5bcbd87d9d9d19673ca4db486652a3",
	"0x21c2ef3ae808bc3b0c1ba5eb6fc594b6e383c1bcb05006b64fad6c3483aaa96f",
	"0x1ea451ecce4adee6b1682153f4038d177e50944ae9cb55b4e0535c24f6911b55",
	"0x0f8df7e95aeaaf0fd8f61c50a0b282c6df25fc884d707ab96936be3d87f75de2",
	"0x2d9abc0ebc4284989c7cc7f6dac8b356c0ed4e6839d8a43d5783dd8dfb57acfb",
	"0x1e851335f8cfaa72342db1d1cd9575d2190c671423fd5cde851051c0dd5746f7",
	"0x189dd6ddeb39ca53540ecd57bfbdebc075f7abf25b1ce4f9ede0f093bee3e6cb",
	"0x1e8e34d095df1eb92444afc3f89b848905b8c2ac63d3805af088e67ff695d5c8",
	"0x187eb13c7f95499b8ebaa0c5100fbacb1184a61e62ca9be6601c3af34c0d0804",
	"0x269000bab546cb9f4e54adc5adb3b08f6f45bb19114977cced6e5035b605e4cc",
	"0x1a4273c2ff4b80a91443e25d1f0ad568ba4586cdb8bd9e412be1fa40ef2f10a2",
	"0x0b2c26399060c182a27682869690bd61c9395795349b873b16ceceed98ccdb0f",
	"0x187370c642e5fd783fba4b7cc7bf03341f6b8efd23fe8c82cf7b627f91bedeb2",
	"0x1162e4855ebcea47475ef6016b2129f42c06dd2262eee9f43e5bdb024cb3e3ab",
	"0x1cb4e22d4b8bfd114320b70e2edc8ec4d077820dc7caaaf3983e0791f77c5afd",
	"0x0b9246a297596e5b285d111c1da4dec37a96a56fe1f3532d45b51cd06da11582",
	"0x06b14752752ad07b43a8daa7963c3c0d9a522671c61f49e7aaa77373839a3ca3",
	"0x21fb7f7798350e11d807e4bac95162f8cbba7f5528f030f22682de559952b444",
	"0x26eaf07e3bb2ad298141174e39808d12d3e8359624d57dcd4bd40d517c889f41",
	"0x1bc988e5a7e158d7367f959b6d877986fb0696e1e9c1f1a59462086d1b4a4a9d",
	"0x1665b29cab9b55ffc5bf2376609265ed9a8f6b8b607636df26ce4f2bc3a6ae56",
	"0x27b2623f1a2a2d769759232e2cff279d0916efd69efb8c8392a402192e4469a5",
	"0x0b11a77df9412a21a871a117cb027da0f8af823131f60add22ed9c4a2928f332",
	"0x1ce9f86e393b0e2d0ddf1270803c496284ded0b35f69972031a5ceaa360c5af4",
	"0x2c97533771428606f2bb3d8cc740ee47c66018fe54fecc8b567483befaa3d898",
	"0x1a257215c9ffb1465ec62562d98025bf33a23fcbb683f89d53a118d060c11cf8",
	"0x25fd8cfe274fe98e6e3ae98aaaab03ade4d1c56cef810df237e42d324184d86d",
	"0x286cea2aeabf040c9bf2160ce8ca90ac489f0098982d790fb42ab33345cfcfe7",
	"0x124c35aa339e0ee2c2046b2f5f0367ce4eedfeb8e3c6c94b7f6460dee9e51099",
	"0x1665dee3f142dbe8f44d85e4d93b39fbf1c86e7a797a8d55932d81a3efa516a4",
	"0x12cc10508db3a8b2f2c53afe91252ddf4bbcf5e4e2738fa8e699edb9fa3df62c",
	"0x1ef850e8b97b2c0560843986acfff158d75aa9210acc6420e9650b56dd9b3c4b",
	"0x2afe8e7f5b4525a1c015f8ace5dd35e62cf89e2232b90eb3eb011d082a114a37",
	"0x2e9c398649994f32c3cd610bc6546bc05aab563fbba41d3124e255ff45e1c940",
	"0x03b21b85b77588506db2fc108bd0cf2c03f6c0653020f46d939d8194ea1e716b",
	"0x2e43508dd63e40681c9122d482d7900ebad01b9392e6c1611019e43cbc5455b8",
	"0x113febd0e87001dda480a8c347f8a368c00740a25ee2d8d36c0608500bec6f2f",
	"0x23cfec0d834aadca55bfe515af80e59183c1b24ea600cf7cde863df02fc859fc",
	"0x1cbb1f36a1e7bc45c29ed8922599374d57010887420c91ecfa8727ad51df9b21",
	"0x1c2a24adf0e0c5254eb4c834e252a04758e84181cfd1a163dc7089337c4eba52",
	"0x2b7a7d74ea33c98f3b45bae98a2498ad5d316a43cfd40e8be9e1c5fd901f5bd8",
	"0x1edb94d38964f284b41136812bd52c7679b1e1a3ce3b3a1354b1417b9012ec4b",
	"0x2c2bad47394f3068c8b996c4859a22dd65460292fef9c250a5a9da8e0628d534",
	"0x0b140b193aab84f6235a88b862ce4275746d5cac940fc494aef23d2279e8a353",
	"0x15673d3dd92656dd60513676f7814459619a09681ee4ab63dd8ff3407a547846",
	"0x2180ff0b613f8cce937068fa4a77a0f97865c4aed76483cb1fe09227517ba888",
	"0x1048b70290f52d668ee6b98950b3e904fee8c844428919300a8945c0fe3e7280",
	"0x0326df120ad22e946c41f475b7702dae0dd42a4387a8702c8e954aca640b2c79",
	"0x2fc77a73fdbaaa22e2fc521f72f9ce5cef4857f58001409898d52c5e5b1723db",
	"0x14938b2b6259f02791cccd157d789c2de68cdad27dc55aa08b9b90ec13dbfd79",
	"0x0493187bf26d38b13ca04c712b42778f8618e6f7d9f9dd52bd4da96e085c7a78",
	"0x1d40769876c58db37289e371fef2ffa559c95630dee045cde6e18370d2ceb561",
	"0x03e7e9b8084366995f7a2f5732349b1139726536b378d78772eabb302705d204",
	"0x06147e6e152d7d58f4cf01e05ae0f024607b8b7c3770bd5b7f3a54e048c30a17",
	"0x2e49300214c5a0a7a6ebbbc8c48cc323be26d42e98e5bbbb0e2a10ecf4fe40fe",
	"0x051c8240e8907e776279e7c66cdbdf39c9516f39ae61883185fac1ef5c64bbba",
	"0x2c737904e8f8e845bf132de2b3be5d638993a9987d0e8003022f08bf6633a5bd",
	"0x07825bfc67f5658bb5a3b1a26c8ad00f657b54a4a1a679ee37df11bed2ba219d",
	"0x1c0e4d8b013541963f8e04667f13e6a60fafebb9bceca823d4054a7a63f8b569",
	"0x1083d7cd5a11e3d3dba85745de17ea4abec8a2790003f39f1b1262521380d4b6",
	"0x0e14bed1525102ddc1d3c17478ab0c2d4cdbfa67f5ef1a568642984665b4dad2",
	"0x1d0c3a89fcc0171b7977f8f20bad9cfae37507a2038d3165d764632461745760",
	"0x03162758d9df43281331905c161ed977e240f4a0c9cdd3f3f18a3b0592c3e67e",
	"0x0fe5ec0343e9832d8a4c7c1bee1d73decc0661a27c527bec309561fda95a529a",
	"0x0ddd28a35af3aced48f61c6558c5c4a72690975c2fc948feec51e1a53a6be5e7",
	"0x2318c886ea334e72e9833e3b0bc9868e51843b8b63e0fa3d814540ad7f5d0359",
	"0x0d5786dec1685237e3d171eb298a36a475c83c0651a450227d261d78260bab70",
	"0x152282540da509e8ab8abcf010d3bd8f29d1c2c60454ffda67ca732db024f3ec",
	"0x1b7f4ec7b4c7a593efd5f53ef204e642bb16fb9298a6ffbd1767183170822ad2",
	"0x19e02df6f343636868908d644e9a2f767bb9fa9c13756669cb1d805898d949c0",
	"0x115d3fa50f1ea1f76b4641586e954be25d7428ad21f5fea76b5889a3f4923ab6",
	"0x059db78b0146183e8e6e0829bad801fcb4a0c4e6a8872cb3a5a497118dd29f2c",
	"0x0e6441f0174ebe123449b9072472442e03c2f72a29c474d514e4dbb72c23bce1",
	"0x2c5302069d7b3b9638a3ea52e5530059155d706af30df469ab929fa1c954efa6",
	"0x181a99f989f2f853ae7db14bef800710a05ddc26ed65cd5e1588150864565829",
	"0x248a2275485f8946848f0d9433ee1cf6501bbfa8f9404341ef81a9c4b128db3a",
	"0x081fa1eb11e0e5198e7135c533688ad0b4e438773b9d99a610ece3dd414845ad",
	"0x17b1a8626b79093deb27cfa548da7bf8499e71928d9433a7243037d493c08b53",
	"0x0af8f7fc8f0ba49245ae28cfbb7b86bdf3f38b4229981b42ad8ba4af993ae5d8",
	"0x19da094b62046661682693af49d35d72a055555b0b2f1a717ef61a0fbdc90169",
	"0x287b1755734c8e691d9651ff53f3bae296bc16d33713bf32457869e3650dbc9e",
	"0x26501a99afdb95154415e3de32b0d4790ff228ee94577537608d30d85a3349d0",
	"0x0c7ca2af6c86f460df0b20ccb30b2a3395f2dabaf7f970f1e1955a1166e0460b",
	"0x141cffa13da7885f34ff295561b5d2da8c5b785932ebcd7039752c0dd1cc08a5",
	"0x188f478690e359cc0b468d095b37f314c53a5873054cbdb5eb4cde9b97d8c837",
	"0x2daafa58220bc8ab507118fb29b65e48e6d8d3ad20d6fa24d57b8a90e21530d6",
	"0x1c8eaf137b6310715fc0881b8a080e9391a0217e8e3980ccc3fbc6423ae10f1b",
	"0x018864151eb108a9115379b3d3ec902aa961f1b846490266855ab48077487948",
	"0x16a426c1847543857385ef8ed03325a50d5cbeff1ac61ef3b710099ab8be88ae",
	"0x13d7ed015205feee9d09307b193811ea89b22bf4ecb3e8dcb951ff1e86ea1dbb",
	"0x2b35afa98d5c3da62ae05e0e3ec587eacbd195fa3405260ce2b910cb198acf5f",
];
pub const MDS_ENTRIES: [[&str; 5]; 5] = [
	[
		"0x098df2176f14c72d9ef76d268c6526704b2dce0d2ab0bf0d0b6506c598a12e39",
		"0x1ee4a90d3dc9864851e9cefb41c3e19218f7ec9b97283b8781d74a9260a9b182",
		"0x090becc1ca85079c519e9519456d0613c4dea08dc17ce3d3ac24c51780bbe653",
		"0x24510d25f0893ebc56cb6d302b08ebc1fe23a842483a1cf105efccdb8f6623be",
		"0x1836f551337cc92e446cda258313ddb9a1291f822c2120d3765f5bb29bccd044",
	],
	[
		"0x235a03a1f31060c62a1bb1b6cbce38317dcf3501e3f0b3d92b4b01a070ee58b2",
		"0x1b0481c924c00fd4b7809ff4a9f9daa7f12efea4821078b869159d34e7c9caca",
		"0x2f1ced49ea067ad0f3a22b51ad12da2d7b0280f3f50977f9b09d4bea92e6b34e",
		"0x28f301e64ff54c671bb7a9fc672a420e3af0382c71af2aec84d1b26d19bd01ee",
		"0x05c29dff5d6ae85f7bc09637a86134c63a8052d1905a8057449cc7d92658f24d",
	],
	[
		"0x2885788b4255180581aac93d5313e7489efc386deceaf13050ebcb8cdddff5fa",
		"0x2a7f03d4b0954a37f9a33ebd9117e2c4cfaa3978e5f221a30db56a7403572a3c",
		"0x291093f3d5182756f267566140d2d8f5356be8408b40ead3748484267f1e90cb",
		"0x29ecfb524f4135deb55d9d9eb02839dedda189c17726aeef96019b205c8aab53",
		"0x1650d221980ec72736322d9fa404a0fe6bea3d8530b71c9522096e455be52379",
	],
	[
		"0x10d08ba1c37b79a36c9d3c9a3d8fd2ff41f2445e7d71dd5ede6f45987e5e1044",
		"0x1eaa7441754632ffae99c9e2f2264c1bc89551e7bbf2c889d92af30bef70e817",
		"0x062101fedd4ecff781f529f57f45e8b479b03d86a11acf549c6555a1293c70b3",
		"0x1c1ec7db63405475e844b5cb6215d9e2919e903a7387721db150c9977a1818b5",
		"0x0b5ca51ebe8fd98da6e8f4a4465e19dd210bf59e0841f50fcf0f06e43d83ce1f",
	],
	[
		"0x0143b223ed92a0b426f8f2886cda3d8fdb565eb6acb4841897489e14cbc943a8",
		"0x017f82dcfdf078265df4cfa1d9d79aeec0fee433eebc489a875785b99dc8832a",
		"0x0013ae98ed23af18461bcde9ff99728edeec173e63c5467a209c2a34b503dc72",
		"0x0e120df26061ea797bba1f6153995de0090ddb744ad23bfcdf1ecc28a9b18338",
		"0x0233c1411c8cb5ff0d33e20a65bfd9c0347deb9a12a50e55fb01a40248ccc366",
	],
];
