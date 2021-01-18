# runefile-parser-rs
Runefile parser

# Instructions

```pub enum Instruction {
    From(instructions::from::FromInstruction),
    Model(instructions::model::ModelInstruction),
    Capability(instructions::capability::CapabilityInstruction),
    Run(instructions::run::RunInstruction),
    ProcBlock(instructions::procBlock::ProcBlockInstruction),
    Out(instructions::out::OutInstruction),
    //Misc(MiscInstruction)
}
```

## Folder creation

Let's create a container with a hash (SHA128) in `$HOME/.rune/containers/{SHA128}`.

The struct is convert to a Cargo project that uses template. For example:

## FROM

version 1 will only have one `FROM` which is the runic-os. This means that 
the Cargo.toml template will have:

```
[dependencies]
runic-types = { git = "ssh://git@github.com/hotg-ai/runic-types" }
```

Later on we will allow people to use their own images as `FROM`. 


## CAPABILITY 

```

    {% for capability in capabilities %}
        let mut params = HashMap::new();

        {%for param in capability_params %} 
            params.insert(String::from({param.key}), String::from({param.val}));
        {%}

        let {capability_var}_request = CapabilityRequest{ capability: {CAPABILITY::AUDIO}, params }; 
        


    {%}

    let capabilities = vec![{capability_var}_request, {%= {capability_var}_request in capabilities %}];

    let manifest = Manifest{
        out: {# OUT instruction},
        capabilities
    };

    let manifest = manifest.to_bytes();


```

## MODEL

Model should take a `.tflite` file and generate quantized byte array `&[u8]`.

Similar to the C approach for [tensorflow lite micro](https://github.com/tensorflow/tensorflow/blob/master/tensorflow/lite/micro/examples/micro_speech/train/train_micro_speech_model.ipynb?short_path=1a1d5ff#L560): 

```xxd -i {MODEL_TFLITE} > {MODEL_TFLITE_MICRO}``` 

This should be added to the Rune Cargo project as `{name_of_model}.rs` and 
add `mod {name_of_model}`. 

### Model Bytes Module
```
//{name_of_model}.rs 

pub model_bytes() -> Vec<u8> {

    return vec!| {quantized bytes of the model file} |;
}

```

### Model declaration in lib.rs
```
mod {name_of_model};

// _call
...

let tf_micro_{model_name}_model = model_load({name_of_model}::model_bytes[..]);

// used below in the pipeline
...

```

## PROC_BLOCK

Add dependency into the `Cargo.toml` file:

```

[dependencies]
...
{proc_block_name} = { github ... }

```

Add the `use` in the `lib.rs`.

```
use {proc_block_module_name};

//_call ... 
{

let pipeline = runic_type::Pipeline::new();

pipeline.add({proc_block_module_name}::Processor::new());

...

pipeline.add({any_model});


}

```


## RUN

Add final command to run `{pipeline_var_name}` in `lib.rs`; 


## OUT 

Adds out key to Manifest. [OUTPUT](https://github.com/hotg-ai/runic-types/blob/master/src/provider.rs#L9) enum is used.

```
    let manifest = Manifest{
        out: {# OUT instruction},
        capabilities
    };
```


## Links

* https://docs.rs/rust-crypto/0.2.36/crypto/sha1/index.html
* https://doc.rust-lang.org/std/fs/fn.create_dir_all.html




