use super::parse_matrix;
use crate::{
	poseidon::{sbox::PoseidonSbox, PoseidonParameters},
	utils::parse_vec,
	Vec,
};
use ark_ff::PrimeField;
// https://github.com/webb-tools/bulletproof-gadgets/tree/main/src/crypto_constants/data/poseidon

// Parameter for:
// exponentiation = 17
// width = 5
// full rounds = 8
// partial rounds = 35
// prime field =
// 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001

// Sage script command:
// sage generate_parameters_grain.sage 1 0 255 5 8 35
// 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
pub const FULL_ROUNDS: u8 = 8;
pub const PARTIAL_ROUNDS: u8 = 60;
pub const WIDTH: u8 = 5;
pub const SBOX: PoseidonSbox = PoseidonSbox(17);

pub const ROUND_CONSTS: [&str; 215] = [
	"0x6da1b0517f41142fbe00add56ac4f2ba585377e76655b9d103d8bac398a742a3",
	"0x53d27acaba9bf475079a567fee4d23faf1e9eff9cf639744bb33aab33e66bd6a",
	"0x62b383bce8186ccc57263f9d68515f6cbe62fa3eb4d5ce77084f10d525797c4c",
	"0x50e75095a604928e61dec22f5bb8f6d57d5483abd3950d86c3c8e7e01304e0b8",
	"0x560a8bfdd03bf8d96c79db430787d9dcb97f800db5bc880e8a5623780e2d36b2",
	"0x0d46c92fe58b5c18e973612439a7c3b54754a1f730fdd74f5db3eedf267d5c18",
	"0x451905eeefb70497ed9062dd3b0e60d6026a60f852b626a1724d78d2ecd3b5d0",
	"0x289d002e6e0d84bb28f47c6b39da3457c0c0148a95d3449e52c8d89e35f71475",
	"0x085c5f9f3ce18fdb6141e0e0128936de2f73591c8543e37ca5e629885066efcd",
	"0x42d385198a45cf4a77e4ce321db5a8a6405cf0b8d2b4aa5b409a5080378f860c",
	"0x0362003017c44db5380a33fcfa1bb23af25c2649920a8a41f917cf30e3e5529e",
	"0x734a5321f4612fe492f6239eeac3b175a557d042d197dc8b3d62c7dcb4295a5e",
	"0x308dbb9dcb2e60c52ddf9dae5d8650f5707a0cf3f72d56c9813c0086464939e1",
	"0x27a4144937fb051ed150c6db636de80fb97e269b90dd8b15095da05bc6c2762c",
	"0x178b132dd5f3f098a1e24fb5d9b87612ea850d7e3df01cdf09745cf4ed09473a",
	"0x23a5e8b12c355ae87989a8939201a98ebdfa4599a8172e512a4eecab58636b60",
	"0x6a4cdf5fa94932d77c8dcd48b0d32ab2af38680a5b57b831cc403ec00dcf08c8",
	"0x6c3be3e10f28d8b7e1e24ff0640a9dfb38523e7b1f90b35c8febba5773781d9e",
	"0x37a35fcd843356e5dd548d0902ba51481ca9da9ca7a94e4cf629f0bc7aa07c24",
	"0x3498cf5a3c8fed21cdea26703e402310db76c3790562cce9723cdab66bb7295c",
	"0x6d85621fce97ba319c708fb484cba050e5b3853b7cad9e26eb3e71cecf28ca6f",
	"0x509796f9875d3a625bd3b1a29c86bdf15f431cae3d814a0bd91fb13150f7910c",
	"0x29021a4935f22cee802a4cba1faf079c5015de698bd6222dc61ae83571cececc",
	"0x63cb708c420401060893b578c3b7f386a932cf29ec07886e8c5f64d5d7491f83",
	"0x17f359879976d288ff45e1b4bcee5d592fc83f1d0086cde1eea84fe683509fd9",
	"0x25767857430720da67a6c2afe05396f6eec523cbb8a585b61dc5e7f63890c95a",
	"0x73be8e01d9b61e70aa29faa1c30de2f1d70106f5f10d382ec517ab9cdc7e35b9",
	"0x33063e9a2aa9df0793da56f47c0056ef7d13fbce8ccf8e40085184d200790a49",
	"0x3a96f4f93a1891c67dac2e36391047820697343638d5f0bb2eaa579ddfda8228",
	"0x263a2cc52bb648c8d390aa857cfbc19f021ce1c73835a826dcebaddb1923560f",
	"0x07680f7d80b4774019b3c4a06f7d159701c28168032763bca72d475d20823435",
	"0x3f49a81efd5cc885875e282481e95fe5231e3dffe1c68eed3f142aff5eba0075",
	"0x6cc48a80a66a7119c2afa6a8f2ecf9db7f4736cf764ffb4cfc93ce9c4b8451f4",
	"0x65f7f740d9a1073e34f1e512f39072ee3dbd5d66816bcc5f39847e8147023fe3",
	"0x320ce67e683b7cb53950c5baedd6de71c584c2da03ebb0ce98df089f58c340b2",
	"0x544572876f0cc54972e58cf6f8f421b5142f587d62c15942fe55eab0cc1812c1",
	"0x184e7166ce940d240a8a87525e52bc3262f2ff1ac6546f4df10c68f84736aee4",
	"0x0209c6f93ed576c4339430099e0b8a079adcf9612a21f2ccf0edd7c7bb2cd1cc",
	"0x15cd85523d558c909fbad0724abdab75e55c9d33581f26e00c66b2adc8e4888c",
	"0x007b7eb864bca0ecfdbd7d47c3f5a75c47867df130a43bfb74051834b73a9cce",
	"0x5da3fc9b75308a0e519f0ec44209a86dc509bda4dcfbdf5c700283d32f26c9c6",
	"0x69ebd1fbb92844e8f4912cdda2d854cc6083fb9082514d5da9a38b173934b838",
	"0x66c212407e02ff376cf6767b75cd8fa243859889c4c1e8078ccec88c937cb06b",
	"0x16f57f5ac66ce1067bcf16908777557ced94689e601b15af6085f8790783de47",
	"0x1f4c3337521607e12a7d4a399b1d714d4064605a97c80092dc830e5df7cab053",
	"0x595efee6be303adc4abba522919ea487e65219a8838c504210ee3e2c2ebfac9f",
	"0x1aa6fc4d3bc82e0912412e0f269b4c0b6f7b5ee4c5663b8fa6037819b8ec5841",
	"0x4f83a9e1af8ec67f2b3555358a3b3d8b10132eda506e44faaf1e021d4bb118f0",
	"0x62440656a78e0e3089a3a16e116b1a9a8945bdbfb938210adb0e753d396fadc2",
	"0x2f6a3883e9485c4bdd6b6dd3878731207f20b29d4c2e7abdff60f5bef44c5022",
	"0x67620085b92054b85b770ed0c6e10d20a9ca4d145c81f132ac95de0e1a184e97",
	"0x4e4715be48d1bf273d655c9f6b69f680559d7041caa5e4b043340bec94327559",
	"0x3203bbfe830e839284e325b29a91ec295abcec7fdc32e710182c29a38af5e587",
	"0x6fa0826c6cecbd772776de1a768ac631bfd2ac2cc4af2a25615c182229455fd4",
	"0x406c8f2da61f009030097cb0e2b0f4bcb52070b8663ab180b1353b98bb26c222",
	"0x22744a575d939fc8d5a2605e7839a6a10f50160e8a9ab25d4264f5bcda3cad84",
	"0x38b46fdcf558f153a7761197f339f3f5f762e97e1c9a9ecabf1f852536882f2e",
	"0x0f11b17e34ebec631d4da01415a029afbae9dfc5ddecffdc9717e7e145414cc0",
	"0x6d25c2494a15dbb51a87b9df7117547fdcf603a6965b1a32b56b8b4f51cb0090",
	"0x076727726a0eaafc757b34fbc00a4693d3ae1eba037f3049dfded2abaaa2b9cd",
	"0x22e0b8b16d0216329f676046cf511a513b76bc775a05e353aeb7a446861c7302",
	"0x5cf7fec64a19fe090ad7378d4711a28687f4f849241a0425b59f89af7c2e0871",
	"0x71877755d1ac7e2d844c43404e10419c2f30793c5668c88ad130451f38d202bf",
	"0x022c674d638c317ca926b74273b768ef1fd18713ea1a6434afc35be587012fac",
	"0x3c9df4fcd4676a15115774816547e03473fe258b060f2b10b5d2b61aaaa7313f",
	"0x188dd84483971ec627076508d272b839aee9fe9d1cf4428fb9ac239cdc2e4e4d",
	"0x5cd1bce49ef78b984dfa79c93bfc557e2b06f9c2df5e6367ec5b707884ee0d18",
	"0x132fe67efe93cbe1c60914b9f03b3ef5eab4508c23ba9c64d0ed44b530418d77",
	"0x3acc44f34242e04d4f45f20836140d4cb36654b6becb44335e5b929c2136fe24",
	"0x5bf4a4c086d049b1afc62192e9f6f9fe4e3fb18679c241b349717868131fccdf",
	"0x70c501090ac26ffdcc642921416bafbb6bfc628d712938242f206f606d0a6e29",
	"0x3445e3e93defb7f879b96519160d03efa4411ba6a2f965a940e53dee14f4f720",
	"0x553dd9f13b03182c4e4f02c7bb055c5be4d1cab6d723b8448fa31d5b3467a506",
	"0x234cf1e2d62ba5a6d1b432c57a806d6d07b8833d928334ee2ee0c8fcc20fff08",
	"0x6c02225857a28eb97bf75f2cef19b8a8ee07507464fcd13a42a1143e121b79b5",
	"0x43f97e53849ea76a6e9f9262c962c9ca8e76d6c527b0c22303d40435a11cb3b8",
	"0x03efefbc9817047b6d4eb5926365f08d89f4e5e3d1d67ff4847dd08979ff7ffc",
	"0x0ac79ef03d74df2f1a515d1addb3f263936442fe6450520731aa9b6d8368256f",
	"0x218e4dca33dc56f4629ba0bfe90a6b8a44dfb2bf30f88430b978d790a2db1b06",
	"0x570f03da10cb711bb5e85217545ce7a9fc86ea7a94bf6c6c2fad70dacbdf80a7",
	"0x17bfbf96a0a75d55218284a609abff15d5d6ed450ad7bc6810b9b470ae741f29",
	"0x36af576522e5929c0e342772ff4ff3fdea53e12a577a29fd516a6b03b80f5831",
	"0x62f9f82c35cd92bd7bb90439584a82be6d5b28ea915c9db683ac2c86fe16c396",
	"0x3cbedef21a364afa9b779e95400df2fa9e44a9f0c81a9dfddcbc3d29defd881c",
	"0x37d087fcb24f61b7171de31d83d4ebeb64dfe8a41c273f7905d9cbc2128c1a48",
	"0x5321c0112dc4e9b6ccc36b165a71a269561082d150fce15f59b719ac44aa1cc8",
	"0x59c83150d50b58c1d4f8184ebdcc56d497738090c41f6daa69a8f4a3da3d5859",
	"0x383a3a1336f09c268319aa71658d69b93b4093781395c1ee47311fee6d602e04",
	"0x2dfeea101cc1d89a8bc443b6726d54fbe0106b94620f85bb38f0dc91aadefd97",
	"0x5496ce14f4832e2e3b1ec798c1d5ae4cb371af1e6776d767ccc9929f28971390",
	"0x367266636f88e2f0cef5b3341bd6496f95526b86f73b99986e639c3d6b26a300",
	"0x2c9863468802e7e80e67656d2a279e6f618c161e3bb54302df4afa2ef8cb145e",
	"0x5b96df89d4dd610803e973f33ac42a3df109a7d56431c8cec33aca6d0d32c82c",
	"0x05061530265ffa3db9368364c59355a5150cd05003fb2fd6656757965691bf94",
	"0x48a29315c2b98dffbb36774060632f1321931c7ee9384720b4893d1f33d1a613",
	"0x11b30b74db486440a842c6131f6f75f332d0c52a718047fd90bfffe23dae9377",
	"0x1f75a5b4aec20def7264c6f849980e6f1336cd6978d143da7250d821d4735eab",
	"0x6d2a5049690fc40301e235ad126a1a7a2f750bdf76ccecb152d1632b42f07ed9",
	"0x7298c46dc3894238cdcaaa8a0f491252238146d8781a1f77f01d75eb028aba6e",
	"0x110959293d1374111853c7892e4fc8640c8fa4c2c1e9ea38e097365e5f97da1f",
	"0x0cedfa1703e6b41a72501949c2534d3fee9bc1f594e04495d10c4c7675c71a25",
	"0x51c44d108306f5b5856367322dc2f9b015291c9ebb5c1a67ef1cb8e5913336bf",
	"0x61fcfd9af35ffb812f6ac8f09957307ace3eb987703fb818754209f88fc7f3c0",
	"0x66753380155ba6575eceb8dd01a3a6cc23a4d1b44995695d271fe1c196500106",
	"0x23153e613f4caf98019551c18cb70cf26941bdbf338f1cdc566fc21e2aca0e99",
	"0x6a368daa93b41732c8b49d580c6e3ffa88bc4b84c806393a77f3a949f2aae86b",
	"0x3400a7deab52d02406a3922d00110ac829b3b7accb9f103a1f2823dc13f3c294",
	"0x10fc2f5888afa4bb8a417a0f6fcd547b8b0bee5c449e0271de8dbf482356b87e",
	"0x57e8d9c523ecb908e399668acff7c639f0ec0e99ddc13561cc916870c0d6273b",
	"0x4f1a7bbc14988b3a656d2a46edab89d05d2a72d0ec6f4f22e35c62411e22d0e7",
	"0x68e1ee1eb14a3b8989ccdffb60c8e423975cf9fb969b88d4f8f6c07a7dda1e3a",
	"0x31e350f0bb7d0f6b8eb57ea0978c7d663ddeaac5b007943772029b535bb4524f",
	"0x4193ab67cd5b40a4429738a54ab7567342d204581db84d34343c50ecb8d60e87",
	"0x2e4fe47b2cfb538848d43f150fc0bbb5b6b940ab53128b2f083c097cca40a605",
	"0x2454f5466a0bc59c48797808c17e4323b2b959684b5737a38689eb32f4a45bbb",
	"0x5c2fd11113ee32b6cbbc06b9f83d69a49a74c8f56d3ce9a78fc53108dedfbbf6",
	"0x513a036ea00876e0066f08cd2fa2055ce008249b2bc7474f6f13d3a8e1e3d7c2",
	"0x24b8b433adf61196fd5afe2de2ccd9649e1cb9e679e764bc8b702a3c760170f0",
	"0x373b37dc625e11ddf8a3b8bad085b9d5fecd3c4554a1dbe6d626253b9b59f541",
	"0x599532f8ab332596d5e806ccdc0826c38f5bd3497ad030a879c6c4a5f144c903",
	"0x49720bb36f9d0bdd4a0e738fe9394c7defe1b368655b3d30e9b5a84e2d0090ec",
	"0x3dd7b066d8ec0f3c5a4a0720781ad569ecc29418636449580eae3c4b5c49a5ab",
	"0x3b414c40033f5f8a986be4aac1914c670ade381c29ade6d4d883f78977dd9df9",
	"0x68cb2b7a473c7478f2c9aec4e7bab2ce101ac7eb13e78a85575802c9ace8eccc",
	"0x37d5f02cf77585c0e442f6a5f7cbbe4381dc290f932a45db9bd30285c87d1f08",
	"0x4f8e6e881339338c885d000101d35f1b5c3ad87f3d52481d0824ddf62be935ad",
	"0x49f41179dc8c5a6ceef4c46795a9d33962583d73fe507470d18fe6d3b6861c19",
	"0x5e194a850dcf9cef7747a6179d7663a5b4071612c0657dd4220e1abde4b64945",
	"0x69fc8ece96dca90caaae41b648a3f0604a10904c36f79e5fedcdbb7de5a958fb",
	"0x34dddba003e3270c562c98e2ba7b5384fd7c531a11e8067799a8b4c80b6f7ec7",
	"0x1cd34662a68a2da03ab38a3a29d429f21e128f67b0c1e51e5d6971e53032d197",
	"0x146824a545b797760f91eb5d04db98f702a0b6b12637ca16ab3080c809fda945",
	"0x39734882e901470183d8127564fc8ee5a90e2f8cb98e3b70444a1d2344f95e08",
	"0x4970f1bfc7ed6aebdb671d5e9efabe6af3106bb0bf78bf8285e50ec374d359ef",
	"0x2de1f698d962f479100efc552d6db60499e9b959db2e5eb41049044658a31be1",
	"0x03670f1674effeab6443569f3e0036dcfd7442e076a5b0b975e0e996613db9c3",
	"0x063e74c5dfbfbb61f575dba1cdf5269d5c78811de313d610bc9f2347175c7857",
	"0x1d991f1b19c2081109e6585173de203bb4a23dc239cfc159feeae334a5e2f356",
	"0x4d58c3fb46ad41237da752a15bcab5330a59b4e7ab4c9ab50653d995aae1334b",
	"0x158a9df1f8b2b642554391ba6d95ca577e6e39b6ae336b49cef21193d914136a",
	"0x20a54de3cd0b18a9a44808ff1005596192dcffbf51aefc21c12ee2649572394d",
	"0x5fe43be591cc8420eb863d5973206c949d2a692a9805bf5f1c2c8ac6d6f86594",
	"0x4f48d753abe22c713ec2f613516fee92f39f471e5dcf495ce60f65c2415264ab",
	"0x26eea1fbb7efbc9216253a40345ef588d3dd99fc490dae0666251ad5a43f8629",
	"0x2c0e91c704bbc382ed1be12c1d1c57b142b89e1f957165870b0e9474519a9c0e",
	"0x34849c4040c78e5ac4d4346b20e5a962fd7ea93a336a3142e6917977ed984807",
	"0x53383db9ffdc0349dbb87b39c2a70fc568dfb999a8a6ee4346e38fe80ebc115c",
	"0x1d05346f8655e9c2635fddd1991883791217289ef4984a05e8095ba718722713",
	"0x61ff4b4f34bbddc187552817a9f971fac806a987dca878bc9c1618dc68221432",
	"0x3e4806015c3e7f379fd95370c9b82eceba9ae13b10bcbcd647f7f6c48d7061ba",
	"0x52726dc7a4a8831653620567a7c48d182db745852f1f76f1c2f0bbd28b932f5e",
	"0x4995039138e24a4feb838da4b7e8f35093417cd70eb0931b21eda1757b83fb49",
	"0x488a83a60c5e54be05ac2d64f73873c2c3d241c835a4c23302cd3cfa298793ff",
	"0x3e94c4bdbf2a0ce7be20f5b4eb18ac07b07cf1124964f1d8e3dc83c12cbbf504",
	"0x436eeb6b8dab93db1792a4bc75bba4c68d39ef12d25e02e1d18edd449b77736b",
	"0x48ce67d02f410b706f2db2f6c7447a7ecdc30612da2382198f5e28219d506d69",
	"0x623fc9e253e0acf10a087e4ad283940002317178af1626a76722ba42aa332002",
	"0x6180d9e4405ee8934a8e66af2f8853576e4466b109717cd70da6a1cb787908a6",
	"0x1f73bd1a2a6fc377f4deb88dddedfa344bfc9c7b1a763247619f626d8a3c2140",
	"0x5d1be5c3c1e8408daaf22c4305a1aa032ea815d461c75813553deaf09c6a89eb",
	"0x17406b760be55332636fc294d90cee6cff73c44c14716982f59f60119e56e304",
	"0x362285910ab09e3de61412bb9018523eaca80ebf9ae00309ee717917b9d9a296",
	"0x34d95a8e759fcf6e90113408d0ff45bfbc48dba798dc06526a3ead2e3d5d4585",
	"0x21f4531aa17c2304527a75f115f47c93462903c0032208f07e32a3adb1b3512b",
	"0x2e1e132d7cb5e54b4f341d44b8f9721d2bcff63be21054ba297257350347356b",
	"0x0aae69cab48b8417ff68e381d2c73e8a387e8deeebca146fa80c73782d93bd6a",
	"0x1fcf6b172c1e28788f71a09b59f20a706cdd86d82c6effdf1913bafd87272dec",
	"0x40035fc8ef8c0ee95fbc2f26436093f7d6bdfc102072f6f1162644131b5a4e6a",
	"0x42baf54839d1aa91c16845078c3b3033dc864c776afc1cf058614aa3920973ab",
	"0x217b5d4fd48c9a3533a225bca1d9255850428614ac0db603058c8ec5d86793cf",
	"0x6bd2a5c858283137cf18514f05187086d2e882ee160dff04f1246a5041d2c33d",
	"0x5426db6185507411b8466a29e0a8231f61cd171f75947ba6cb98a5bc7343a008",
	"0x187f615bf0391b7e7e877cee52e58ef81f74383b43c966aedfd24e6fcf6ea135",
	"0x2d3c1c87dec02fce9bafcd6e1aa55a5333bba3c6a0dc2a18ec55277f1dad0f12",
	"0x37e6c7e86fc2701939077427225d1d2435acb542407ade958cedb3b034134343",
	"0x4be68040a4b1ba11ed066840f8df32fcd4b74625775010ef94f10010d80ca641",
	"0x0059308a7ff64113fabf576c0e3cd1fd42ad48cefc971fac2d05b7c8065f8324",
	"0x0ee3d0ad88f7c7fae9fd007c773852cde33298cfdc95c5b1765bc373ebfaf0d5",
	"0x6f2b965ca7cfdc8f2868800f19deaf4986caeb3345a039eb41da6bb9d232cd3f",
	"0x55267e9e19a14ccf78c1ad6f7beb8c4c7b04f7bf962b0fedacd28ca26a36d3ce",
	"0x017d24d06890f9564d584345772dc844f8dae4c3febde743f8f8249a4223a59d",
	"0x2240d7e092b74a62c5cf4c8ff754166e6ccc36f3b390282c76226d1713f1e58b",
	"0x01ecfd1e46d7fcec298d5496bd8360368acbf2d042a1ecb2dc25ba981eae0133",
	"0x1fd07bef7f2f67f127d7949f0b4fa9e3e8ac22bafb2c4737f50b7c0f80600315",
	"0x65fead422db6ab4d70b1424ac22fcf4cd0e6c7fb359117ca901dd336edf1c355",
	"0x6bcd8a02ec7d86851a11a96d7ed9d185ef09d6f569c4eab7351f6b67421dff61",
	"0x295f2101f4068661c5f696ba105a7a1cd5633af5826d214eda8f872c1b0b96a8",
	"0x17361c56c0896e369b040a8927594b846dcc877b063ad7fba0b0f849d366792a",
	"0x6beb255f4a660e16b6da8fd0c5ffd8c22ebc420d4b9aa215aef03bac8e7e5b8d",
	"0x0e9ea00b0324d4291f9ecbcc21f61d62171b0c5c675b0caf12012e88978b665c",
	"0x3c662d3a4690e6fdd6a2adb77f6aa1fd306573f610e649595f0d73d009bc3ae9",
	"0x2ff1e83d40a209635c5a61ea802f32fb1d18980545e0ed5900f1be58642d5573",
	"0x05a643de16c7f88c46333d298512bc769860daad0076c37cfcf0a686f8252685",
	"0x01853ef6f145e0f737e62a74e601abb493c39bd026fddc5e165cc725a8501862",
	"0x23b2f756fc9d6b685198c6798fa0543bc705c1bcc06680382f0aa75b43be5928",
	"0x4bcb92013628cd18ae975f6c04e0795b225852fdc816bbf5b4035244e17adeb0",
	"0x386818e0b70b1e8df8821ebbd32ec73a34481fc4070e36287519199f8b052693",
	"0x619a8515506e9f1caa44c3ad10129ebaccb744007aca889491f942065052b2bb",
	"0x3a925fb3bd1346d55ba04e3a33c56b690591e2b1f7ac95572313d897bcc70021",
	"0x5c537c86293124af20e40e2e775654fbb42953bee6bc5e47f618348ce981522c",
	"0x5006acc99dbde13e2cb525904a37cc208c7ed739119f48cc9a8910c4fe0d180a",
	"0x34c78eae36fbe7002fc4f9109c81e0fc6b56de183f3ebc61e96871b00d815129",
	"0x59596d310faa34a1fa973e13f98f0d7911eaaf677785d47e5aadf4eaccb1e450",
	"0x6d0ff927de2d95adce2ec50925ebeb8841926e02af594135c4922f750dbe3d16",
	"0x1328caacd06ada22aefa16f38fea4739237cd3a2c8238ea75f8e6c1a54f64535",
	"0x38f3473ddca27fc4d42ef2308b28f6a27871523b1276502b10e93bec8aa59441",
	"0x6b0e98c4af97d9eb1c1f3d9ab7e1d6c11b4076ac2b565db50b3410fc6addac5d",
	"0x0a8b29fd1e8e7f9781b68d2d5749120ac025be11cf89eda13daefe79994b4847",
	"0x6bdb8a5a09058904a6e993cfdab4c9bb308d7dd28b570ae695b070ef0165f841",
	"0x544b1fb29c2a95a47de51d9b3fe0e13b8d6644c632f65b237508397dbf86c42e",
	"0x5cc5942a10bd5520d1109cecad0aa3a1fe50289fb7215eb338602069a6293655",
	"0x38ad8f1ea68b9d0381581aa87ce5b7bc7dc56102f210916b317fb3a042929514",
	"0x17fbe9b99bc06833d647bdc69873c1c25f241333e6461382ac45ce0f7d9b0594",
	"0x35df8a17c45193c492458a795b9e910a1126b7310573d6bb1402bfbbfed19981",
	"0x4c473f4724d411f05672001f3537ae9d2bb6821f6ebd5c7870c40cb26bb18482",
];
pub const MDS_ENTRIES: [[&str; 5]; 5] = [
	[
		"0x50eac0cb3682fd261af47d46f3022c9be7b7f1b478f87933ab5dc41d81c8c53e",
		"0x413d8c95716c4cb54b470c2d64cde4a038965f973ab73132c887496a64c1b2d5",
		"0x61dd2727a07e08f7ad1588adeb210c5dc47b150ef50ecc6081823de093475c63",
		"0x367c2f0a60c089e9992d13f2b15aaf49aed8dfd855ce9f52d39edaa5f8ffb006",
		"0x49666eaad353141baa332c23d770651771a3152f6e7e1f8bfbc26c7e627d895b",
	],
	[
		"0x66b8d116744a088a767b9ce09616ae57a32fb01526c1bcfdbb918ba18a20b3ec",
		"0x3b10e07cf3876e02982e8780b902bc49341a21e737153e43697a0af2e7d8e1f3",
		"0x390efc2fb5ca9bf7010424465948591d2901f0bdb9af87534066cd3d85eac34f",
		"0x21fceaf87d8bf2591bdfe6f53d426998fa31b10466214b5118436350ceeca4b5",
		"0x5e23f1bc20026e131c81d1b4cb7bef01f7f9dc54f00408740b15940d229e73a4",
	],
	[
		"0x12c0f46c2fb7799e8d8e4328bde3ad8e809cf40971dd1902dd849013ed2f6aa7",
		"0x3725d34747e2694082a355fa79107daea7b117a5c507475db27f4763a53978e8",
		"0x0f5b244c063e11dec15c97b3389ee0cb5d117b809adb60653a17a6a5bac80c9a",
		"0x4665ab93e3a7f0821aaf47db4130572eb6462f1075fb758bee222bb8ab46df4a",
		"0x3644aa925664befb99ffb0c5d6b71ebdb03b8d56a74d0e7640f3a53daee35d95",
	],
	[
		"0x4199cc6ce105b4bf24a9f5befb31580e721a1267d5a2698acc97dc5e95566121",
		"0x29e1e38a011746aebc3e50fe312e4b56ee025fa29bd5e7c5b84d8b11cbb4732a",
		"0x216a7ac11fc900444d046adcb0121f3ef8bd4b4ed0df1c01c7f58a7f40f6912f",
		"0x0bf1765bfeebf80cdbb177688b842cc08d8ac6623c6207aef5082d6c7e4e9beb",
		"0x3418f06b56ddbc844a2fc7f1ffd44cd460bcd07189e82ddd290a10fb36cc78b5",
	],
	[
		"0x01e6e6952432284d3f85baef96e954cb227c9a47f714bc42fb508a1e27fe0076",
		"0x494b09ecd7b7c623f7713e6c437a11a213024699f7e27fad862c0df133f65c2f",
		"0x06003b7cae04821443b1311cc425efccbc0cc44cfc4cbb08090e109ecea3aaf8",
		"0x0cd84b2b9f355689c5deb611c0e6bf0ac65b3fdab59efa858e2a35b11ff2e2d7",
		"0x0d3cb6f90c619dce9396f191bf6822c9bcd51c4bd3f8b405a7ff547fbc83b648",
	],
];

pub fn get_rounds_poseidon_bls381_x17_5<F: PrimeField>() -> Vec<F> {
	parse_vec(ROUND_CONSTS.to_vec())
}
pub fn get_mds_poseidon_bls381_x17_5<F: PrimeField>() -> Vec<Vec<F>> {
	parse_matrix(MDS_ENTRIES.iter().map(|x| x.to_vec()).collect::<Vec<_>>())
}

pub fn get_poseidon_bls381_x17_5<F: PrimeField>() -> PoseidonParameters<F> {
	let rounds = get_rounds_poseidon_bls381_x17_5();
	let mds = get_mds_poseidon_bls381_x17_5();
	PoseidonParameters::<F>::new(rounds, mds, FULL_ROUNDS, PARTIAL_ROUNDS, WIDTH, SBOX)
}
