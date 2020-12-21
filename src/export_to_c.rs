use crate::dfa::Dfa;

/// Generate C-code for a state machine.
pub fn write_c_code(dfa: &Dfa, basename: &str) -> std::io::Result<()> {
    write_c_header(dfa, basename)?;
    write_c_source(dfa, basename)?;

    Ok(())
}

fn write_c_header(dfa: &Dfa, basename: &str) -> std::io::Result<()> {
    let filename = format!("{}.h", basename);
    let mut f = std::fs::File::create(filename)?;
    use std::io::Write;

    let token_types = &dfa.token_types;

    writeln!(
        f,
        r###"
#ifndef SCANNER_H
#define SCANNER_H

"###
    )?;

    writeln!(f, "enum token_type {{")?;
    for typ in token_types {
        writeln!(f, "    TOKEN_TYP_{},", typ)?;
    }
    writeln!(f, "    TOKEN_ERROR,")?;
    writeln!(f, "}};")?;

    writeln!(
        f,
        r###"
typedef struct {{
    int state;
    int index;
    int start_index;

    // Of matched token:
    int tok_start;
    int tok_end;
    enum token_type tok_type;
}} lexer_t;

lexer_t* lexer_new();
void lexer_init(lexer_t* lex);
void lexer_add(lexer_t* lex, int c);
const char* lexer_token_type_name(enum token_type token_type);

#endif"###
    )?;

    Ok(())
}

fn write_c_source(dfa: &Dfa, basename: &str) -> std::io::Result<()> {
    let (token_types, all_transitions, accepting_states, error_state) = (
        &dfa.token_types,
        &dfa.transitions,
        &dfa.accepting,
        &dfa.error_state,
    );

    let filename = format!("{}.c", basename);
    let mut f = std::fs::File::create(filename)?;
    use std::io::Write;

    let header = r###"

// IMPLEMENTATION

#include <stdlib.h>
#include "scanner.h"

lexer_t* lexer_new()
{
    lexer_t* lex = malloc(sizeof(lexer_t));
    lexer_init(lex);
    return lex;
}

void lexer_init(lexer_t* lex)
{
    lex->state = 0;
    lex->index = 0;
    lex->start_index = 0;
    lex->tok_start = 0;
    lex->tok_end = 0;
    lex->tok_type = TOKEN_ERROR;
}

// Mark the current cursor as accepting the given ID
void lexer_mark_accepting(lexer_t* lex, int id)
{
    lex->tok_type = id;
    lex->tok_start = lex->start_index;
    lex->tok_end = lex->index;
}

"###;
    write!(f, "{}", header)?;

    writeln!(f, "void lexer_add(lexer_t* lex, int c) {{")?;

    // Create transition code
    writeln!(f, "    // Transition to next state, based on character")?;
    writeln!(f, "    switch (lex->state)")?;
    writeln!(f, "    {{")?;
    for (state, transitions) in all_transitions {
        writeln!(f, "        case {}:", state)?;
        writeln!(f, "        {{")?;

        // TODO: sort below in a binary splitted tree (tree of if statements)
        for (cc, next_state) in transitions {
            writeln!(f, "            // --> {:?}", cc)?;
            for c_range in &cc.ranges {
                if c_range.begin == c_range.end {
                    writeln!(f, "            if (c == '{}')", c_range.begin)?;
                } else {
                    writeln!(
                        f,
                        "            if (('{}' <= c) && (c <= '{}'))",
                        c_range.begin, c_range.end
                    )?;
                }
                writeln!(f, "            {{")?;
                writeln!(f, "                lex->state = {};", next_state)?;
                writeln!(f, "            }}")?;
            }
        }

        writeln!(f, "        }}")?;
        writeln!(f, "        break;")?;
        writeln!(f)?;
    }

    writeln!(f, "    }}")?;
    writeln!(f)?;

    writeln!(f, "    // Check for accept or error states")?;
    writeln!(f, "    switch (lex->state)")?;
    writeln!(f, "    {{")?;

    // Accepting states:
    for (accept_state, tokens) in accepting_states {
        let tok_typ = tokens.first().unwrap();
        writeln!(f, "        case {}:  // {:?}", accept_state, tokens)?;
        writeln!(
            f,
            "            lexer_mark_accepting(lex, TOKEN_TYP_{});",
            tok_typ
        )?;
        writeln!(f, "            break;")?;
        writeln!(f)?;
    }

    // Error state:
    writeln!(f, "        case {}:  // Error state", error_state)?;
    writeln!(
        f,
        "            // If we cannot match further, we are in the error state"
    )?;
    writeln!(f, "         {{")?;
    writeln!(f, "             // emit token")?;
    writeln!(f, "             // set index back")?;
    writeln!(
        f,
        "             lex->start_index = lex->index = lex->tok_end;"
    )?;
    writeln!(f, "             lex->state = 0;")?;
    writeln!(f, "         }}")?;
    writeln!(f, "         break;")?;
    writeln!(f)?;

    writeln!(f, "    }}")?;

    writeln!(f, "}}")?;

    writeln!(f)?;

    writeln!(
        f,
        "const char* lexer_token_type_name(enum token_type token_type) {{"
    )?;
    writeln!(f, "   switch (token_type) {{")?;

    for token_type in token_types {
        writeln!(
            f,
            r###"      case TOKEN_TYP_{0}: return "{0}"; "###,
            token_type
        )?;
    }
    writeln!(f, "   }}")?;
    writeln!(f, r###"   return "?"; "###)?;
    writeln!(f, "}}")?;

    writeln!(f, "// END OF IMPLEMENTATION")?;
    writeln!(f)?;

    Ok(())
}
