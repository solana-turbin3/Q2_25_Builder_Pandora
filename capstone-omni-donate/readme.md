# 🌍 Omni Donate

**Amplifying Impact Through Parametric Conditional Donations**  
A decentralized donation protocol that ensures funds are released only when verifiable real-world conditions are met.

---

## 🔗 Overview

Omni Donate is a smart contract-powered platform built on Solana that enables transparent, data-driven donations. It leverages programmatic escrow and oracle-fed environmental data (e.g., NDVI) to release funds only when specific conditions occur, such as droughts. This ensures that aid is delivered efficiently, securely, and only when needed.

---

## 🚀 Features

- **Programmatic Escrow**  
  Donations are securely locked in smart contracts until the trigger condition is met.

- **Parametric Triggers**  
  Oracle-monitored metrics like NDVI and rainfall data determine when funds are released.

- **Transparent On-Chain Logic**  
  All campaign actions, triggers, and disbursements are publicly viewable on Solana.

- **Mobile Disbursement Support**  
  Beneficiaries can receive aid via custodians or local mobile money platforms like M-Pesa.

---

## 🛠️ Tech Stack

- **Blockchain:** Solana  
- **Smart Contract Language:** Rust  
- **Framework:** Anchor  
- **Oracle:** OffChain Script 
- **Token Standard:** SPL (e.g., USDC)

---

## 🧩 Program Modules

- `CampaignManager` — Sets up campaign parameters and manages lifecycle.  
- `SmartVault` — Holds and releases donor funds based on oracle conditions.   
- `Claim.rs` — Handles beneficiary claims post-trigger.

---

## 🔄 Flow Overview

1. **Campaign Created**  
   A sponsor defines a donation campaign, including environmental trigger conditions.

2. **Donors Contribute**  
   Donors deposit tokens into a programmatic escrow contract.

3. **Oracle Monitors Conditions**  
   Trusted oracles monitor metrics in real-time.

4. **Trigger Activated**  
   When conditions are met, funds are automatically released.

5. **Beneficiaries Claim Funds**  
   Funds are claimed directly by Beneficiary

---

## 🧪 Development

```bash
git clone https://github.com/cdpandora/omni-donate.git
cd omni-donate
anchor build
anchor test
