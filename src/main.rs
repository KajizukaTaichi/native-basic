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
    Let(String, Value),
}

impl Stmt {
    fn parse(source: &str) -> Option<Stmt> {
        let source = source.trim();
        if let Some(token) = source.strip_prefix("say") {
            Some(Stmt::Say(Value::parse(token)?))
        } else if let Some(token) = source.strip_prefix("let") {
            let (name, val) = token.split_once("=")?;
            Some(Stmt::Let(name.trim().to_owned(), Value::parse(val.trim())?))
        } else {
            None
        }
    }

    fn compile(&self, ctx: &mut Compiler) -> String {
        match self {
            Stmt::Say(text) => format!("\tmov si, {}\n\tcall print\n", text.compile(ctx)),
            Stmt::Let(name, val) => {
                let val = val.compile(ctx);
                ctx.variable.push(format!("{name} db {val}\n"));
                ctx.index += 1;
                String::new()
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Value {
    String(String),
    Variable(String),
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
            Some(Value::Variable(source.to_owned()))
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
            Value::Variable(name) => name.to_string(),
        }
    }
}
