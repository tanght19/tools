# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/css-parser/index.test.ts --update-snapshots` to update.

## `invalid > calc-division`

```javascript
CSSRoot {
	comments: Array []
	corrupt: false
	integrity: undefined
	loc: SourceLocation invalid/calc-division/input.css 1:0-3:1
	path: RelativeFilePath<invalid/calc-division/input.css>
	diagnostics: Array [
		Object {
			origins: Array [Object {category: "parse"}]
			description: Object {
				advice: Array []
				category: "parse"
				categoryValue: "css"
				message: RAW_MARKUP {value: "Incorrect character, expected a number or a parenthesis"}
			}
			location: Object {
				integrity: undefined
				language: "css"
				sourceText: undefined
				end: Position 2:20
				path: RelativeFilePath<invalid/calc-division/input.css>
				start: Position 2:17
			}
		}
	]
	body: Array [
		CSSRule {
			loc: SourceLocation invalid/calc-division/input.css 1:0-3:1
			prelude: Array [
				CSSSelector {
					loc: SourceLocation invalid/calc-division/input.css 1:0-1:7
					patterns: Array [
						CSSClassSelector {
							value: "style"
							loc: SourceLocation invalid/calc-division/input.css 1:0-1:6
						}
					]
				}
			]
			block: CSSBlock {
				value: Array [
					CSSDeclaration {
						name: "width"
						value: Array [
							CSSCalcFunction {
								name: "calc"
								loc: SourceLocation invalid/calc-division/input.css 2:13-2:20
								params: Array [
									CSSCalcSum {
										value: Array [
											CSSCalcProduct {
												value: Array [
													CSSCalcValue {
														value: CSSDimension {
															value: 2
															unit: "px"
															loc: SourceLocation invalid/calc-division/input.css 2:13-2:13
														}
														loc: SourceLocation invalid/calc-division/input.css 2:13-2:16
													}
												]
												loc: SourceLocation invalid/calc-division/input.css 2:16-2:17
											}
										]
										loc: SourceLocation invalid/calc-division/input.css 2:13-2:17
									}
								]
							}
							CSSRaw {
								value: undefined
								loc: SourceLocation invalid/calc-division/input.css 2:20-2:21
							}
						]
						important: false
						loc: SourceLocation invalid/calc-division/input.css 2:1-2:21
					}
				]
				startingTokenValue: "{"
				loc: SourceLocation invalid/calc-division/input.css 1:7-3:1
			}
		}
	]
}
```