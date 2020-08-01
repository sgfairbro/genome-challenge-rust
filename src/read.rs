use std::fmt; 

// A read contains a portion of a genome string.
// It can wrap around from the end of the genome to 
// the beginning
pub struct Read<'a> {
	pub genome_slice_1: &'a str,
	pub genome_slice_2: Option<&'a str>,
	pub b_has_two_slices : bool
}

impl Read<'_> {
	pub fn len(&self) -> u32 {
		if self.b_has_two_slices {
			let num_chars = self.genome_slice_1.chars().count() + 
				self.genome_slice_2.as_deref().unwrap_or("default string").chars().count(); 
			return num_chars as u32;

		}
		else {
			return self.genome_slice_1.chars().count() as u32;
		}
	}
}

impl fmt::Display for Read<'_>{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.b_has_two_slices {
			write!(f, "{}{}", self.genome_slice_1, self.genome_slice_2.as_deref().unwrap_or("default string") )			
		}
		else {
			write!(f, "{}", self.genome_slice_1)
}
}
}