use serde::{Deserialize, Serialize};

trait GameDriver {
    fn apply(&self, input: &str) -> String;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
enum Op {
    Uppercase(UppercaseOp),
    Prefix(PrefixOp),
}

#[derive(Serialize, Deserialize, Debug)]
struct UppercaseOp {}

#[derive(Serialize, Deserialize, Debug)]
struct PrefixOp {
    prefix: String,
}

impl Operation for UppercaseOp {
    fn apply(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

impl Operation for PrefixOp {
    fn apply(&self, input: &str) -> String {
        format!("{}{}", self.prefix, input)
    }
}

// Implement Operation for the enum to dispatch to concrete types
impl Operation for Op {
    fn apply(&self, input: &str) -> String {
        match self {
            Op::Uppercase(o) => o.apply(input),
            Op::Prefix(o) => o.apply(input),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ops: Vec<Op> = vec![
        Op::Uppercase(UppercaseOp {}),
        Op::Prefix(PrefixOp {
            prefix: "Hello: ".into(),
        }),
    ];

    let json = serde_json::to_string_pretty(&ops)?;
    println!("serialized:\n{}\n", json);

    let de: Vec<Op> = serde_json::from_str(&json)?;
    for op in de {
        println!("{}", op.apply("world"));
    }
    Ok(())
}
