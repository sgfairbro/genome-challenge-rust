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
fn generate_genome(genome_length: u32) -> String{
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
	
	#[test]
	fn test_generate_short_genome(){
		let req_length = 10; 
		let genome_str = generate_genome(req_length);
		println!("genome str: {}", genome_str);
		assert_eq!(genome_str.chars().count(), 10);

		let slices_vec = genome::split_genome( &genome_str, 0.3, 4);
		assert_eq!(slices_vec.len(), 4);
		for slice_it in slices_vec.iter(){
			println!("genome slice: {}", slice_it ); 
			assert_eq!( slice_it.len(), 3);
		}
	}

	#[test]
	fn test_generate_random_genome(){
		let mut rng = rand::thread_rng();
	    let genome_length = rng.gen_range(1, 100);
		let genome_str = generate_genome(genome_length);
		println!("genome str: {}", genome_str);
		println!("genome length: {}", genome_length);
		assert_eq!(genome_str.chars().count() as u32, genome_length);

		let num_reads = ( genome_length / 4 ) as u32; 
		println!("num reads: {}", num_reads); 

		let fraction = rng.gen_range( 0.0, 1.0 ); 
		println!("fraction: {}", fraction); 

		let slices_vec = genome::split_genome( &genome_str, fraction, num_reads);
		assert_eq!(slices_vec.len() as u32, num_reads);
		for slice_it in slices_vec.iter(){
			println!("genome slice: {}", slice_it ); 
		}
	}

    #[test]
	fn test_graph(){
		assert_eq!(1,1);
	}
}