/** 
* ZoKrates Grammar
* Author: Jacob Eberhardt
*/

grammar zokrates;
 
/* 
* Parser Rules
*/ 

file 
    : import_directive* program
    ;

import_directive
    : 'import' Path ('as' Identifier)? Newline+
    ;

program
    : functionDefinition 
    | program functionDefinition
    ;

functionDefinition
    : Newline* 'def' Identifier '(' parameterList? ')' '->' '(' typeList ')' ':' Newline+ statementList Newline*;

parameterList
    :   'private'? declaration
    |   parameterList ',' 'private'? declaration
    ;

typeList
    : typeName
    | typeList ',' typeName
    ;

statementList
    : statement
    | statementList Newline+ statement
    ;

declaration: typeName Identifier;


statement
    : definitionStatement
    | assignmentStatement
    | multiReturnStatement
    | expressionStatement
    | iterationStatement
    | returnStatement
    ;

iterationStatement
    : 'for' declaration 'in' Constant '..' Constant 'do' Newline* statementList Newline* 'endfor'
    ;    

definitionStatement
    : declaration '=' expression
    ;

multiReturnStatement
    : multiAssignment '=' postfixExpression '(' expressionList? ')'
    ;

// Left side of multireturn statement. Can be assignments or declarations.
multiAssignment
    : declaration
    | Identifier //assignment
    | declaration ',' multiAssignment
    | Identifier ',' multiAssignment
    ;

assignmentStatement
    : Identifier('['(Constant|Identifier)?']')*? '=' expression
    ;

expressionStatement
    : expression
    ;

returnStatement
    : 'return' expressionList
    ;

expressionList
    : expression
    | expressionList ',' expression
    ;

typeName 
    : 'field'('['Constant?']')*?
    | 'bool'('['Constant?']')*?
    ;

expression
    : primaryExpression
    | inclusiveOrExpression // includes all logical and arithmetic rules. Wrapper name?
    ;

primaryExpression
    :   Identifier
    |   Constant
    |   '(' expression ')'
    |   '[' expressionList ']' //special case for array initialization
    ;

unaryExpression
    : postfixExpression
    | conditionalExpression
    | '!' unaryExpression
    ;

conditionalExpression
    : 'if' expression 'then' expression 'else' expression 'fi'
    ;

relationalExpression
    :   additiveExpression
    |   relationalExpression '<' additiveExpression
    |   relationalExpression '>' additiveExpression
    |   relationalExpression '<=' additiveExpression
    |   relationalExpression '>=' additiveExpression
    ;

equalityExpression
    :   relationalExpression
    |   equalityExpression '==' relationalExpression
    |   equalityExpression '!=' relationalExpression
    ;

andExpression
    :   equalityExpression
    |   andExpression '&' equalityExpression
    ;

exclusiveOrExpression
    :   andExpression
    |   exclusiveOrExpression '^' andExpression
    ;

inclusiveOrExpression
    :   exclusiveOrExpression
    |   inclusiveOrExpression '|' exclusiveOrExpression
    ;

postfixExpression
    :   primaryExpression
    |   postfixExpression '[' expression ']'
    |   postfixExpression '(' expressionList? ')'
    ;

additiveExpression
    :   multiplicativeExpression
    |   additiveExpression '+' multiplicativeExpression
    |   additiveExpression '-' multiplicativeExpression
    ;

multiplicativeExpression
    :   powerExpression
    |   multiplicativeExpression '*' powerExpression
    |   multiplicativeExpression '/' powerExpression
    ;

powerExpression
    :  unaryExpression
    |  unaryExpression '**' powerExpression
    ;


/* 
* Lexer Rules - Token Definitions
*/ 

// Import  
Import : 'import';
DoubleQuote : '"';
As : 'as';

// Declarations
Def : 'def';

// Simple Types
Boolean : 'bool'; // TODO: Decide on naming & caps rules for types!
Field : 'field';

// Modifiers
Private : 'private';

// Looping
For : 'for';
In : 'in';
Dotdot : '..';
Do : 'do';
Endfor : 'endfor';

// Branching
If : 'if';
Then : 'then';
Else : 'else';
Fi : 'fi';
Return : 'return';

// Assignment
Assign : '=';

// Comparison
Equal : '==';
NotEqual : '!=';
Less : '<';
LessEqual : '<=';
Greater : '>';
GreaterEqual : '>=';

// Arithmetics
Plus : '+';
Minus : '-';
Mult : '*';
Div : '/';
Pow : '**';

// Boolean Arithmetics
And : '&';
Or : '|';
Not : '!';

// Separators
Colon : ':';
Comma : ',';
Arrow : '->';
Semi : ';'; //currently not used


// Separator for Statements
Newline : ('\r' '\n'? | '\n' )+;

// Parentheses, Brackets, Curly Braces
LeftParen : '(';
RightParen : ')';
LeftBracket : '[';
RightBracket : ']';
LeftBrace : '{'; //currently not used
RightBrace : '}'; //currently not used

// Path
Path : '"'(.+?)'/'.+?'.'.+?'"';

// Identifiers
Identifier : Nondigit (Nondigit | Digit)*;

// Integer
Constant : FieldConstant;

// Numbers
FieldConstant : '0' | IntegerConstant;
IntegerConstant : NonzeroDigit Digit*;

// Fragments
fragment Nondigit : [a-zA-Z];
fragment Digit : [0-9];
fragment NonzeroDigit : [1-9];


// Ignore linebreak after \
MultiLine
    :  '\\'[\r\n] -> skip
    ;

// Skip Whitespaces and Comments
Whitespace 
    : [ \t]+ -> skip
    ;

BlockComment
    :   '/*' .*? '*/' -> skip
    ;

LineComment
    :   '//' ~[\r\n]* -> skip
    ;

