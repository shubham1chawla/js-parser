use relational::RelationalExpressionParsable;

use super::*;

pub trait AssignmentExpressionParsable {
    /**
     * AssignmentExpression
     *  : RelationalExpression
     *  | LeftHandSideExpression ASSIGNMENT_OPERATOR AssignmentExpression
     *  ;
     */
    fn assignment_expression(&mut self) -> Result<Tree, SyntaxError>;

    /**
     * Whether the token is an assignment operator.
     */
    fn is_assignment_operator(&self) -> bool;

    /**
     * Extra check whether it's valid assignment target.
     */
    fn check_valid_assignment_target(&mut self, node: Tree) -> Result<Tree, SyntaxError>;

    /**
     * AssignmentOperator
     *  : SIMPLE_ASSIGNMENT_OPERATOR
     *  | COMPLEX_ASSIGNMENT_OPERATOR
     *  ;
     */
    fn assignment_operator(&mut self) -> Result<Token, SyntaxError>;
}

impl AssignmentExpressionParsable for Parser {
    fn assignment_expression(&mut self) -> Result<Tree, SyntaxError> {
        let mut left = self.relational_expression()?;

        // Checking if the lookahead token is not of assignment type, then its an AdditiveExpression
        if !self.is_assignment_operator() {
            return Ok(left);
        }

        // Consuming assignment operator
        let operator = self.assignment_operator()?.value;

        // Checking if the left hand side expression is valid, aka an identifier
        left = self.check_valid_assignment_target(left)?;

        // Right-recursing to create the AssignmentExpression
        let right = self.assignment_expression()?;

        Ok(Tree::AssignmentExpression { 
            operator, 
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn is_assignment_operator(&self) -> bool {
        match self.lookahead.token_type {
            TokenType::SimpleAssignmentOperator | TokenType::ComplexAssignmentOperator => true,
            _ => false,
        }
    }

    fn check_valid_assignment_target(&mut self, node: Tree) -> Result<Tree, SyntaxError> {
        if let Tree::Identifier {..} = node {
            return Ok(node);
        }
        Err(SyntaxError {
            message: String::from("Invalid left-hand side in assignment expression, expected Identifier!"),
        })
    }

    fn assignment_operator(&mut self) -> Result<Token, SyntaxError> {
        match self.lookahead.token_type {
            TokenType::SimpleAssignmentOperator => self.eat(TokenType::SimpleAssignmentOperator),
            _ => self.eat(TokenType::ComplexAssignmentOperator),
        }
    }
}

#[cfg(test)]
mod tests {
    use parsable::tests::{assert_syntax_error, assert_tree};

    use super::*;

    #[test]
    fn test_parse_simple_assignment_expression_1() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("="), 
                        left: Box::new(Tree::Identifier { name: String::from("num") }), 
                        right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "num = 42;");
    }

    #[test]
    fn test_parse_simple_assignment_expression_2() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("="), 
                        left: Box::new(Tree::Identifier { name: String::from("str") }), 
                        right: Box::new(Tree::StringLiteral { value: String::from("Hello, World!") }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "str = 'Hello, World!';");
    }

    #[test]
    fn test_parse_simple_assignment_expression_3() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("="), 
                        left: Box::new(Tree::Identifier { name: String::from("xyz") }), 
                        right: Box::new(Tree::BinaryExpression { 
                            operator: String::from("+"), 
                            left: Box::new(Tree::NumericLiteral { value: 2.0 }), 
                            right: Box::new(Tree::NumericLiteral { value: 3.0 }),
                        }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "xyz = 2 + 3;");
    }

    #[test]
    fn test_parse_chained_assignment_expression() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("="), 
                        left: Box::new(Tree::Identifier { name: String::from("x") }), 
                        right: Box::new(Tree::AssignmentExpression { 
                            operator: String::from("="), 
                            left: Box::new(Tree::Identifier { name: String::from("y") }), 
                            right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                        }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "x = y = 42;");
    }

    #[test]
    fn test_parse_complex_assignment_expression_1() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("+="), 
                        left: Box::new(Tree::Identifier { name: String::from("num") }), 
                        right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "num += 42;");
    }

    #[test]
    fn test_parse_complex_assignment_expression_2() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("-="), 
                        left: Box::new(Tree::Identifier { name: String::from("num") }), 
                        right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "num -= 42;");
    }

    #[test]
    fn test_parse_complex_assignment_expression_3() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("*="), 
                        left: Box::new(Tree::Identifier { name: String::from("num") }), 
                        right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "num *= 42;");
    }

    #[test]
    fn test_parse_complex_assignment_expression_4() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("/="), 
                        left: Box::new(Tree::Identifier { name: String::from("num") }), 
                        right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "num /= 42;");
    }

    #[test]
    fn test_parse_invalid_assignment_expression() {
        let expected = SyntaxError {
            message: String::from("Invalid left-hand side in assignment expression, expected Identifier!"),
        };
        assert_syntax_error(expected, "42 = 42;");
    }
}