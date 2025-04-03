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

    fn add_nft_name(&self, index_input: usize) {
        // Ajoute la nouvelle entrée 
        let value_input = self.build_token_name_buffer(index_input);
        self.nft_name(index_input).set_if_empty(value_input);
    }

    #[view(getNftName)]
    fn get_nft_name(&self, storage_index: usize) -> ManagedBuffer {
        let nft_name = self.nft_name(storage_index);
        require!(!nft_name.is_empty(), "Index out of bounds");
        
        nft_name.get()
    }

    //TODO : a modifier plus tard ou supprimer une fois le storage remplie
    fn build_token_name_buffer(&self, index_input: usize) -> ManagedBuffer {
        
        // Utilisation de match pour analyser le contenu du message
        let name = match index_input {
            1 => ManagedBuffer::new_from_bytes(b"Afghanistan"),
            10 => ManagedBuffer::new_from_bytes(b"Antarctica"),
            100 => ManagedBuffer::new_from_bytes(b"Guyana"),
            101 => ManagedBuffer::new_from_bytes(b"Haiti"),
            102 => ManagedBuffer::new_from_bytes(b"Heard I and McDonald Is"),
            103 => ManagedBuffer::new_from_bytes(b"Honduras"),
            104 => ManagedBuffer::new_from_bytes(b"Hong Kong"),
            105 => ManagedBuffer::new_from_bytes(b"Hungary"),
            106 => ManagedBuffer::new_from_bytes(b"Iceland"),
            107 => ManagedBuffer::new_from_bytes(b"India"),
            108 => ManagedBuffer::new_from_bytes(b"Indian Ocean Ter"),
            109 => ManagedBuffer::new_from_bytes(b"Indonesia"),
            11 => ManagedBuffer::new_from_bytes(b"Antigua and Barb"),
            110 => ManagedBuffer::new_from_bytes(b"Iran"),
            111 => ManagedBuffer::new_from_bytes(b"Iraq"),
            112 => ManagedBuffer::new_from_bytes(b"Ireland"),
            113 => ManagedBuffer::new_from_bytes(b"Isle of Man"),
            114 => ManagedBuffer::new_from_bytes(b"Israel"),
            115 => ManagedBuffer::new_from_bytes(b"Italy"),
            116 => ManagedBuffer::new_from_bytes(b"Jamaica"),
            117 => ManagedBuffer::new_from_bytes(b"Japan"),
            118 => ManagedBuffer::new_from_bytes(b"Jersey"),
            119 => ManagedBuffer::new_from_bytes(b"Jordan"),
            12 => ManagedBuffer::new_from_bytes(b"Argentina"),
            120 => ManagedBuffer::new_from_bytes(b"Kazakhstan"),
            121 => ManagedBuffer::new_from_bytes(b"Kenya"),
            122 => ManagedBuffer::new_from_bytes(b"Kiribati"),
            123 => ManagedBuffer::new_from_bytes(b"Kosovo"),
            124 => ManagedBuffer::new_from_bytes(b"Kuwait"),
            125 => ManagedBuffer::new_from_bytes(b"Kyrgyzstan"),
            126 => ManagedBuffer::new_from_bytes(b"Laos"),
            127 => ManagedBuffer::new_from_bytes(b"Latvia"),
            128 => ManagedBuffer::new_from_bytes(b"Lebanon"),
            129 => ManagedBuffer::new_from_bytes(b"Lesotho"),
            13 => ManagedBuffer::new_from_bytes(b"Armenia"),
            130 => ManagedBuffer::new_from_bytes(b"Liberia"),
            131 => ManagedBuffer::new_from_bytes(b"Libya"),
            132 => ManagedBuffer::new_from_bytes(b"Liechtenstein"),
            133 => ManagedBuffer::new_from_bytes(b"Lithuania"),
            134 => ManagedBuffer::new_from_bytes(b"Luxembourg"),
            135 => ManagedBuffer::new_from_bytes(b"Macao"),
            136 => ManagedBuffer::new_from_bytes(b"Madagascar"),
            137 => ManagedBuffer::new_from_bytes(b"Malawi"),
            138 => ManagedBuffer::new_from_bytes(b"Malaysia"),
            139 => ManagedBuffer::new_from_bytes(b"Maldives"),
            14 => ManagedBuffer::new_from_bytes(b"Aruba"),
            140 => ManagedBuffer::new_from_bytes(b"Mali"),
            141 => ManagedBuffer::new_from_bytes(b"Malta"),
            142 => ManagedBuffer::new_from_bytes(b"Marshall Is"),
            143 => ManagedBuffer::new_from_bytes(b"Mauritania"),
            144 => ManagedBuffer::new_from_bytes(b"Mauritius"),
            145 => ManagedBuffer::new_from_bytes(b"Mexico"),
            146 => ManagedBuffer::new_from_bytes(b"Micronesia"),
            147 => ManagedBuffer::new_from_bytes(b"Moldova"),
            148 => ManagedBuffer::new_from_bytes(b"Monaco"),
            149 => ManagedBuffer::new_from_bytes(b"Mongolia"),
            15 => ManagedBuffer::new_from_bytes(b"Ashmore and Cartier Is"),
            150 => ManagedBuffer::new_from_bytes(b"Montenegro"),
            151 => ManagedBuffer::new_from_bytes(b"Montserrat"),
            152 => ManagedBuffer::new_from_bytes(b"Morocco"),
            153 => ManagedBuffer::new_from_bytes(b"Mozambique"),
            154 => ManagedBuffer::new_from_bytes(b"Myanmar"),
            155 => ManagedBuffer::new_from_bytes(b"N Cyprus"),
            156 => ManagedBuffer::new_from_bytes(b"N Mariana Is"),
            157 => ManagedBuffer::new_from_bytes(b"Namibia"),
            158 => ManagedBuffer::new_from_bytes(b"Nauru"),
            159 => ManagedBuffer::new_from_bytes(b"Nepal"),
            16 => ManagedBuffer::new_from_bytes(b"Australia"),
            160 => ManagedBuffer::new_from_bytes(b"Netherlands"),
            161 => ManagedBuffer::new_from_bytes(b"New Caledonia"),
            162 => ManagedBuffer::new_from_bytes(b"New Zealand"),
            163 => ManagedBuffer::new_from_bytes(b"Nicaragua"),
            164 => ManagedBuffer::new_from_bytes(b"Niger"),
            165 => ManagedBuffer::new_from_bytes(b"Nigeria"),
            166 => ManagedBuffer::new_from_bytes(b"Niue"),
            167 => ManagedBuffer::new_from_bytes(b"Norfolk Island"),
            168 => ManagedBuffer::new_from_bytes(b"North Korea"),
            169 => ManagedBuffer::new_from_bytes(b"North Macedonia"),
            17 => ManagedBuffer::new_from_bytes(b"Austria"),
            170 => ManagedBuffer::new_from_bytes(b"Norway"),
            171 => ManagedBuffer::new_from_bytes(b"Oman"),
            172 => ManagedBuffer::new_from_bytes(b"Pakistan"),
            173 => ManagedBuffer::new_from_bytes(b"Palau"),
            174 => ManagedBuffer::new_from_bytes(b"Palestine"),
            175 => ManagedBuffer::new_from_bytes(b"Panama"),
            176 => ManagedBuffer::new_from_bytes(b"Papua New Guinea"),
            177 => ManagedBuffer::new_from_bytes(b"Paraguay"),
            178 => ManagedBuffer::new_from_bytes(b"Peru"),
            179 => ManagedBuffer::new_from_bytes(b"Philippines"),
            18 => ManagedBuffer::new_from_bytes(b"Azerbaijan"),
            180 => ManagedBuffer::new_from_bytes(b"Pitcairn Is"),
            181 => ManagedBuffer::new_from_bytes(b"Poland"),
            182 => ManagedBuffer::new_from_bytes(b"Portugal"),
            183 => ManagedBuffer::new_from_bytes(b"Puerto Rico"),
            184 => ManagedBuffer::new_from_bytes(b"Qatar"),
            185 => ManagedBuffer::new_from_bytes(b"Romania"),
            186 => ManagedBuffer::new_from_bytes(b"Russia"),
            187 => ManagedBuffer::new_from_bytes(b"Rwanda"),
            188 => ManagedBuffer::new_from_bytes(b"S Geo and the Is"),
            189 => ManagedBuffer::new_from_bytes(b"S Sudan"),
            19 => ManagedBuffer::new_from_bytes(b"Bahamas"),
            190 => ManagedBuffer::new_from_bytes(b"Saint Helena"),
            191 => ManagedBuffer::new_from_bytes(b"Saint Lucia"),
            192 => ManagedBuffer::new_from_bytes(b"Samoa"),
            193 => ManagedBuffer::new_from_bytes(b"San Marino"),
            194 => ManagedBuffer::new_from_bytes(b"Sao Tome and Principe"),
            195 => ManagedBuffer::new_from_bytes(b"Saudi Arabia"),
            196 => ManagedBuffer::new_from_bytes(b"Scarborough Reef"),
            197 => ManagedBuffer::new_from_bytes(b"Senegal"),
            198 => ManagedBuffer::new_from_bytes(b"Serbia"),
            199 => ManagedBuffer::new_from_bytes(b"Serranilla Bank"),
            2 => ManagedBuffer::new_from_bytes(b"Akrotiri"),
            20 => ManagedBuffer::new_from_bytes(b"Bahrain"),
            200 => ManagedBuffer::new_from_bytes(b"Seychelles"),
            201 => ManagedBuffer::new_from_bytes(b"Siachen Glacier"),
            202 => ManagedBuffer::new_from_bytes(b"Sierra Leone"),
            203 => ManagedBuffer::new_from_bytes(b"Singapore"),
            204 => ManagedBuffer::new_from_bytes(b"Sint Maarten"),
            205 => ManagedBuffer::new_from_bytes(b"Slovakia"),
            206 => ManagedBuffer::new_from_bytes(b"Slovenia"),
            207 => ManagedBuffer::new_from_bytes(b"Solomon Is"),
            208 => ManagedBuffer::new_from_bytes(b"Somalia"),
            209 => ManagedBuffer::new_from_bytes(b"Somaliland"),
            21 => ManagedBuffer::new_from_bytes(b"Baikonur"),
            210 => ManagedBuffer::new_from_bytes(b"South Africa"),
            211 => ManagedBuffer::new_from_bytes(b"South Korea"),
            212 => ManagedBuffer::new_from_bytes(b"Southern Patagonian Ice Field"),
            213 => ManagedBuffer::new_from_bytes(b"Spain"),
            214 => ManagedBuffer::new_from_bytes(b"Spratly Is"),
            215 => ManagedBuffer::new_from_bytes(b"Sri Lanka"),
            216 => ManagedBuffer::new_from_bytes(b"St Kitts and Nevis"),
            217 => ManagedBuffer::new_from_bytes(b"St Pierre and Miquelon"),
            218 => ManagedBuffer::new_from_bytes(b"St Vin and Gren"),
            219 => ManagedBuffer::new_from_bytes(b"StBarthelemy"),
            22 => ManagedBuffer::new_from_bytes(b"Bajo Nuevo Bank"),
            220 => ManagedBuffer::new_from_bytes(b"StMartin"),
            221 => ManagedBuffer::new_from_bytes(b"Sudan"),
            222 => ManagedBuffer::new_from_bytes(b"Suriname"),
            223 => ManagedBuffer::new_from_bytes(b"Sweden"),
            224 => ManagedBuffer::new_from_bytes(b"Switzerland"),
            225 => ManagedBuffer::new_from_bytes(b"Syria"),
            226 => ManagedBuffer::new_from_bytes(b"Taiwan"),
            227 => ManagedBuffer::new_from_bytes(b"Tajikistan"),
            228 => ManagedBuffer::new_from_bytes(b"Tanzania"),
            229 => ManagedBuffer::new_from_bytes(b"Thailand"),
            23 => ManagedBuffer::new_from_bytes(b"Bangladesh"),
            230 => ManagedBuffer::new_from_bytes(b"TimorLeste"),
            231 => ManagedBuffer::new_from_bytes(b"Togo"),
            232 => ManagedBuffer::new_from_bytes(b"Tonga"),
            233 => ManagedBuffer::new_from_bytes(b"Trinidad and Tobago"),
            234 => ManagedBuffer::new_from_bytes(b"Tunisia"),
            235 => ManagedBuffer::new_from_bytes(b"Turkey"),
            236 => ManagedBuffer::new_from_bytes(b"Turkmenistan"),
            237 => ManagedBuffer::new_from_bytes(b"Turks and Caicos Is"),
            238 => ManagedBuffer::new_from_bytes(b"Tuvalu"),
            239 => ManagedBuffer::new_from_bytes(b"US Minor Outlying Is"),
            24 => ManagedBuffer::new_from_bytes(b"Barbados"),
            240 => ManagedBuffer::new_from_bytes(b"US Virgin Is"),
            241 => ManagedBuffer::new_from_bytes(b"USNB Guantanamo Bay"),
            242 => ManagedBuffer::new_from_bytes(b"Uganda"),
            243 => ManagedBuffer::new_from_bytes(b"Ukraine"),
            244 => ManagedBuffer::new_from_bytes(b"United Arab Emirates"),
            245 => ManagedBuffer::new_from_bytes(b"United Kingdom"),
            246 => ManagedBuffer::new_from_bytes(b"United States of America"),
            247 => ManagedBuffer::new_from_bytes(b"Uruguay"),
            248 => ManagedBuffer::new_from_bytes(b"Uzbekistan"),
            249 => ManagedBuffer::new_from_bytes(b"Vanuatu"),
            25 => ManagedBuffer::new_from_bytes(b"Belarus"),
            250 => ManagedBuffer::new_from_bytes(b"Vatican"),
            251 => ManagedBuffer::new_from_bytes(b"Venezuela"),
            252 => ManagedBuffer::new_from_bytes(b"Vietnam"),
            253 => ManagedBuffer::new_from_bytes(b"W Sahara"),
            254 => ManagedBuffer::new_from_bytes(b"Wallis and Futuna Is"),
            255 => ManagedBuffer::new_from_bytes(b"Yemen"),
            256 => ManagedBuffer::new_from_bytes(b"Zambia"),
            257 => ManagedBuffer::new_from_bytes(b"Zimbabwe"),
            258 => ManagedBuffer::new_from_bytes(b"eSwatini"),
            26 => ManagedBuffer::new_from_bytes(b"Belgium"),
            27 => ManagedBuffer::new_from_bytes(b"Belize"),
            28 => ManagedBuffer::new_from_bytes(b"Benin"),
            29 => ManagedBuffer::new_from_bytes(b"Bermuda"),
            3 => ManagedBuffer::new_from_bytes(b"Aland"),
            30 => ManagedBuffer::new_from_bytes(b"Bhutan"),
            31 => ManagedBuffer::new_from_bytes(b"Bir Tawil"),
            32 => ManagedBuffer::new_from_bytes(b"Bolivia"),
            33 => ManagedBuffer::new_from_bytes(b"Bosnia and Herz"),
            34 => ManagedBuffer::new_from_bytes(b"Botswana"),
            35 => ManagedBuffer::new_from_bytes(b"Br Indian Ocean Ter"),
            36 => ManagedBuffer::new_from_bytes(b"Brazil"),
            37 => ManagedBuffer::new_from_bytes(b"Brazilian I"),
            38 => ManagedBuffer::new_from_bytes(b"British Virgin Is"),
            39 => ManagedBuffer::new_from_bytes(b"Brunei"),
            4 => ManagedBuffer::new_from_bytes(b"Albania"),
            40 => ManagedBuffer::new_from_bytes(b"Bulgaria"),
            41 => ManagedBuffer::new_from_bytes(b"Burkina Faso"),
            42 => ManagedBuffer::new_from_bytes(b"Burundi"),
            43 => ManagedBuffer::new_from_bytes(b"Cabo Verde"),
            44 => ManagedBuffer::new_from_bytes(b"Cambodia"),
            45 => ManagedBuffer::new_from_bytes(b"Cameroon"),
            46 => ManagedBuffer::new_from_bytes(b"Canada"),
            47 => ManagedBuffer::new_from_bytes(b"Cayman Is"),
            48 => ManagedBuffer::new_from_bytes(b"Central African Rep"),
            49 => ManagedBuffer::new_from_bytes(b"Chad"),
            5 => ManagedBuffer::new_from_bytes(b"Algeria"),
            50 => ManagedBuffer::new_from_bytes(b"Chile"),
            51 => ManagedBuffer::new_from_bytes(b"China"),
            52 => ManagedBuffer::new_from_bytes(b"Clipperton I"),
            53 => ManagedBuffer::new_from_bytes(b"Colombia"),
            54 => ManagedBuffer::new_from_bytes(b"Comoros"),
            55 => ManagedBuffer::new_from_bytes(b"Congo"),
            56 => ManagedBuffer::new_from_bytes(b"Cook Is"),
            57 => ManagedBuffer::new_from_bytes(b"Coral Sea Is"),
            58 => ManagedBuffer::new_from_bytes(b"Costa Rica"),
            59 => ManagedBuffer::new_from_bytes(b"Cote dIvoire"),
            6 => ManagedBuffer::new_from_bytes(b"American Samoa"),
            60 => ManagedBuffer::new_from_bytes(b"Croatia"),
            61 => ManagedBuffer::new_from_bytes(b"Cuba"),
            62 => ManagedBuffer::new_from_bytes(b"Curacao"),
            63 => ManagedBuffer::new_from_bytes(b"Cyprus UN Buffer Zone"),
            64 => ManagedBuffer::new_from_bytes(b"Cyprus"),
            65 => ManagedBuffer::new_from_bytes(b"Czechia"),
            66 => ManagedBuffer::new_from_bytes(b"Dem Rep Congo"),
            67 => ManagedBuffer::new_from_bytes(b"Denmark"),
            68 => ManagedBuffer::new_from_bytes(b"Dhekelia"),
            69 => ManagedBuffer::new_from_bytes(b"Djibouti"),
            7 => ManagedBuffer::new_from_bytes(b"Andorra"),
            70 => ManagedBuffer::new_from_bytes(b"Dominica"),
            71 => ManagedBuffer::new_from_bytes(b"Dominican Rep"),
            72 => ManagedBuffer::new_from_bytes(b"Ecuador"),
            73 => ManagedBuffer::new_from_bytes(b"Egypt"),
            74 => ManagedBuffer::new_from_bytes(b"El Salvador"),
            75 => ManagedBuffer::new_from_bytes(b"Eq Guinea"),
            76 => ManagedBuffer::new_from_bytes(b"Eritrea"),
            77 => ManagedBuffer::new_from_bytes(b"Estonia"),
            78 => ManagedBuffer::new_from_bytes(b"Ethiopia"),
            79 => ManagedBuffer::new_from_bytes(b"Faeroe Is"),
            8 => ManagedBuffer::new_from_bytes(b"Angola"),
            80 => ManagedBuffer::new_from_bytes(b"Falkland Is"),
            81 => ManagedBuffer::new_from_bytes(b"Fiji"),
            82 => ManagedBuffer::new_from_bytes(b"Finland"),
            83 => ManagedBuffer::new_from_bytes(b"Fr Polynesia"),
            84 => ManagedBuffer::new_from_bytes(b"Fr S Antarctic Lands"),
            85 => ManagedBuffer::new_from_bytes(b"France"),
            86 => ManagedBuffer::new_from_bytes(b"Gabon"),
            87 => ManagedBuffer::new_from_bytes(b"Gambia"),
            88 => ManagedBuffer::new_from_bytes(b"Georgia"),
            89 => ManagedBuffer::new_from_bytes(b"Germany"),
            9 => ManagedBuffer::new_from_bytes(b"Anguilla"),
            90 => ManagedBuffer::new_from_bytes(b"Ghana"),
            91 => ManagedBuffer::new_from_bytes(b"Gibraltar"),
            92 => ManagedBuffer::new_from_bytes(b"Greece"),
            93 => ManagedBuffer::new_from_bytes(b"Greenland"),
            94 => ManagedBuffer::new_from_bytes(b"Grenada"),
            95 => ManagedBuffer::new_from_bytes(b"Guam"),
            96 => ManagedBuffer::new_from_bytes(b"Guatemala"),
            97 => ManagedBuffer::new_from_bytes(b"Guernsey"),
            98 => ManagedBuffer::new_from_bytes(b"Guinea"),
            99 => ManagedBuffer::new_from_bytes(b"GuineaBissau"),
            _ => ManagedBuffer::new_from_bytes(b"Unknown"),
            };
        
        name
    }
}