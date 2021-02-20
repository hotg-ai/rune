use runic_types::{Transform, PipelineContext};

struct Normalize {

}

impl Transform<Vec<f32>> for Normalize {
    type Output = Vec<f32>;

    fn transform(&mut self,
        input: Vec<f32>,
        ctx: &mut PipelineContext) -> Self::Output {

        // let input_size = input.len();
        let mut output: [f32; 384]= [0.0; 384];
        
        let min_value = input.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_value = input.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let denom = max_value-min_value;
        if denom == 0.0 {
          return input;
        }
      
        for i in 0..input.len() {
            
            output[i] = (input[i]-min_value)/denom as f32;
        }
      
        return output.to_vec();
      }
    
}


#[cfg(test)]
mod tests {
    use runic_types::{Transform, PipelineContext};
    use crate::Normalize;

    use rand::{thread_rng, Rng};
    #[test]
    fn it_works() {
        let mut input: [f32; 384] = [0.0;384];
        thread_rng().fill(&mut input[..]);
        let mut norm_pb: Normalize = Normalize{}; 
        let mut pipeline = PipelineContext{};
        let output = norm_pb.transform(input.to_vec(), &mut pipeline);
      
        //println!(" {:?}", output);
        assert_eq!(output.len(), 384);
    }

    #[test]
    fn handle_empty() {
        let input: [f32; 384] = [0.0;384];
       
        let mut norm_pb: Normalize = Normalize{}; 
        let mut pipeline = PipelineContext{};
        let output = norm_pb.transform(input.to_vec(), &mut pipeline);
      
        
        assert_eq!(output, input);
        assert_eq!(output.len(), 384);
    }
}
