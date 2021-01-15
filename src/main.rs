#[macro_use]
extern crate handlebars;

use anyhow::Result;
use async_std::io;
use async_std::prelude::*;
use clap::clap_app;
use handlebars::Handlebars;
use num_cpus;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::iter::FromIterator;
use sysinfo::SystemExt;

struct TemplateRenderer<'a> {
    inner: Handlebars<'a>,
}

#[derive(Serialize)]
struct TemplateRendererContext {
    cpu: HashMap<String, String>,
    env: HashMap<String, String>,
    mem: HashMap<String, String>,
}

impl TemplateRendererContext {
    pub fn new() -> Result<TemplateRendererContext> {
        let mut cpu = HashMap::<String, String>::new();
        let mut mem = HashMap::<String, String>::new();
        let env = HashMap::<String, String>::from_iter(env::vars());
        let system = sysinfo::System::new_all();

        cpu.insert("logical".to_string(), num_cpus::get().to_string());
        cpu.insert("physical".to_string(), num_cpus::get_physical().to_string());

        mem.insert("total".to_string(), system.get_total_memory().to_string());
        mem.insert("used".to_string(), system.get_used_memory().to_string());
        mem.insert(
            "free".to_string(),
            (system.get_total_memory() - system.get_used_memory()).to_string(),
        );

        Ok(TemplateRendererContext { cpu, env, mem })
    }
}

impl<'a> TemplateRenderer<'a> {
    pub fn new() -> TemplateRenderer<'a> {
        TemplateRenderer {
            inner: Handlebars::new(),
        }
    }

    pub fn from<S>(template: S) -> Result<TemplateRenderer<'a>>
    where
        S: AsRef<str>,
    {
        let mut inner = Handlebars::new();

        inner.register_template_string("stdin", template)?;

        inner.register_helper("add", Box::new(add));
        inner.register_helper("div", Box::new(div));
        inner.register_helper("mod", Box::new(mdo));
        inner.register_helper("mul", Box::new(mul));
        inner.register_helper("sub", Box::new(sub));

        Ok(TemplateRenderer { inner })
    }

    pub async fn from_stdin() -> Result<TemplateRenderer<'a>> {
        let mut stdin = io::stdin();
        let mut template = String::new();

        stdin.read_to_string(&mut template).await?;

        Ok(TemplateRenderer::from(&template)?)
    }

    pub fn render(&self) -> Result<String> {
        Ok(self
            .inner
            .render("stdin", &TemplateRendererContext::new()?)?)
    }

    pub async fn render_to_stdout(&self) -> Result<()> {
        let mut stdout = io::stdout();

        Ok(stdout.write_all(self.render()?.as_bytes()).await?)
    }
}

impl<'a> Default for TemplateRenderer<'a> {
    fn default() -> Self {
        Self::new()
    }
}

handlebars_helper!(add: |x: i64, y: i64| x + y);
handlebars_helper!(div: |x: i64, y: i64| x / y);
handlebars_helper!(mdo: |x: i64, y: i64| x % y);
handlebars_helper!(mul: |x: i64, y: i64| x * y);
handlebars_helper!(sub: |x: i64, y: i64| x - y);

#[async_std::main]
async fn main() -> Result<()> {
    clap_app!(myapp =>
        (version: "0.1.0")
        (author: "Steve Jabour <steve@jabour.me>")
        (about: "Substitutes the values of environment variables, but with handlebars.")
        (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@arg INPUT: "Sets the input file to use")
        (@arg debug: -d ... "Sets the level of debugging information")
    )
    .get_matches();

    TemplateRenderer::from_stdin()
        .await?
        .render_to_stdout()
        .await?;

    Ok(())
}
