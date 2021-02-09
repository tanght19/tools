# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/css-parser/index.test.ts --update-snapshots` to update.

## `functions`

```javascript
CSSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	filename: "functions/input.css"
	integrity: undefined
	loc: SourceLocation functions/input.css 1:0-5:1
	body: Array [
		CSSRule {
			loc: SourceLocation functions/input.css 1:0-5:1
			prelude: Array [
				CSSSelector {
					loc: SourceLocation functions/input.css 1:0-1:7
					patterns: Array [
						CSSClassSelector {
							value: "style"
							loc: SourceLocation functions/input.css 1:0-1:6
						}
					]
				}
			]
			block: CSSBlock {
				value: Array [
					CSSDeclaration {
						name: CSSCustomProperty {
							value: "--fancy"
							loc: SourceLocation functions/input.css 2:1-2:1
						}
						value: Array [
							CSSDimension {
								value: 2
								unit: "px"
								loc: SourceLocation functions/input.css 2:10-2:13
							}
						]
						important: false
						loc: SourceLocation functions/input.css 2:1-2:13
					}
					CSSDeclaration {
						name: "border"
						value: Array [
							CSSVarFunction {
								name: "var"
								loc: SourceLocation functions/input.css 3:9-3:21
								params: Array [
									CSSCustomProperty {
										value: "--fancy"
										loc: SourceLocation functions/input.css 3:20-3:20
									}
								]
							}
						]
						important: false
						loc: SourceLocation functions/input.css 3:1-3:21
					}
					CSSDeclaration {
						name: "font-size"
						value: Array [
							CSSFunction {
								name: "calc"
								loc: SourceLocation functions/input.css 4:12-4:29
								params: Array [
									CSSDimension {
										value: 10
										unit: "px"
										loc: SourceLocation functions/input.css 4:17-4:21
									}
									CSSRaw {
										value: "+"
										loc: SourceLocation functions/input.css 4:22-4:23
									}
									CSSDimension {
										value: 5
										unit: "rem"
										loc: SourceLocation functions/input.css 4:24-4:28
									}
								]
							}
						]
						important: false
						loc: SourceLocation functions/input.css 4:1-4:29
					}
				]
				startingTokenValue: "{"
				loc: SourceLocation functions/input.css 1:7-5:1
			}
		}
	]
}
```