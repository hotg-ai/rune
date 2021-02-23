#![no_std]
pub use runic_types::{Transform, PipelineContext};

pub struct Normalize {

}

impl<const N: usize> Transform<[f32; N]> for Normalize {
    type Output = [f32; N];

    fn transform(&mut self,
        mut input: [f32; N],
        ctx: &mut PipelineContext) -> Self::Output {

        // let input_size = input.len();
        let mut output: [f32; 384]= [0.0; 384];
        
        let min_value = input.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_value = input.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let denom = max_value-min_value;
        if denom == 0.0 {
          return input;
        }
      
        for element in &mut input {
            
            *element = (*element - min_value) / denom;
        }
      
        return input
      }
    
}


#[cfg(test)]
mod tests {
    use runic_types::{Transform, PipelineContext};
    use crate::Normalize;

    use rand::{thread_rng, Rng};
    #[test]
    fn it_works() {
        let mut input: [f32; 348] = [0.0;348];
        thread_rng().fill(&mut input[..]);
        let mut norm_pb: Normalize = Normalize{}; 
        let mut pipeline = PipelineContext{};
        let output = norm_pb.transform(input, &mut pipeline);
        let min_value = output.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_value = output.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        assert_eq!(min_value, 0.0);
        assert_eq!(max_value, 1.0);
        assert_eq!(output.len(), 348);
    }

    #[test]
    fn handle_empty() {
        let input: [f32; 384] = [0.0;384];
       
        let mut norm_pb: Normalize = Normalize{}; 
        let mut pipeline = PipelineContext{};
        let output = norm_pb.transform(input, &mut pipeline);
      
        
        assert_eq!(output, input);
        assert_eq!(output.len(), 384);
    }
}
