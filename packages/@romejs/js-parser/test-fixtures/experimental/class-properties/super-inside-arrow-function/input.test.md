# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romejs/js-parser/index.test.ts --update-snapshots` to update.

## `experimental > class-properties > super-inside-arrow-function`

```javascript
Program {
  comments: Array []
  corrupt: false
  diagnostics: Array []
  directives: Array []
  filename: 'input.js'
  hasHoistedVars: false
  interpreter: undefined
  mtime: undefined
  sourceType: 'script'
  syntax: Array []
  loc: Object {
    filename: 'input.js'
    end: Object {
      column: 0
      index: 47
      line: 4
    }
    start: Object {
      column: 0
      index: 0
      line: 1
    }
  }
  body: Array [
    ClassDeclaration {
      id: BindingIdentifier {
        name: 'A'
        loc: Object {
          filename: 'input.js'
          end: Object {
            column: 7
            index: 7
            line: 1
          }
          start: Object {
            column: 6
            index: 6
            line: 1
          }
        }
      }
      loc: Object {
        filename: 'input.js'
        end: Object {
          column: 1
          index: 46
          line: 3
        }
        start: Object {
          column: 0
          index: 0
          line: 1
        }
      }
      meta: ClassHead {
        implements: undefined
        superTypeParameters: undefined
        typeParameters: undefined
        loc: Object {
          filename: 'input.js'
          end: Object {
            column: 1
            index: 46
            line: 3
          }
          start: Object {
            column: 0
            index: 0
            line: 1
          }
        }
        superClass: ReferenceIdentifier {
          name: 'B'
          loc: Object {
            filename: 'input.js'
            end: Object {
              column: 17
              index: 17
              line: 1
            }
            start: Object {
              column: 16
              index: 16
              line: 1
            }
          }
        }
        body: Array [
          ClassProperty {
            key: StaticPropertyKey {
              value: Identifier {
                name: 'foo'
                loc: Object {
                  filename: 'input.js'
                  end: Object {
                    column: 5
                    index: 25
                    line: 2
                  }
                  start: Object {
                    column: 2
                    index: 22
                    line: 2
                  }
                }
              }
              variance: undefined
              loc: Object {
                filename: 'input.js'
                end: Object {
                  column: 5
                  index: 25
                  line: 2
                }
                start: Object {
                  column: 2
                  index: 22
                  line: 2
                }
              }
            }
            value: ArrowFunctionExpression {
              loc: Object {
                filename: 'input.js'
                end: Object {
                  column: 23
                  index: 43
                  line: 2
                }
                start: Object {
                  column: 8
                  index: 28
                  line: 2
                }
              }
              head: FunctionHead {
                async: false
                hasHoistedVars: false
                params: Array []
                predicate: undefined
                rest: undefined
                returnType: undefined
                thisType: undefined
                loc: Object {
                  filename: 'input.js'
                  end: Object {
                    column: 14
                    index: 34
                    line: 2
                  }
                  start: Object {
                    column: 8
                    index: 28
                    line: 2
                  }
                }
              }
              body: CallExpression {
                arguments: Array []
                loc: Object {
                  filename: 'input.js'
                  end: Object {
                    column: 23
                    index: 43
                    line: 2
                  }
                  start: Object {
                    column: 14
                    index: 34
                    line: 2
                  }
                }
                callee: MemberExpression {
                  loc: Object {
                    filename: 'input.js'
                    end: Object {
                      column: 21
                      index: 41
                      line: 2
                    }
                    start: Object {
                      column: 14
                      index: 34
                      line: 2
                    }
                  }
                  object: Super {
                    loc: Object {
                      filename: 'input.js'
                      end: Object {
                        column: 19
                        index: 39
                        line: 2
                      }
                      start: Object {
                        column: 14
                        index: 34
                        line: 2
                      }
                    }
                  }
                  property: StaticMemberProperty {
                    value: Identifier {
                      name: 'x'
                      loc: Object {
                        filename: 'input.js'
                        end: Object {
                          column: 21
                          index: 41
                          line: 2
                        }
                        start: Object {
                          column: 20
                          index: 40
                          line: 2
                        }
                      }
                    }
                    loc: Object {
                      filename: 'input.js'
                      end: Object {
                        column: 21
                        index: 41
                        line: 2
                      }
                      start: Object {
                        column: 20
                        index: 40
                        line: 2
                      }
                    }
                  }
                }
              }
            }
            definite: undefined
            typeAnnotation: undefined
            loc: Object {
              filename: 'input.js'
              end: Object {
                column: 24
                index: 44
                line: 2
              }
              start: Object {
                column: 2
                index: 22
                line: 2
              }
            }
            meta: ClassPropertyMeta {
              abstract: false
              accessibility: undefined
              optional: false
              readonly: false
              static: false
              typeAnnotation: undefined
              start: Object {
                column: 2
                index: 22
                line: 2
              }
              loc: Object {
                filename: 'input.js'
                end: Object {
                  column: 5
                  index: 25
                  line: 2
                }
                start: Object {
                  column: 2
                  index: 22
                  line: 2
                }
              }
            }
          }
        ]
      }
    }
  ]
}
```