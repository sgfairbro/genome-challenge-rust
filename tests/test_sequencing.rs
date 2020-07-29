use rand::Rng;

// Contains possible genome chars - A, C, G, T
struct PossibleReadChars {
	pub char_opts : [char; 4],
}

impl Default for PossibleReadChars{
	fn default() -> PossibleReadChars {
		PossibleReadChars {
			char_opts: ['A', 'C', 'G', 'T'],
		}
	}
}
// Returns a random char from {A, C, G, T}
fn get_random_read_char() -> char {
	let options : PossibleReadChars = Default::default(); 
	// Generate a random index between 0-4
	let mut rng = rand::thread_rng(); 
	let idx = rng.gen_range(0, 4); 
	return options.char_opts[idx];
}

// Generates a genome string of the specified length
fn generate_genome(genome_length: i32) -> String{
	let mut genome = String::new();
	for _ in 0..genome_length {
		let read_char = get_random_read_char(); 
		genome.push( read_char )
	}
	return genome; 
}

#[cfg(test)]
mod tests {

	use super::*;
	use genome::split_genome;

	#[test]
	fn test_generate_genome(){
		let req_length = 10; 
		let genome_str = generate_genome(req_length);
		println!("genome str: {}", genome_str);
		assert_eq!(genome_str.chars().count(), 10);

		let slices_vec = genome::split_genome( &genome_str, 0.3, 4);
		for slice_it in slices_vec.iter(){
			println!("genome slice: {}", slice_it ); 
		}
	}

    #[test]
	fn test_graph(){
		assert_eq!(1,1);
	}
}