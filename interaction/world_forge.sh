#!/bin/bash

# Configuration
PROXY="https://devnet-gateway.multiversx.com"
CHAIN="D"
PEM_FILE="/home/mehdi/Desktop/smart-contract/multiversx_projets/wallet/wallet.pem"
ALICE_WALLET="/home/mehdi/Desktop/smart-contract/multiversx_projets/wallet/walletAlice.pem"
GAS_LIMIT=600000000



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
    echo "  create-nft <nom> <prix> - Crée un NFT et le met en vente"
    echo "  buy-nft <nonce>         - Achète un NFT"
    echo "  upgrade               - Met à jour le contrat"
    echo "  create-nft-esdt <nom> <prix_esdt> <token_id> - Crée un NFT avec prix en ESDT"
    echo "  buy-nft-esdt <token> <prix> <nonce> - Achète un NFT avec prix en ESDT"
    echo "  get-rarety-storage <nom> - Récupère le storage du contrat"
    echo "  clean-all-storage       - Nettoie tout le storage"
    echo "  fill-all               - Remplit tout le storage"
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
    if [ -z "$1" ] || [ -z "$2" ]; then
        echo "Erreur: Veuillez spécifier tous les paramètres."
        echo "Usage: $0 create-nft <nom> <royalties> <uri> <prix>"
        exit 1
    fi
    
    echo "Création du NFT '$1' avec un prix $2..."
    
    mxpy contract call $CONTRACT_ADDRESS --function="createNft" --pem=$PEM_FILE --gas-limit=$GAS_LIMIT --proxy=$PROXY --chain=$CHAIN --arguments str:"$1" $2 --recall-nonce --send
}

# Fonction pour créer un NFT avec prix en USDC
function create_nft_esdt {
    if [ -z "$1" ] || [ -z "$2" ]  || [ -z "$3" ]; then
        echo "Erreur: Veuillez spécifier tous les paramètres."
        echo "Usage: $0 create-nft-esdt <nom> <prix_esdt> <token_id>"
        exit 1
    fi
    
    echo "Création du NFT '$1' prix $2 pour le token $3..."
    
    mxpy contract call $CONTRACT_ADDRESS \
        --function="createNft" \
        --pem=$PEM_FILE \
        --gas-limit=$GAS_LIMIT \
        --proxy=$PROXY \
        --chain=$CHAIN \
        --arguments str:"$1" $2 str:"$3" 0 \
        --recall-nonce \
        --send
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

# Fonction pour acheter un NFT avec prix en ESDT
function buy_nft_esdt {
    if [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ]; then
        echo "Erreur: Veuillez spécifier l'identifiant du token et le prix et nonce du NFT."
        echo "Usage: $0 buy-nft-esdt <token> <prix> <nonce>"
        exit 1
    fi
    
    echo "Achat du NFT avec le nonce $3 pour $2 ESDT TOKEN $1..."
    
    mxpy contract call $CONTRACT_ADDRESS --function="ESDTTransfer" --pem=$ALICE_WALLET --gas-limit=$GAS_LIMIT --proxy=$PROXY --chain=$CHAIN --arguments str:"$1" $2 str:"buyNft" $3 --recall-nonce --send || return
}

# Fonction pour mettre à jour le contrat
function upgrade {
    echo "Mise à jour du contrat..."
    
    mxpy contract upgrade $CONTRACT_ADDRESS --bytecode="output/world-forge.wasm" --pem=$PEM_FILE --metadata-payable --gas-limit=$GAS_LIMIT --proxy=$PROXY --chain=$CHAIN --recall-nonce --send
}

# Fonction pour récupérer le storage du contrat
function get_rarety_storage {
    if [ -z "$1" ]; then
        echo "Erreur: Veuillez spécifier le nom de View."
        echo "Usage: $0 get-rarety-storage"
        exit 1
    fi
    echo "recuperation du storage..."
    
    mxpy contract query $CONTRACT_ADDRESS --function="$1" --proxy=$PROXY 
}

# Fonction pour nettoyer tout les rarety storage
function clean_all_storage {
    echo "Nettoyage de tout le storage..."
    
    mxpy contract call $CONTRACT_ADDRESS --function="clearAllStorage" --pem=$PEM_FILE --gas-limit=$GAS_LIMIT --proxy=$PROXY --chain=$CHAIN --recall-nonce --send
}

# Fonction pour remplir tout les rarety storages
function fill_all {
    echo "Remplissage de tout les storage..."
    
    mxpy contract call $CONTRACT_ADDRESS --function="fillAll" --pem=$PEM_FILE --gas-limit=$GAS_LIMIT --proxy=$PROXY --chain=$CHAIN --recall-nonce --send
}


# Traitement des commandes
case "$1" in
    deploy)
        deploy
        ;;
    upgrade)
        upgrade
        ;;
    issue-token)
        issue_token "$2" "$3"
        ;;
    set-roles)
        set_roles
        ;;
    create-nft)
        create_nft "$2" "$3"
        ;;
    buy-nft)
        buy_nft "$2" "$3"
        ;;
    create-nft-esdt)
        create_nft_esdt "$2" "$3" "$4" 
        ;;
    buy-nft-esdt)
        buy_nft_esdt "$2" "$3" "$4"
        ;;
    get-rarety-storage)
        get_rarety_storage "$2"
        ;;
    clean-all-storage)
        clean_all_storage
        ;;
    fill-all)
        fill_all
        ;;
    help|*)
        display_help
        ;;
esac 