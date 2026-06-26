use crate::consts::addons::Addons;

pub struct StaticFiles;

impl StaticFiles {

    pub fn get_default_css_style(&self) -> &'static str {
        Addons::DEFAULT_CSS_STYLE
    }

    pub fn get_default_latex_css_style(&self) -> &'static str {
        Addons::DEFAULT_LATEX_CSS_STYLE
    }

    pub fn get_default_latex_js_script(&self) -> &'static str {
        Addons::DEFAULT_LATEX_JS_SCRIPT
    }

}