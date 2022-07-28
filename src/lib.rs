use std::io;
use std::{collections::HashMap, io::BufReader};

pub use rscache;
use rscache::definition::osrs::Definition;
use rscache::extension::ReadExt;
use rscache::Cache;

macro_rules! impl_osrs_loader {
    ($ldr:ident, $def:ty, index_id: $idx_id:expr $(, archive_id: $arc_id:expr)?) => {
        impl $ldr {
            #[allow(unreachable_code)]
            pub fn new(cache: &Cache) -> crate::Result<Self> {
                $(
                    let map = <$def>::fetch_from_archive(cache, $idx_id, $arc_id)?;

                    return Ok(Self(map));
                )?

                let map = <$def>::fetch_from_index(cache, $idx_id)?;

                Ok(Self(map))
            }

            pub fn load(&self, id: u16) -> Option<&$def> {
                self.0.get(&id)
            }
        }

        impl_iter_for_loader!($ldr, u16, $def);
    };
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct InventoryDefinition {
    pub id: u16,
    pub capacity: Option<u16>,
}

/*impl Definition for InventoryDefinition {
    fn new(id: u16, buffer: &[u8]) -> crate::Result<Self> {
        let mut reader = BufReader::new(buffer);
        let item_def = decode_buffer(id, &mut reader)?;

        Ok(item_def)
    }
}*/

fn decode_buffer(id: u16, reader: &mut BufReader<&[u8]>) -> io::Result<InventoryDefinition> {
    let mut inv_def = InventoryDefinition { id, capacity: None };

    loop {
        let opcode = reader.read_u8()?;
        match opcode {
            0 => break,
            2 => inv_def.capacity = reader.read_u16().ok(),
            _ => {}
        }
    }

    Ok(inv_def)
}

/// Loads all inventory definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct InventoryLoader(HashMap<u16, InventoryDefinition>);

impl_osrs_loader!(InventoryLoader, InventoryDefinition, index_id: 2, archive_id: 5);
