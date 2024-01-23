# chainsight-rating-stablecoin

In this repo, we implement asset ratings for stablecoins. The targeted assets are the following 10 stablecoins widely used in the Ethereum mainnet.

Symbol | Address
--- | ---
USDC | [0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48](https://etherscan.io/token/0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48)
USDT | [0xdac17f958d2ee523a2206206994597c13d831ec7](https://etherscan.io/token/0xdac17f958d2ee523a2206206994597c13d831ec7)
DAI | [0x6b175474e89094c44da98b954eedeac495271d0f](https://etherscan.io/token/0x6b175474e89094c44da98b954eedeac495271d0f)
TUSD | [0x0000000000085d4780B73119b644AE5ecd22b376](https://etherscan.io/token/0x0000000000085d4780B73119b644AE5ecd22b376)
FRAX | [0x853d955acef822db058eb8505911ed77f175b99e](https://etherscan.io/token/0x853d955acef822db058eb8505911ed77f175b99e)
LUSD | [0x5f98805A4E8be255a32880FDeC7F6728C6568bA0](https://etherscan.io/token/0x5f98805A4E8be255a32880FDeC7F6728C6568bA0)
crvUSD | [0xf939E0A03FB07F59A73314E73794Be0E57ac1b4E](https://etherscan.io/token/0xf939E0A03FB07F59A73314E73794Be0E57ac1b4E)
GHO | [0x40D16FC0246aD3160Ccc09B8D0D3A2cD28aE6C2f](https://etherscan.io/token/0x40D16FC0246aD3160Ccc09B8D0D3A2cD28aE6C2f)
sUSD | [0x57ab1ec28d129707052df4df418d58a2d46d5f51](https://etherscan.io/token/0x57ab1ec28d129707052df4df418d58a2d46d5f51)
MIM | [0x99d8a9c45b2eca8864373a26d1459e3dff1e17f3](https://etherscan.io/token/0x99d8a9c45b2eca8864373a26d1459e3dff1e17f3)

## Methodology

In this implementation, we evaluate each stablecoin in terms of how many people use and how much they use it. We use the following metrics to measure the usage of each stablecoin.

1. Decentralization of holders

    We need to determine if anyone who holds the stablecoin has an unfair advantage. The stablecoin is distributed among many people, which makes it difficult for any one person to manipulate its price. Therefore, if the coin is held by a large number of people in an even manner, it will receive a higher score. To measure this, we use the Herfindahl-Hirschman Index (HHI).

2. Transaction Structure

    It is not enough for a stablecoin to have a large number of owners if it is only being used by a limited group of people. To ensure proper distribution, we use the Gini Coefficient to measure whether the coins are being circulated among a diverse group of users. This is done by tracking adjacent addresses and evaluating the network in which the coins are being used.

3. Daily Transaction Volume

    Measuring the daily transaction volume of a cryptocurrency is a straightforward approach to gauge its demand in the market. However, this data is also the most susceptible to manipulation by certain individuals. Therefore, it is advisable to evaluate coins in combination with other indicators, rather than solely relying on this indicator.

## How to measure

From a gas cost perspective, it is not efficient to track all prevous ERC-20 Transfer events so far. Instead, we will track all transactions that tool place over a 24-hour period, thus recoding daily results from the start date of the measurement to today on a daily basis.

It is challenging to make an absolute assessment based on the mentioned items alone. We can only provide a relative evaluation based on the ten items that need to be assessed. 

Let's simulate the scoring of stablecoins using random values. Please note that these values are not actual and are only for demonstration purposes.

Daily measurements of Stablecoin and Daily Tx Volume would produce the following chart. These will be accumulated on Chainsight as raw data, but they are only one of the evaluation items.

<img width="905" alt="Screenshot 2024-01-23 at 22 26 07" src="https://github.com/horizonx-tech/chainsight-rating-stablecoin/assets/108332185/08d13590-2ec1-4765-bc0f-900f1d0dc062">

<img width="908" alt="Screenshot 2024-01-23 at 22 26 18" src="https://github.com/horizonx-tech/chainsight-rating-stablecoin/assets/108332185/63d06bfa-becf-484f-9d5c-6bf536b7c74e">

The ultimate assessment must be presented in relative measures, considering all available data. As this rating measures the level of demand for a stablecoin, with a broad range of users frequently utilizing it, we can objectively state that a stablecoin with a score of 10 is the most prevalent one according to our criteria.

<img width="939" alt="Screenshot 2024-01-23 at 22 25 43" src="https://github.com/horizonx-tech/chainsight-rating-stablecoin/assets/108332185/8c894bde-af09-4cd8-9d93-01f46636f4c7">

