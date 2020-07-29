use std::fmt; 

// A read contains a portion of a genome string.
// It can wrap around from the end of the genome to 
// the beginning
pub struct Read<'a> {
	pub genome_slice_1: &'a str,
	pub genome_slice_2: Option<&'a str>,
	pub b_has_two_slices : bool
}

impl fmt::Display for Read<'_>{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.b_has_two_slices {
			write!(f, "{}{:?}", self.genome_slice_1, self.genome_slice_2)			
		}
		else {
			write!(f, "{}", self.genome_slice_1)
}
}
}