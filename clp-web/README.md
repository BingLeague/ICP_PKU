[![Cybernetically enhanced web apps: Svelte](https://testnet.clp.finance/images/logo.png)](https://testnet.clp.finance/)

[![license](https://img.shields.io/npm/l/svelte.svg)](LICENSE.md)

## What is CLP?

Crato Liquidity Protocol (CLP) is a decentralized financial protocol built on the Internet Computer Protocol (ICP) ecosystem. By adopting a zero-interest lending mechanism, CLP aims to provide users with low-cost, efficient, and secure digital asset lending services. Users can borrow the stablecoin CUSD by pledging ckETH or ckBTC, thereby enhancing the liquidity of digital assets. Through the dual-token mechanism (CUSD and CLPT), users can also earn additional rewards from mining and liquidation. CLP is committed to providing innovative financial tools for ICP ecosystem users, facilitating the appreciation and circulation of digital assets.

We encourage and support other platforms to call CLP to deploy their own front-ends

## Prerequisites

- Node : V16+
- dfx: 20+

## Installation
To install, navigate to the `frontend` directory and run:
```shell
npm install
```
To preview the website locally before deployment, run:
```shell
npm run dev
```
For preparing the build before deployment, run:
```shell
npm run build
```
Deploy your frontend website on the ICP mainnet. Before doing so, ensure you have enough cycles and an ICP wallet. For more information, refer to [ICP Wallet](https://internetcomputer.org/docs/current/developer-docs/defi/wallets/overview)
```shell
dfx deploy frontend --ic
```
## Configuration Modifications

Before deployment, you might need to modify certain assets and configurations:
- The logo file is located at `frontend_assets/images/logo.png`, and you can directly replace it.
- To modify the domain, go to `frontend_assets/.well-known/ic-domains` and replace the domain. For detailed steps, refer to [DNS setup](https://internetcomputer.org/docs/current/developer-docs/web-apps/custom-domains/dns-setup).

## License

[MIT](LICENSE.md)