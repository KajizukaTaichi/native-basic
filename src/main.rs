fn main() {
    let mut compiler = Compiler {
        index: 0,
        variable: Vec::new(),
    };
    let code = compiler.build(include_str!("../example.nab")).unwrap();
    println!("{code}");
}

struct Compiler {
    index: usize,
    variable: Vec<String>,
}
impl Compiler {
    fn build(&mut self, source: &str) -> Option<String> {
        let ast = source
            .lines()
            .map(|x| Stmt::parse(x))
            .collect::<Option<Vec<_>>>()?;
        let code = ast
            .iter()
            .map(|x| x.compile(self))
            .collect::<Vec<String>>()
            .concat();
        Some(format!(
            "[ORG 0x7C00]\n[BITS 16]\n{code}\tjmp $\n{lib}\n{var}\ntimes 510 - ($ - $$) db 0\ndw 0xAA55\n",
            lib = include_str!("./lib.asm"),
            var = self.variable.concat()
        ))
    }
}

#[derive(Debug, Clone)]
enum Stmt {
    Say(Value),
}

impl Stmt {
    fn parse(source: &str) -> Option<Stmt> {
        let source = source.trim();
        if let Some(token) = source.strip_prefix("say") {
            Some(Stmt::Say(Value::parse(token)?))
        } else {
            None
        }
    }

    fn compile(&self, ctx: &mut Compiler) -> String {
        match self {
            Stmt::Say(text) => format!("\tmov si, {}\n\tcall print\n", text.compile(ctx)),
        }
    }
}

#[derive(Debug, Clone)]
enum Value {
    String(String),
}
impl Value {
    fn parse(source: &str) -> Option<Value> {
        let source = source.trim();
        if let Some(token) = source
            .strip_prefix("\"")
            .map(|x| x.strip_suffix("\""))
            .flatten()
        {
            Some(Value::String(token.to_string()))
        } else {
            None
        }
    }

    fn compile(&self, ctx: &mut Compiler) -> String {
        match self {
            Value::String(text) => {
                let name = format!("str_literal_{}", ctx.index);
                ctx.variable.push(format!("{name} db \"{text}\", 0\n"));
                ctx.index += 1;
                name
            }
        }
    }
}
