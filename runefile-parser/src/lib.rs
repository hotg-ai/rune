
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod parser;

#[cfg(test)]
mod tests {

    use crate::parser::*;
    #[test]
    fn read_rune_microspeech_file() {
        let fileloc = "examples/microspeech/Runefile".to_string();
        let contents = std::fs::read_to_string(fileloc)
        .expect("Failed to load file");
        generate(contents);
    }

    use std::collections::HashMap;
    use runic_pb_fft::Processor;
    use runic_types::proc_block::ProcBlock;
    
    #[test]
    fn test_ftt() {
        let mut map = HashMap::new();
        map.insert(String::from("hz"), String::from("16000"));

        let waveform: Vec<i16> = vec![0; 16000];
        let result=fft(waveform);
        assert_eq!(result.len(), 1960);
        //println!("ftt output: {:?}",result)
    }


    //generated code by proc_block
    pub fn fft(waveform: Vec<i16>) -> Vec<u8> {
        let mut map = HashMap::new();
        map.insert(String::from("hz"), String::from("16000"));
        let fft = Processor{};
        return fft.process(waveform, map);
    }
    

}

