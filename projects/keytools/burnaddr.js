// 2020年中元在國曆9月2日
const { encodeAddress } = require('@polkadot/keyring');

const zero = '0x' + '00'.repeat(28) + '20200715';
// https://github.com/paritytech/substrate/wiki/External-Address-Format-(SS58)
// 00000000b (0) Polkadot Live (SS58 checksum preimage)
// 00000001b (1) Polkadot Live (reserved for secondary use)
// 00000010b (2) Polkadot Canary (SS58 checksum preimage)
// 00000011b (3) Polkadot Canary (reserved for secondary use)
// 00101010b (42) Generic Substrate wildcard (SS58 checksum preimage)
// https://github.com/polkadot-js/common/blob/91340577df23cc8698106444f98063860369b5bb/packages/util-crypto/src/address/ss58.ts
const SS58FormatPolkadot = 0;
const SS58FormatKusama = 2;
console.log(encodeAddress(zero, SS58FormatPolkadot).toString());
// 11111111111111111111111111111Gzr8zVsW
console.log(encodeAddress(zero, SS58FormatKusama).toString());
// CaKWz5omakTK7ovp4m3koXrHyHb7NG3Nt7GENHcCi37ZFKP