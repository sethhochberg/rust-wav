extern mod std;
use std::path;
use std::io;

struct Header {
//RIFF Header
  chunk_id: @str,
  chunk_size: int,
  format: @str,

//WAVE Format Subchunk
  subchunk1_id: @str,
  subchunk1_size: int,
  audio_format: int,
  num_channels: int,
  sample_rate: int,
  byte_rate: int,
  block_align: int,
  bits_per_sample: int,

//Data Subchunk
  subchunk2_id: @str,
  subchunk2_size: int
}

fn load(filename: ~str) -> ~[~str] {
   let read_result: Result<@Reader, ~str>;
   read_result = io::file_reader(~path::Path(filename));

  if read_result.is_ok() {
    let file = read_result.unwrap();
    return file.read_lines();
  }

  println(fmt!("Error reading file: %?", read_result.unwrap_err()));
  return ~[];
}

fn main() {
  //do nothing, placeholder to use the compiler for syntax checking
}
