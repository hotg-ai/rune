use runic_types::{Transform, PipelineContext};

struct OhvLabel {
    labels: Vec<&'static str>
}

impl Transform<Vec<u8>> for OhvLabel {
    type Output = &'static str;

    

    fn transform(&mut self,
        input: Vec<u8>,
        _ctx: &mut PipelineContext) -> Self::Output {

        let index: i8 = match input.iter().position(|&r| r == 1) {
            Some(i) => i as i8,
            None => -1
        };

        if index != -1 {
           let label = match self.labels.get(index as usize) {
               Some(l) => l,
               None => "NO_LABEL_FOUND"
           };

           return label;
        } else {
            return "OHV_NULL";
        }
        
      }
    
}


#[cfg(test)]
mod tests {
    use rand::Rng;
    use rand::thread_rng;

    use runic_types::{Transform, PipelineContext};
    
    use crate::OhvLabel;
    
    #[test]
    fn it_works() {
        let mut input: [u8; 4] = [0;4];
        let mut rng = thread_rng();
        let idx = rng.gen_range(0..4) as usize;
        let mut pipeline = PipelineContext{};
        input[idx] = 1;
        
        let mut pb = OhvLabel{
            labels: vec!["Wing", "Ring", "Slope", "Unknown"]
        };

        let out = pb.transform(input.to_vec(), &mut pipeline);
        let labels = vec!["Wing", "Ring", "Slope", "Unknown"];
        assert_eq!(out, labels[idx]);
        println!("OhV={:?} | Label={}", input, out);
    }

    #[test]
    fn handles_missing_labels() {
        let mut input: [u8; 4] = [0;4];
        let mut rng = thread_rng();
        let idx = rng.gen_range(0..4) as usize;
        let mut pipeline = PipelineContext{};
        input[idx] = 1;
        
        let mut pb = OhvLabel{
            labels: vec![]
        };

        let out = pb.transform(input.to_vec(), &mut pipeline);
        
        assert_eq!(out, "NO_LABEL_FOUND");
        println!("OhV={:?} | Label={}", input, out);
    }

    #[test]
    fn handles_null_ohv() {
        let mut input: [u8; 4] = [0;4];

        
        let mut pb = OhvLabel{
            labels: vec!["a", "b", "c", "d"]
        };
        let mut pipeline = PipelineContext{};
        
        let out = pb.transform(input.to_vec(), &mut pipeline);
        
        assert_eq!(out, "OHV_NULL");
        println!("OhV={:?} | Label={}", input, out);
    }


}
