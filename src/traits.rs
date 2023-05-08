use extension_trait::extension_trait;
use syntect::easy::HighlightLines;
use syntect::highlighting::Style;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use syntect::Error;

#[extension_trait]
pub impl Highlight for HighlightLines<'_> {
    fn highlight0<'source>(
        &mut self,
        source: &'source str,
        syntax_set: &SyntaxSet,
    ) -> Result<Vec<Vec<(Style, &'source str)>>, Error> {
        LinesWithEndings::from(source).map(|line| self.highlight_line(line, syntax_set)).collect()
    }
}
