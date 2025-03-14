#!/bin/bash

# Configuration
PROXY="https://devnet-gateway.multiversx.com"
CHAIN="D"
PEM_FILE="/home/mehdi/Desktop/smart-contract/multiversx_projets/wallet/wallet.pem"
ALICE_WALLET="/home/mehdi/Desktop/smart-contract/multiversx_projets/wallet/walletAlice.pem"
GAS_LIMIT=60000000

# Adresse du contrat (à remplir après le déploiement)
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqcecutl5axtqejqsuyyatwn7xvdftjh5ule3seyapst"

# Fonction pour afficher l'aide
function display_help {
    echo "World Forge - NFT Minter sur MultiversX"
    echo ""
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  deploy                   - Déploie le contrat"
    echo "  issue-token <nom> <ticker> - Crée une collection NFT"
    echo "  set-roles               - Attribue les rôles nécessaires"
    echo "  create-nft <nom> <royalties> <uri> <prix> - Crée un NFT et le met en vente"
    echo "  buy-nft <nonce>         - Achète un NFT"
    echo "  claim-royalties <marketplace> <token> <nonce> - Réclame les royalties"
    echo "  get-token-id            - Récupère l'identifiant du token NFT"
    echo "  get-nft-price <nonce>   - Récupère le prix d'un NFT"
    echo "  upgrade               - Met à jour le contrat"
    echo "  update-nft-price <nonce> <nouveau_prix> - Met à jour le prix d'un NFT"
    echo "  help                    - Affiche cette aide"
    echo ""
}

# Fonction pour déployer le contrat
function deploy {
    echo "Déploiement du contrat..."
    
    RESULT=$(mxpy contract deploy --bytecode="output/world-forge.wasm" --pem=$PEM_FILE --gas-limit=$GAS_LIMIT --proxy=$PROXY --chain=$CHAIN --recall-nonce --send)
    
    # Extraire l'adresse du contrat
    CONTRACT_ADDRESS=$(echo "$RESULT" | grep -oP 'contract address: \K[a-zA-Z0-9]+')
    
    echo "Contrat déployé à l'adresse: $CONTRACT_ADDRESS"
    echo "Veuillez mettre à jour la variable CONTRACT_ADDRESS dans ce script."
}

# Fonction pour créer une collection NFT
function issue_token {
    if [ -z "$1" ] || [ -z "$2" ]; then
        echo "Erreur: Veuillez spécifier le nom et le ticker de la collection."
        echo "Usage: $0 issue-token <nom> <ticker>"
        exit 1
    fi
    
    echo "Création de la collection NFT '$1' avec le ticker '$2'..."
    
    mxpy contract call $CONTRACT_ADDRESS --function="issueToken" --value=50000000000000000 --pem=$PEM_FILE --gas-limit=$GAS_LIMIT --proxy=$PROXY --chain=$CHAIN --arguments str:"$1" str:"$2" --recall-nonce --send
}

# Fonction pour attribuer les rôles
function set_roles {
    echo "Attribution des rôles..."
    
    mxpy contract call $CONTRACT_ADDRESS --function="setLocalRoles" --pem=$PEM_FILE --gas-limit=$GAS_LIMIT --proxy=$PROXY --chain=$CHAIN --recall-nonce --send
}

# Fonction pour créer un NFT
function create_nft {
    if [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ] || [ -z "$4" ]; then
        echo "Erreur: Veuillez spécifier tous les paramètres."
        echo "Usage: $0 create-nft <nom> <royalties> <uri> <prix>"
        exit 1
    fi
    
    echo "Création du NFT '$1' avec $2 royalties, URI '$3' et prix $4..."
    
    mxpy contract call $CONTRACT_ADDRESS --function="createNft" --pem=$PEM_FILE --gas-limit=$GAS_LIMIT --proxy=$PROXY --chain=$CHAIN --arguments str:"$1" $2 str:"$3" $4 --recall-nonce --send
}

# Fonction pour acheter un NFT
function buy_nft {
    if [ -z "$1" ] || [ -z "$2" ]; then
        echo "Erreur: Veuillez spécifier le nonce du NFT et le prix."
        echo "Usage: $0 buy-nft <nonce> <prix>"
        exit 1
    fi
    
    echo "Achat du NFT avec le nonce $1 pour $2 EGLD..."
    
    mxpy contract call $CONTRACT_ADDRESS --function="buyNft" --value=$2 --pem=$ALICE_WALLET --gas-limit=$GAS_LIMIT --proxy=$PROXY --chain=$CHAIN --arguments $1 --recall-nonce --send
}

# Fonction pour réclamer les royalties
function claim_royalties {
    if [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ]; then
        echo "Erreur: Veuillez spécifier tous les paramètres."
        echo "Usage: $0 claim-royalties <marketplace> <token> <nonce>"
        exit 1
    fi
    
    echo "Réclamation des royalties du marketplace $1 pour le token $2 avec le nonce $3..."
    
    mxpy contract call $CONTRACT_ADDRESS --function="claimRoyaltiesFromMarketplace" --pem=$PEM_FILE --gas-limit=$GAS_LIMIT --proxy=$PROXY --chain=$CHAIN --arguments $1 str:"$2" $3 --recall-nonce --send
}

# Fonction pour récupérer l'identifiant du token NFT
function get_token_id {
    echo "Récupération de l'identifiant du token NFT..."
    
    mxpy contract query $CONTRACT_ADDRESS --function="getNftTokenId" --proxy=$PROXY
}

# Fonction pour récupérer le prix d'un NFT
function get_nft_price {
    if [ -z "$1" ]; then
        echo "Erreur: Veuillez spécifier le nonce du NFT."
        echo "Usage: $0 get-nft-price <nonce>"
        exit 1
    fi
    
    echo "Récupération du prix du NFT avec le nonce $1..."
    
    mxpy contract query $CONTRACT_ADDRESS --function="getNftPrice" --proxy=$PROXY --arguments $1
}

# Fonction pour mettre à jour le contrat
function upgrade {
    echo "Mise à jour du contrat..."
    
    mxpy --verbose contract upgrade $CONTRACT_ADDRESS --recall-nonce \
        --bytecode="output/world-forge.wasm" \
        --metadata-payable \
        --pem=$PEM_FILE \
        --gas-limit=$GAS_LIMIT \
        --proxy=$PROXY \
        --chain=$CHAIN \
        --send || return
}

# Fonction pour mettre à jour le prix d'un NFT
function update_nft_price {
    if [ -z "$1" ] || [ -z "$2" ]; then
        echo "Erreur: Veuillez spécifier le nonce du NFT et le nouveau prix."
        echo "Usage: $0 update-nft-price <nonce> <nouveau_prix>"
        exit 1
    fi
    
    echo "Mise à jour du prix du NFT avec le nonce $1 à $2..."
    
    mxpy contract call $CONTRACT_ADDRESS --function="updateNftPrice" --pem=$PEM_FILE --gas-limit=$GAS_LIMIT --proxy=$PROXY --chain=$CHAIN --arguments $1 $2 --recall-nonce --send
}

# Traitement des commandes
case "$1" in
    deploy)
        deploy
        ;;
    upgrade)
        upgrade
        ;;
    update-nft-price)
        update_nft_price "$2" "$3"
        ;;
    issue-token)
        issue_token "$2" "$3"
        ;;
    set-roles)
        set_roles
        ;;
    create-nft)
        create_nft "$2" "$3" "$4" "$5"
        ;;
    buy-nft)
        buy_nft "$2" "$3"
        ;;
    claim-royalties)
        claim_royalties "$2" "$3" "$4"
        ;;
    get-token-id)
        get_token_id
        ;;
    get-nft-price)
        get_nft_price "$2"
        ;;
    help|*)
        display_help
        ;;
esac 