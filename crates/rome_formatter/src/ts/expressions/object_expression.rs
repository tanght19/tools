use crate::{
	format_elements, group_elements, if_group_breaks, join_elements, soft_indent,
	soft_line_break_or_space, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsObjectExpression;

impl ToFormatElement for JsObjectExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let props = formatter.format_separated(self.members())?;

		Ok(group_elements(format_elements!(
			formatter.format_token(&self.l_curly_token()?)?,
			soft_indent(format_elements![
				join_elements(soft_line_break_or_space(), props),
				if_group_breaks(token(",")),
			]),
			formatter.format_token(&self.r_curly_token()?)?,
		)))
	}
}
