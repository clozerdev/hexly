#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub uid: Option<String>,
    pub atr: Option<String>,
    pub card_type: Option<CardType>,
    pub capacity_bytes: Option<u32>,
    pub card_data: Option<CardData>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardType {
    MifareClassic1K,
    MifareClassic4K,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardData {
    pub total_sectors: u32,
    pub total_blocks: u32,
    pub readable_sectors: u32,
    pub protected_sectors: u32,
    pub key_a_known: u32,
    pub key_b_known: u32,
    pub sectors: Vec<SectorDump>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectorDump {
    pub sector_index: u32,
    pub first_block_index: u32,
    pub authenticated_with: Option<KeyType>,
    pub blocks: Vec<BlockDump>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockDump {
    pub block_index: u32,
    pub kind: BlockKind,
    pub data: Option<[u8; 16]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockKind {
    Data,
    Trailer,
    Manufacturer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyType {
    A,
    B,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reader {
    pub id: ReaderId,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReaderId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReaderStatus {
    Ready,
    Working,
    Disconnected,
}
