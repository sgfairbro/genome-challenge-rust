mod read;
use std::collections::HashMap;
use crate::read::Read;
use petgraph::graph::Graph; 
use rand::Rng;

pub fn build_overlap_graph(
	read_vec : &Vec<Read> ) -> Graph::<Read, u32>{
	if read_vec.is_empty() {
		println!("Error, can't build graph because read_vec is empty"); 
	}
	let num_reads = read_vec.len();

	let mut overlap_graph = Graph::<Read, u32>::new(); 

	let mut read_to_node_hash = HashMap::new();
	// Add all reads to graph
	for r in read_vec {
		let node = overlap_graph.add_node(Read::clone(r)); 
		read_to_node_hash.insert(r, node);
	}

	for (pos, r) in read_vec.iter().enumerate() {
		for other_pos in ( pos + 1 )..num_reads {
			let other_read = &read_vec[other_pos];
			let overlap_len: u32 = r.get_overlap(other_read); 

			let this_node = read_to_node_hash[r]; 
			let other_node = read_to_node_hash[other_read];
			overlap_graph.add_edge(this_node, other_node, overlap_len); 
		}
	}
	overlap_graph
}


// splits a genome into a vector of string reads. String length 
// is determined by fraction input. num_reads specifies how many
// reads to return
pub fn split_genome<'a>( 
	genome_str : &'a str,
	fraction : f32,
	num_reads : u32 )  -> Vec<Read> {

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
			let mut read_str_1 = genome_str[idx..].to_string();
			let read_str_2 = &genome_str[..end_idx]; 
			read_str_1.push_str( read_str_2 ); 

			let genome_read = Read {
				read_str : read_str_1
			};
			reads_vec.push( genome_read );
		}
		// easier case. Just slice genome
		else {
			let genome_read = Read {
				read_str : genome_str[idx..end_idx].to_string(),
			};
			reads_vec.push( genome_read );
		}
	} 

	return reads_vec; 
}