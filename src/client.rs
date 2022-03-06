#![allow(unused)]

use regex::Regex;

use crate::algorithm;

//refresh interval
const NEVER: u8 = 0;
const TIMED_OR_AFTER_STARTED_ANNOUNCE: u8 = 1;
const TORRENT_VOLATILE: u8 = 2;
const TORRENT_PERSISTENT: u8 = 3;

//case
const LOWER: u8 = 8;
const UPPER: u8 = 9;

//algorithms for ket and peer generator
const REGEX: u8 = 10;
const HASH: u8 = 11;
const HASH_NO_LEADING_ZERO: u8 = 12;
const DIGIT_RANGE_TRANSFORMED_TO_HEX_WITHOUT_LEADING_ZEROES: u8 = 13; //inclusive bounds: from 1 to 2147483647
const RANDOM_POOL_WITH_CHECKSUM: u8 = 14;
const PEER_ID_LENGTH: usize = 20;

#[derive(Default, Clone)]
pub struct Client {
    //----------- algorithms
    ///key algorithm
    key_algorithm: u8, //length=8
    ///for REGEX method, for RANDOM_POOL_WITH_CHECKSUM: list pf available chars, the base is the length of the string
    key_pattern: String,
    /// for RANDOM_POOL_WITH_CHECKSUM
    prefix: String,
    key_refresh_on: u8,
    key_refresh_every: u16,
    key_uppercase: Option<bool>,

    //----------- peer ID
    peer_algorithm: u8,
    peer_pattern: String,
    peer_refresh_on: u8,
    peer_prefix:String,

    //----------- URL encoder 
    encoding_exclusion_pattern: String,
    /// if the encoded hex string should be in upper case or no
    uppercase_encoded_hex: bool,
    should_url_encode: bool,

    query: String,
    //request_headers: HashMap<String, String>, //HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
    user_agent: String,
    accept:String,
    accept_encoding: String,
    accept_language: String,
    connection:Option<String>,
    num_want: u16,
    num_want_on_stop: u16,

    //generated values
    infohash :String,
    peer_id: String,
}

impl Client {
    fn default() -> Self {
        Client {
            //key generator default values
            key_algorithm: HASH,
            key_pattern:String::new(), prefix:String::new(),
            key_uppercase: None,
            key_refresh_on: TIMED_OR_AFTER_STARTED_ANNOUNCE,
            key_refresh_every: 0,
            //peer ID generator
            peer_algorithm: HASH,
            peer_pattern: String::new(),
            peer_prefix:String::new(),
            peer_refresh_on: NEVER,
            //URL encoder
            encoding_exclusion_pattern: r"[A-Za-z0-9-]".to_owned(),
            uppercase_encoded_hex: false,
            should_url_encode: false,
            //misc
            num_want: 200,
            num_want_on_stop: 0,
            //query headers
            query: "info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1".to_owned(),
            user_agent: String::new(), //must be defined
            accept: String::new(),
            accept_encoding: String::from("gzip"),
            accept_language: String::new(),
            connection: Some(String::from("Close")),
            infohash: String::new(),
            peer_id: String::new(),
        }
    }
    /*pub fn get_query(self: &Self, infohash: &str) -> &str {
        let mut q = self.query.clone();
        //format!(String::new(self.query), infohash=infohash, peerid=self.peer_id, uploaded=0, downloaded=0, left=0, corrupt=0, key=self.key, numwant=self.numwant);
        //format!(String::from_str(&self.query), infohash=infohash, peerid=self.peer_id, uploaded=0, downloaded=0, left=0, corrupt=0, key=self.key, numwant=self.numwant);
        return &q;
    }*/
    fn build(mut self: &Self) {
        //generate PEER_ID
        if self.peer_algorithm == REGEX {/*self.peer_id=*/algorithm::regex(self.peer_pattern);}
        else {algorithm::random_pool_with_checksum(PEER_ID_LENGTH, self.peer_prefix, self.peer_pattern);}
        //generate KEY
        if self.key_algorithm == HASH {algorithm::hash(8, false, self.key_uppercase);}
        else if self.key_algorithm == HASH_NO_LEADING_ZERO {algorithm::hash(8, true, self.key_uppercase);}
        else if self.key_algorithm == DIGIT_RANGE_TRANSFORMED_TO_HEX_WITHOUT_LEADING_ZEROES {algorithm::digit_range_transformed_to_hex_without_leading_zero();}
    }
}

/// URL encode a string. It does NOT change the casing of the regular characters, but it lower all encoded characters.
///
/// # Arguments
///
/// * `pattern` - regular expression pattern reprented by string slice
/// * `data` - data to process
/// * `uppercase` - if the output should be in upper case
fn get_URL_encoded_char<'a>(pattern: &str, c: char, uppercase: bool) -> String {
    let mut hex=String::new();
    if !pattern.is_empty() && Regex::new(pattern).unwrap().is_match(&String::from(c)) {return String::from(c);}
    if c==0 as char {hex.push_str("%00")}
    else {hex.push_str(&format!("%{:02x}", c as u8));}
    if uppercase {return hex.to_uppercase();} else {return hex;}
}

pub fn get_URL_encoded(pattern: &str, data: &str, uppercase: bool) -> String {
    let mut r = String::with_capacity(128);
    for c in data.chars() {r.push_str(&get_URL_encoded_char(pattern, c, uppercase));}
    return r;
}

/// Get the client from the name provided in parameter. If you had/remove client, edit static/index.html to modify the select in the HTML.
pub fn get_client(name : &str) -> Option<Client> {
    let mut c:Option<Client>=None; 
    //bittorrent
    if name == "bittorrent-7.10.1_43917" {c=Some(Client {should_url_encode:true, peer_pattern:"-BT71000(\u{008d}\u{00ab})[\u{0001}-\u{00ff}]{10}", user_agent:"BitTorrent/7100(255961997)(43917)", ..Client::default()});}
    else if name == "bittorrent-7.10.3_44359" {c=Some(Client {should_url_encode:true, peer_pattern:"-BT7a3S-G(\u{00ad})[\u{0001}-\u{00ff}]{10}", user_agent:"BitTorrent/7103(256355655)(44359)", ..Client::default()});}
    else if name == "bittorrent-7.10.3_44429" {c=Some(Client {should_url_encode:true, peer_pattern:"-BT7a3S-(\u{008d})(\u{00ad})[\u{0001}-\u{00ff}]{10}", user_agent:"BitTorrent/7103(256355725)(44429)", ..Client::default()});}
    
    //deluge
    else if name == "deluge-1.3.13" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on:TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-DE13D0-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]",user_agent:"Deluge 1.3.13", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), ..Client::default()});}
    else if name == "deluge-1.3.14" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on:TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-DE13E0-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"Deluge 1.3.14", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), ..Client::default()});}
    else if name == "deluge-1.3.15" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on:TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-DE13F0-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"Deluge 1.3.15", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), ..Client::default()});}
    
    //leap
    else if name == "leap-2.6.0.1" {c=Some(Client {key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-LT1100-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"libtorrent_leap/1.1.1.0", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    
    //qbittorrent
    else if name == "qBittorrent-3.3.1" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB3310-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent v3.3.1", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-3.3.7" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB3310-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent v3.3.7", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-3.3.13" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB33D0-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent/3.3.13", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-3.3.14" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB33E0-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent/3.3.14", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-3.3.15" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB33F0-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent/3.3.15", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-3.3.16" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB33G0-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent/3.3.16", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.0.0" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4000-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent /4.0.0", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.0.1" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4010-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent /4.0.1", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.0.2" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4020-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent /4.0.2", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.0.3" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4030-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent /4.0.3", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.0.4" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4040-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent /4.0.4", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.1.0" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4100-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent /4.1.0", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.1.1" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4110-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent /4.1.1", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.1.2" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4120-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent /4.1.2", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.1.3" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4130-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent /4.1.3", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.1.4" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4140-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent /4.1.4", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.1.5" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4150-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent /4.1.5", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.1.6" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4160-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent /4.1.6", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.1.7" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4170-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent /4.1.7", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.1.8" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4180-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent /4.1.8", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.1.9" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4190-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent /4.1.9", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.2.0" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4200-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent/4.2.0", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.2.1" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4210-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent/4.2.1", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.2.2" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4220-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent/4.2.2", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.2.3" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4230-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent/4.2.3", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.2.4" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4240-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent/4.2.4", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.2.5" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"-qB4250-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent/4.2.5", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.3.0" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"4.3-qB4300-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent/4.3.0", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.3.0.1" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"4.3-qB4301-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent/4.3.0.1", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.3.1" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"4.3-qB4310-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent/4.3.1", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.3.2" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"4.3-qB4320-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent/4.3.2", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}
    else if name == "qBittorrent-4.3.3" {c=Some(Client {key_algorithm:HASH_NO_LEADING_ZERO, key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(true), peer_pattern:"4.3-qB4330-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}", encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"qBittorrent/4.3.3", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0".to_owned(), connection:Some("close"), ..Client::default()});}

    //rtorrent
    else if name == "rtorrent-0.9.6_0.13.6" {c=Some(Client {key_refresh_on: TORRENT_PERSISTENT, key_uppercase:Some(false), peer_algorithm:REGEX, peer_pattern:"-lt0D60-[\u{0001}-\u{00ff}]{12}", should_url_encode:true, peer_refresh_on:TORRENT_PERSISTENT, uppercase_encoded_hex:true, user_agent:"rtorrent/0.9.6/0.13.6", accept:"*/*", accept_encoding:"deflate, gzip", query:"info_hash={infohash}&peer_id={peerid}&key={key}&compact=1&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&event={event}".to_owned(), num_want:50, connection:None, ..Client::default()});}
    
    //transmission
    else if name == "transmission-2.82_14160" {c=Some(Client {key_algorithm:DIGIT_RANGE_TRANSFORMED_TO_HEX_WITHOUT_LEADING_ZEROES, key_pattern:"1-2147483647", key_refresh_on: NEVER, key_uppercase:Some(false), peer_prefix:"-TR2820-", peer_pattern:"0123456789abcdefghijklmnopqrstuvwxyz", peer_refresh_on:TORRENT_VOLATILE, encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"Transmission/2.82", accept:"*/*", accept_encoding:"gzip;q=1.0, deflate, identity", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&numwant={numwant}&key={key}&compact=1&supportcrypto=1&event={event}&ipv6={ipv6}".to_owned(), connection:None, num_want:80, ..Client::default()});}
    else if name == "transmission-2.92_14714" {c=Some(Client {key_algorithm:DIGIT_RANGE_TRANSFORMED_TO_HEX_WITHOUT_LEADING_ZEROES, key_pattern:"1-2147483647", key_refresh_on: NEVER, key_uppercase:Some(false), peer_prefix:"-TR2824Z-", peer_pattern:"0123456789abcdefghijklmnopqrstuvwxyz", peer_refresh_on:TORRENT_VOLATILE, encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"Transmission/2.84+", accept:"*/*", accept_encoding:"gzip;q=1.0, deflate, identity", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&numwant={numwant}&key={key}&compact=1&supportcrypto=1&event={event}&ipv6={ipv6}".to_owned(), connection:None, num_want:80, ..Client::default()});}
    else if name == "transmission-2.93" {c=Some(Client {key_algorithm:DIGIT_RANGE_TRANSFORMED_TO_HEX_WITHOUT_LEADING_ZEROES, key_pattern:"1-2147483647", key_refresh_on: NEVER, key_uppercase:Some(false), peer_prefix:"-TR2930-", peer_pattern:"0123456789abcdefghijklmnopqrstuvwxyz", peer_refresh_on:TORRENT_VOLATILE, encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"Transmission/2.93", accept:"*/*", accept_encoding:"gzip;q=1.0, deflate, identity", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&numwant={numwant}&key={key}&compact=1&supportcrypto=1&event={event}&ipv6={ipv6}".to_owned(), connection:None, num_want:80, ..Client::default()});}
    else if name == "transmission-2.94" {c=Some(Client {key_algorithm:DIGIT_RANGE_TRANSFORMED_TO_HEX_WITHOUT_LEADING_ZEROES, key_pattern:"1-2147483647", key_refresh_on: NEVER, key_uppercase:Some(false), peer_prefix:"-TR2940-", peer_pattern:"0123456789abcdefghijklmnopqrstuvwxyz", peer_refresh_on:TORRENT_VOLATILE, encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"Transmission/2.94", accept:"*/*", accept_encoding:"gzip;q=1.0, deflate, identity", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&numwant={numwant}&key={key}&compact=1&supportcrypto=1&event={event}&ipv6={ipv6}".to_owned(), connection:None, num_want:80, ..Client::default()});}
    else if name == "transmission-3.00" {c=Some(Client {key_algorithm:DIGIT_RANGE_TRANSFORMED_TO_HEX_WITHOUT_LEADING_ZEROES, key_pattern:"1-2147483647", key_refresh_on: NEVER, key_uppercase:Some(false), peer_prefix:"-TR3000-", peer_pattern:"0123456789abcdefghijklmnopqrstuvwxyz", peer_refresh_on:TORRENT_VOLATILE, encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", user_agent:"Transmission/3.00", accept:"*/*", accept_encoding:"gzip;q=1.0, deflate, identity", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&numwant={numwant}&key={key}&compact=1&supportcrypto=1&event={event}&ipv6={ipv6}".to_owned(), connection:None, num_want:80, ..Client::default()});}

    //utorrent
    else if name == "utorrent-3.2.2_28500" {c=Some(Client {key_refresh_on: TIMED_OR_AFTER_STARTED_ANNOUNCE, key_refresh_every: 10, key_uppercase:Some(true), peer_pattern:"-UT3220-To[\u{0001}-\u{00ff}]{10}", should_url_encode:true, user_agent:"uTorrent/3220(28500)", accept_encoding:"gzip;q=1.0, deflate, identity", accept_language:"{locale}", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&ipv6={ipv6}".to_owned(), num_want_on_stop:200, ..Client::default()});}
    else if name == "utorrent-3.5.0_43916" {c=Some(Client {key_refresh_on: TIMED_OR_AFTER_STARTED_ANNOUNCE, key_refresh_every: 10, key_uppercase:Some(true), peer_pattern:"-UT3500-(\u{008c}\u{00ab})[\u{0001}-\u{00ff}]{10}", should_url_encode:true, user_agent:"uTorrent/350(111258508)(43916)", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1".to_owned(), ..Client::default()});}
    else if name == "utorrent-3.5.0_44090" {c=Some(Client {key_refresh_on: TIMED_OR_AFTER_STARTED_ANNOUNCE, key_refresh_every: 10, key_uppercase:Some(true), peer_pattern:"-UT3500-(\u{003a}\u{00ac})[\u{0001}-\u{00ff}]{10}", should_url_encode:true, user_agent:"uTorrent/350(111258682)(44090)", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1".to_owned(), ..Client::default()});}
    else if name == "utorrent-3.5.0_44294" {c=Some(Client {key_refresh_on: TIMED_OR_AFTER_STARTED_ANNOUNCE, key_refresh_every: 10, key_uppercase:Some(true), peer_pattern:"-UT3500-(\u{0006}\u{00ad})[\u{0001}-\u{00ff}]{10}", should_url_encode:true, user_agent:"uTorrent/350(111258886)(44294)", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1".to_owned(), ..Client::default()});}
    else if name == "utorrent-3.5.1_44332" {c=Some(Client {key_refresh_on: TIMED_OR_AFTER_STARTED_ANNOUNCE, key_refresh_every: 10, key_uppercase:Some(true), peer_pattern:"-UT3515-(\u{002c}\u{00ad})[\u{0001}-\u{00ff}]{10}", should_url_encode:true, user_agent:"uTorrent/351(111389996)(44332)", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1".to_owned(), ..Client::default()});}
    else if name == "utorrent-3.5.3_44358" {c=Some(Client {key_refresh_on: TIMED_OR_AFTER_STARTED_ANNOUNCE, key_refresh_every: 10, key_uppercase:Some(true), peer_pattern:"-UT353S-F(\u{002c}\u{00ad})[\u{0001}-\u{00ff}]{10}", should_url_encode:true, user_agent:"uTorrent/353(111652166)(44358)", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1".to_owned(), ..Client::default()});}
    else if name == "utorrent-3.5.3_44428" {c=Some(Client {key_refresh_on: TIMED_OR_AFTER_STARTED_ANNOUNCE, key_refresh_every: 10, key_uppercase:Some(true), peer_pattern:"-UT353S-(\u{008c}\u{00ad})[\u{0001}-\u{00ff}]{10}", should_url_encode:true, user_agent:"uTorrent/353(111652236)(44428)", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1".to_owned(), ..Client::default()});}
    else if name == "utorrent-3.5.4_44498" {c=Some(Client {key_refresh_on: TIMED_OR_AFTER_STARTED_ANNOUNCE, key_refresh_every: 10, key_uppercase:Some(true), peer_pattern:"-UT354S-(\u{00d2}\u{00ad})[\u{0001}-\u{00ff}]{10}", should_url_encode:true, user_agent:"uTorrent/353(111783378)(44498)", query:"info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1".to_owned(), ..Client::default()});}

    //vuze
    else if name == "vuze-5.7.5.0" {c=Some(Client {key_algorithm:REGEX, key_uppercase:None, key_refresh_on: TORRENT_VOLATILE, peer_pattern:"-AZ5750-[a-zA-Z0-9]{12}", peer_refresh_on:TORRENT_VOLATILE, encoding_exclusion_pattern:r"[A-Za-z0-9_~\\(\\)\\!\\.\\*-]", uppercase_encoded_hex:true, user_agent:"Azureus 5.7.5.0;{os};Java {java}", accept:"text/html, image/gif, image/jpeg, *; q=.2, */*; q=.2", query:"info_hash={infohash}&peer_id={peerid}&port={port}&azudp={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&event={event}&numwant={numwant}&no_peer_id=1&compact=1&key={key}&azver=3".to_owned(), connection:Some("close"), num_want:100, ..Client::default()});}
    //if c.is_some() {move || c.unwrap().build();}
    return c;
}

//******************************************* TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;
    #[test]
    fn should_encode_chars() {
        assert_eq!(get_URL_encoded_char("", 0x00 as char, false), "%00");
        assert_eq!(get_URL_encoded_char("", 0x01 as char, false), "%01");
        assert_eq!(get_URL_encoded_char("", 0x10 as char, false), "%10");
        assert_eq!(get_URL_encoded_char("", 0x1e as char, false), "%1e");
        assert_eq!(get_URL_encoded_char("", 0x32 as char, false), "%32");
        assert_eq!(get_URL_encoded_char("", 0x7a as char, false), "%7a");
        assert_eq!(get_URL_encoded_char("", 0xff as char, false), "%ff");
    }
    #[test]
    fn should_not_encode_if_regex_dot_star() {
        assert_eq!(get_URL_encoded_char(r".*", 0x32 as char, false), "2");
        assert_eq!(get_URL_encoded_char(r".*", 0x6e as char, false), "n");
        assert_eq!(get_URL_encoded_char(r".*", 0x7a as char, false), "z");
    }
    #[test]
    fn should_not_encode_excluded_chars() {
        assert_eq!(get_URL_encoded_char(r"[a-zA-Z0-9]", 0x00 as char, false), "%00");
        assert_eq!(get_URL_encoded_char(r"[a-zA-Z0-9]", 0x10 as char, false), "%10");
        assert_eq!(get_URL_encoded_char(r"[a-zA-Z0-9]", 0x1e as char, false), "%1e");
        assert_eq!(get_URL_encoded_char(r"[a-zA-Z0-9]", 0x32 as char, false), "2");
        assert_eq!(get_URL_encoded_char(r"[a-zA-Z0-9]", 0x7a as char, false), "z");
        assert_eq!(get_URL_encoded_char(r"[a-zA-Z0-9]", 0xff as char, false), "%ff");
    }
    #[test]
    fn should_not_encode_translate_case_if_not_encoded_char() {
        assert_eq!(get_URL_encoded_char(r"[a-zA-Z0-9]", 0x79 as char, true), "y");
        assert_eq!(get_URL_encoded_char(r"[a-zA-Z0-9]", 0x59 as char, true), "Y");
    }
    #[test]
    fn should_translate_case_if_encoded_char() {
        assert_eq!(get_URL_encoded_char(r"[a-zA-Z0-9]", 0xae as char, true), "%AE");
    }
    #[test]
    fn should_encode() {
        assert_eq!(get_URL_encoded("[a-zA-Z0-9]", &String::from_iter(vec!['a', 0x11 as char, 'q', 0xf3 as char]), false), "a%11q%f3");
        assert_eq!(get_URL_encoded("", &String::from_iter(vec![0xA2 as char, 0x11 as char, 0xf3 as char]), true), "%A2%11%F3");
    }
}