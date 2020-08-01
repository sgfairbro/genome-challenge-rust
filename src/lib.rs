mod read;
use crate::read::Read;
use rand::Rng;

// splits a genome into a vector of string reads. String length 
// is determined by fraction input. num_reads specifies how many
// reads to return
pub fn split_genome<'a>( 
	genome_str : &'a str,
	fraction : f32,
	num_reads : i32 )  -> Vec<Read> {

	let mut reads_vec : Vec<Read> = vec![]; 

	if  genome_str.is_empty() {
		println!("Error. Got empty string");
		return reads_vec; 
	}

	let genome_num_chars = genome_str.chars().count();

	// Calculate how many characters will be in each read
	let num_chars_per_read =  ( fraction * genome_num_chars as f32 ).round() as usize ; 

	if num_chars_per_read == 0 || num_chars_per_read > genome_num_chars {
		println!("Error. Invalid number of characters per read, {}", num_chars_per_read );
		return reads_vec;
	}

	let mut increment = num_chars_per_read as usize; 
	while  genome_num_chars % increment == 0  {
		increment += 1;
	}

	// Start at a random character in the genome
	for _ in 0..num_reads {
		let mut rng = rand::thread_rng();
	    let idx = rng.gen_range(0, genome_num_chars);
		let end_idx =  ( idx + num_chars_per_read ) % genome_num_chars;
		// Create two slices if we need to wrap around genome string
		if end_idx < idx {
			let genome_read = Read {
				genome_slice_1 : &genome_str[idx..],
				genome_slice_2 : Some(&genome_str[..end_idx]),
				b_has_two_slices : true
			};
			reads_vec.push( genome_read );

		}
		// easier case. Just slice genome
		else {
			let genome_read = Read {
				genome_slice_1 : &genome_str[idx..end_idx],
				genome_slice_2 : None, 
				b_has_two_slices : false
			};
			reads_vec.push( genome_read );
		}
	} 

	return reads_vec; 
}