# Event2020

# 2020-0902-burn-dot

中元流動性普渡

去年本來要燒 Grin 結果太隱密很難燒換燒比特幣，今年波卡上線加上以太坊流動性挖礦風潮，趕流行徵求流動性提供者 Liquidity Provider (LP)，不需合約，直接先燒 0.5 DOT 到 11111111111111111111111111111Gzr8zVsW 地址後保證 200% 返還 1 DOT，名額有限只有五名，欲燒從速，但須等台北時間中元子時過後才開始算，注意單一地址只能燒一次。話說不管是金爐合約還是金爐跑環RUNTIME現在都還沒辦法上波卡主鏈，燒完 DOT 沒辦法自動燒出 LP token 來換個素果之類的。

```js
// 2020年中元在國曆9月2日
const { encodeAddress } = require('@polkadot/keyring');

const zero = '0x' + '00'.repeat(28) + '20200715';
// https://github.com/paritytech/substrate/wiki/External-Address-Format-(SS58)
// 00000000b (0) Polkadot Live (SS58 checksum preimage)
// 00000001b (1) Polkadot Live (reserved for secondary use)
// 00000010b (2) Polkadot Canary (SS58 checksum preimage)
// 00000011b (3) Polkadot Canary (reserved for secondary use)
// 00101010b (42) Generic Substrate wildcard (SS58 checksum preimage)
const SS58FormatPolkadot = 0;
const SS58FormatKusama = 2;
console.log(encodeAddress(zero, SS58FormatPolkadot).toString());
// 11111111111111111111111111111Gzr8zVsW
console.log(encodeAddress(zero, SS58FormatKusama).toString());
// CaKWz5omakTK7ovp4m3koXrHyHb7NG3Nt7GENHcCi37ZFKP
```


參考資料

- [blockchain - How do I generate a burn address for Substrate chains like Polkadot or Kusama? - Stack Overflow](https://stackoverflow.com/questions/60044378/how-do-i-generate-a-burn-address-for-substrate-chains-like-polkadot-or-kusama)
- [11111111111111111111111111111Gzr8zVsW | Polkadot](https://polkadot.subscan.io/account/11111111111111111111111111111Gzr8zVsW)
- [The SushiSwap Project 🍣🍣🍣. An evolution of Uniswap with SUSHI… | by SushiSwap | SushiSwap | Aug, 2020 | Medium](https://medium.com/sushiswap/the-sushiswap-project-c4049ea9941e)
- [1Ghost2o18o715ZZZZZZZZZZZZZZfTUTEh | BTC](https://www.blockchain.com/btc/address/1Ghost2o18o715ZZZZZZZZZZZZZZfTUTEh)
- [1Friends2o19o715VVVVVVVVVVVVUA9mTC | BTC](https://www.blockchain.com/btc/address/1Friends2o19o715VVVVVVVVVVVVUA9mTC)
- [book2018-quantumxie/GIJ2018.md · dltdojo/book2018-quantumxie](https://github.com/dltdojo/book2018-quantumxie/blob/master/projects/gij2018/GIJ2018.md)
