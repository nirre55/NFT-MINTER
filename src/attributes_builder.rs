multiversx_sc::imports!();

const IPFS_GATEWAY_HOST: &[u8] = "https://ipfs.io/ipfs/".as_bytes();
const METADATA_KEY_NAME: &[u8] = "metadata:".as_bytes();
const METADATA_FILE_EXTENSION: &[u8] = ".json".as_bytes();
const ATTR_SEPARATOR: &[u8] = ";".as_bytes();
const URI_SLASH: &[u8] = "/".as_bytes();
const TAGS_KEY_NAME: &[u8] = "tags:".as_bytes();

use crate::storage;



#[multiversx_sc::module]
pub trait AttributesBuilder: storage::StorageModule {

    // private
    fn build_uris_vec(&self, index_to_mint: usize) -> ManagedVec<ManagedBuffer> {
        let mut uris = ManagedVec::new();
        let file_index = self.decimal_to_ascii(index_to_mint.try_into().unwrap());
        let image_cid = self.image_base_cid().get();
        let uri_slash = ManagedBuffer::new_from_bytes(URI_SLASH);
        let image_file_extension = self.file_extension().get();
        
        // Construction de l'URI d'image
        let mut img_ipfs_gateway_uri = ManagedBuffer::new_from_bytes(IPFS_GATEWAY_HOST);
        img_ipfs_gateway_uri.append(&image_cid);
        img_ipfs_gateway_uri.append(&uri_slash);
        img_ipfs_gateway_uri.append(&file_index);
        img_ipfs_gateway_uri.append(&image_file_extension);
        uris.push(img_ipfs_gateway_uri);
        
        // Ajout de l'URI de métadonnées 
        let metadata_cid = self.metadata_base_cid().get();
        let metadata_file_extension = ManagedBuffer::new_from_bytes(METADATA_FILE_EXTENSION);
        
        let mut ipfs_metadata_uri = ManagedBuffer::new_from_bytes(IPFS_GATEWAY_HOST);
        ipfs_metadata_uri.append(&metadata_cid);
        ipfs_metadata_uri.append(&uri_slash);
        ipfs_metadata_uri.append(&file_index);
        ipfs_metadata_uri.append(&metadata_file_extension);
        uris.push(ipfs_metadata_uri);
        
        uris
    }
    
    fn build_attributes_buffer(&self, index_to_mint: usize) -> ManagedBuffer {
        let index_file = self.decimal_to_ascii(index_to_mint.try_into().unwrap());
        let metadata_cid = self.metadata_base_cid().get();
        
        // Création du buffer d'attributs en une seule fois
        let mut attributes = ManagedBuffer::new();
        
        // Ajout des tags
        attributes.append(&ManagedBuffer::new_from_bytes(TAGS_KEY_NAME));
        attributes.append(&self.tags().get());
        attributes.append(&ManagedBuffer::new_from_bytes(ATTR_SEPARATOR));
        
        // Ajout des métadonnées
        attributes.append(&ManagedBuffer::new_from_bytes(METADATA_KEY_NAME));
        attributes.append(&metadata_cid);
        attributes.append(&ManagedBuffer::new_from_bytes(URI_SLASH));
        attributes.append(&index_file);
        attributes.append(&ManagedBuffer::new_from_bytes(METADATA_FILE_EXTENSION));
        
        attributes
    }
    
    // fn build_token_name_buffer(&self, index_to_mint: usize) -> ManagedBuffer {
    //     let mut full_token_name = self.nft_token_name().get();
        
    //     if !self.no_number_in_nft_name().get() {
    //         let token_index = self.decimal_to_ascii(index_to_mint.try_into().unwrap());
    //         let hash_and_space_sign = ManagedBuffer::new_from_bytes(" #".as_bytes());
    //         full_token_name.append(&hash_and_space_sign);
    //         full_token_name.append(&token_index);
    //     }
        
    //     full_token_name
    // }
    
    fn decimal_to_ascii(&self, mut number: u32) -> ManagedBuffer {
        let mut buffer = [0u8; 10]; // Supposons un nombre de 10 chiffres max
        let mut pos = buffer.len();
    
        if number == 0 {
            return ManagedBuffer::new_from_bytes(&[b'0']);
        }
    
        while number > 0 {
            pos -= 1;
            buffer[pos] = b'0' + (number % 10) as u8;
            number /= 10;
        }
    
        ManagedBuffer::new_from_bytes(&buffer[pos..])
    }
}