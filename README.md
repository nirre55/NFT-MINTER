# World Forge - NFT Minter sur MultiversX

Ce projet est un contrat intelligent pour la blockchain MultiversX qui permet de créer et de vendre des NFTs (Non-Fungible Tokens). Il est basé sur le framework MultiversX SC et implémente les fonctionnalités essentielles d'un NFT minter.

## Fonctionnalités

- Création d'une collection NFT
- Attribution des rôles nécessaires pour la création de NFTs
- Création de NFTs avec des royalties personnalisables
- Mise en vente des NFTs créés
- Achat de NFTs par les utilisateurs
- Réclamation des royalties des marketplaces

## Prérequis

- [mxpy](https://docs.multiversx.com/sdk-and-tools/sdk-py/) - L'outil en ligne de commande MultiversX
- [Rust](https://www.rust-lang.org/tools/install) - Le langage de programmation utilisé pour développer le contrat
- [wasm-opt](https://github.com/WebAssembly/binaryen) - Pour optimiser le bytecode WebAssembly

## Installation

1. Clonez ce dépôt :
```bash
git clone https://github.com/votre-utilisateur/world-forge.git
cd world-forge
```

2. Compilez le contrat :
```bash
mxpy contract build
```

## Déploiement

Pour déployer le contrat sur le devnet de MultiversX :

```bash
mxpy contract deploy --bytecode=output/world-forge.wasm --pem=wallet.pem --gas-limit=60000000 --proxy=https://devnet-gateway.multiversx.com --chain=D --recall-nonce --send
```

## Utilisation

### 1. Création d'une collection NFT

Pour créer une collection NFT, vous devez appeler la fonction `issueToken` avec un paiement de 0.05 EGLD :

```bash
mxpy contract call <adresse-du-contrat> --function=issueToken --value=50000000000000000 --pem=wallet.pem --gas-limit=60000000 --proxy=https://devnet-gateway.multiversx.com --chain=D --arguments str:NomCollection str:TICKER --recall-nonce --send
```

### 2. Attribution des rôles

Après avoir créé la collection, vous devez attribuer les rôles nécessaires pour la création de NFTs :

```bash
mxpy contract call <adresse-du-contrat> --function=setLocalRoles --pem=wallet.pem --gas-limit=60000000 --proxy=https://devnet-gateway.multiversx.com --chain=D --recall-nonce --send
```

### 3. Création d'un NFT

Pour créer un NFT et le mettre en vente :

```bash
mxpy contract call <adresse-du-contrat> --function=createNft --pem=wallet.pem --gas-limit=60000000 --proxy=https://devnet-gateway.multiversx.com --chain=D --arguments str:NomNFT 500 str:https://ipfs.io/ipfs/QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG 1000000000000000000 --recall-nonce --send
```

Les arguments sont :
- `NomNFT` : Le nom du NFT
- `500` : Les royalties (5.00%)
- `https://ipfs.io/ipfs/QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG` : L'URI du NFT (image, vidéo, etc.)
- `1000000000000000000` : Le prix de vente (1 EGLD)

### 4. Achat d'un NFT

Pour acheter un NFT, l'utilisateur doit appeler la fonction `buyNft` avec le paiement correspondant :

```bash
mxpy contract call <adresse-du-contrat> --function=buyNft --value=1000000000000000000 --pem=wallet-acheteur.pem --gas-limit=60000000 --proxy=https://devnet-gateway.multiversx.com --chain=D --arguments 1 --recall-nonce --send
```

L'argument `1` correspond au nonce du NFT à acheter.

### 5. Réclamation des royalties

Pour réclamer les royalties d'un marketplace :

```bash
mxpy contract call <adresse-du-contrat> --function=claimRoyaltiesFromMarketplace --pem=wallet.pem --gas-limit=60000000 --proxy=https://devnet-gateway.multiversx.com --chain=D --arguments erd1qqqqqqqqqqqqqpgqd77fnev2sthnczp2lnfx0y58ar3ywcyj0n4s2gctac EGLD-abcdef 0 --recall-nonce --send
```

Les arguments sont :
- `erd1qqqqqqqqqqqqqpgqd77fnev2sthnczp2lnfx0y58ar3ywcyj0n4s2gctac` : L'adresse du marketplace
- `EGLD-abcdef` : L'identifiant du token utilisé pour le paiement
- `0` : Le nonce du token utilisé pour le paiement

## Licence

Ce projet est sous licence MIT.

## Contribution

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou à soumettre une pull request. 