use std::fmt; 

// A read contains a portion of a genome string.
// It can wrap around from the end of the genome to 
// the beginning
pub struct Read {
	pub read_str: String
}

impl Read {
	pub fn len(&self) -> usize {
		return self.read_str.chars().count(); 
	}

	pub fn get_overlap(&self, other_read: Read) ->u32 {
		let mut overlap_len = 0 as u32; 
		let mut max_overlap_len = 0 as u32; 
		for this_c in self.read_str.chars() {
			for other_c in other_read.read_str.chars() {
				if this_c == other_c {
					overlap_len += 1; 
				}
				else {
					if overlap_len > max_overlap_len {
						max_overlap_len = overlap_len; 
					}
				}
			}
		}
		return max_overlap_len; 
	}
}



impl fmt::Display for Read{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			write!(f, "{}", self.read_str )			
}
}