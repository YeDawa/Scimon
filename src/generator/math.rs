use mathjax::MathJax;
use tokio::task;
use futures::future::join_all;

use std::{
    fs::write,
    error::Error,
};

use crate::{
    syntax::vars::Vars,

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

pub struct Math {
    contents: Vec<(String, String)>,
}

impl Math {

    pub fn new(contents: &str) -> Self {
        Self {
            contents: Vars.get_all_math(contents),
        }
    }

    pub async fn render(&self) -> Result<(), Box<dyn Error>> {
        let math_expressions = &self.contents;

        if math_expressions.is_empty() {
            return Ok(());
        }

        let renderer = MathJax::new()?;
        UI::section_header("math render", "normal");

        // MathJax exposes a single renderer, so the typesetting itself stays
        // sequential; the CPU-heavy rasterization and disk write of each result
        // are offloaded to the blocking pool so they run concurrently.
        let mut tasks = Vec::new();

        for (expression, file_name) in math_expressions {
            let file_name = file_name.clone();

            let result = match renderer.render(expression) {
                Ok(result) => result,
                Err(e) => {
                    ErrorsAlerts::generic(&e.to_string());
                    continue;
                }
            };

            tasks.push(task::spawn_blocking(move || -> Result<(), Box<dyn Error + Send + Sync>> {
                if file_name.contains(".png") {
                    let image = result.into_image(10.0)?;
                    image.save(&file_name)?;
                } else if file_name.contains(".svg") {
                    let svg_string = result.into_raw();
                    write(&file_name, svg_string)?;
                } else {
                    ErrorsAlerts::math(&file_name);
                    return Ok(());
                }

                SuccessAlerts::math(&file_name);
                Ok(())
            }));
        }

        for handle in join_all(tasks).await {
            match handle {
                Ok(Err(e)) => ErrorsAlerts::generic(&e.to_string()),
                Err(e) => ErrorsAlerts::generic(&e.to_string()),
                Ok(Ok(())) => {}
            }
        }

        Ok(())
    }

}
