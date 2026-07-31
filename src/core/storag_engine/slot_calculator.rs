use alloy::{primitives::{Address, U256, keccak256,B256}, sol_types::SolValue};



pub fn calculate_mapping_slot(key:Address,base_slot:U256)->U256{
  let encoded = (key,base_slot).abi_encode();
  keccak256(encoded).into()
}


pub fn calculate_nested_mapping_slot(keys:&[Address],base_slot:U256)->B256{
  let mut final_slot=base_slot;
  for key in keys {
    final_slot=calculate_mapping_slot(*key,final_slot);
    
  }
  B256::from(final_slot)
}




fn calculate_slot_and_offset(
    base_slot: U256,
    index: U256,
    element_size: u8,
) -> (U256, u8) {
    let elements_per_slot = U256::from(256 / u16::from(element_size));

    let slot_offset = index / elements_per_slot;
    let bit_offset = (index % elements_per_slot) * U256::from(element_size);

    // Direct conversion: keccak256 returns B256, converted directly to U256 via .into()
    let base_hash: U256 = keccak256(base_slot.abi_encode()).into();
    let final_slot = base_hash + slot_offset;

  
    let offset_u8 = u8::try_from(bit_offset)
        .expect("Invalid: bit offset out of bounds");

    (final_slot, offset_u8)
}