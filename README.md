<h1 align="center">
  Compressed NFT 404
</h1>
<p>
  This program implements the 404 dynamic NFT mechanism for CNFTs. It includes functionalities for initializing pools, depositing tokens and CNFTs, swapping tokens for CNFTs, and vice versa.
</p>

## Overview

Compressed NFT 404 is a Solana-based program designed to implement the 404 dynamic NFT mechanism for Compressed NFTs (CNFTs). This program allows users to interact with CNFTs and tokens through various functionalities, including:

- Initializing Pools: Set up pools that can hold CNFTs and tokens.
- Depositing Tokens and CNFTs: Add tokens and CNFTs to the pools.
- Swapping Tokens for CNFTs: Exchange tokens for CNFTs within the pool.
- Swapping CNFTs for Tokens: Get tokens for your CNFTs.

## Example Usecases

As Drip is the biggest issuer of CNFTs on Solana, here are some use cases based on its mechanism:
Single Creator Pool: A Drip Creator can set up a 404 pool with CNFTs from their collection.
Collaborative Pool: Multiple Drip Creators can partner to create a single pool with CNFTs and a token at a set price. The token could be a memecoin or a new T22 token.
Legendary CNFT: A Drip Creator could seed the pool with legendary CNFTs and let the users decide who gets it through the re-roll mechanism.

## Technical Overview

The Anchor program contains the following instructions :-

`init_pool` : Initializes the Pool Account for a give authority.

`deposit_cnft`: Deposits the Initial cnft liquidity in the pool.

`deposit_token`: Deposits the SPL token liquidity in the pool.

`swap_cnft_to_token`: Given a user deposits a cnft in the pool the pool gives the user tokens.

`swap_token_to_cnft`: Given a user deposits tokens in the pool this instruction generates a random no and creates a coupon PDA to claim the cnft.

`claim_cnft`: the user get's the cnft via this instruction. It is the part of the process for `swap_token_to_cnft`.

## Prerequisites

- Node.js
- Yarn
- Rust
- Solana CLI
   
### Installation

1. Clone the repository:
   ```bash
   $ git clone <repository-url>
   $ cd <repository-directory>
   ```
2. Install Dependencies:
   ```bash
   $ yarn install
   ```
4. Build the Anchor Program:
   ```bash
   $ anchor build
   ```

## Disclaimer

This code is provided for experimental purposes only. Use it at your own risk. The authors and contributors are not responsible for any damages or losses that may arise from using this code. It is recommended to thoroughly review and test the code in a safe environment before deploying it in any production system.

