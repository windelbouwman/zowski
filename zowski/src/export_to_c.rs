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

// ATTENTION: automatically generated code, do not edit by hand

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
}} zowski_lexer_t;

// result of char add function
enum add_result {{
    // we need more characters
    ADD_NEED_MORE,

    // we are done with a token
    ADD_DONE,
}};

zowski_lexer_t* zowski_lexer_new();
void zowski_lexer_init(zowski_lexer_t* lex);
void zowski_lexer_delete(zowski_lexer_t* lex);
enum add_result zowski_lexer_add(zowski_lexer_t* lex, int c);
const char* zowski_lexer_token_type_name(enum token_type token_type);

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

// ATTENTION: automatically generated code, do not edit by hand

#include <stdlib.h>
#include "scanner.h"

zowski_lexer_t* zowski_lexer_new()
{
    zowski_lexer_t* lex = malloc(sizeof(zowski_lexer_t));
    zowski_lexer_init(lex);
    return lex;
}

void zowski_lexer_delete(zowski_lexer_t* lex)
{
    free(lex);
}

void zowski_lexer_init(zowski_lexer_t* lex)
{
    lex->state = 0;
    lex->index = 0;
    lex->start_index = 0;
    lex->tok_start = 0;
    lex->tok_end = 0;
    lex->tok_type = TOKEN_ERROR;
}

// Mark the current cursor as accepting the given ID
void zowski_lexer_mark_accepting(zowski_lexer_t* lex, int id)
{
    lex->tok_type = id;
    lex->tok_start = lex->start_index;
    lex->tok_end = lex->index;
}

"###;
    write!(f, "{}", header)?;

    writeln!(
        f,
        "void zowski_lexer_transition(zowski_lexer_t* lex, int c) {{"
    )?;

    // Create transition code
    writeln!(f, "    // Transition to next state, based on character")?;
    writeln!(f, "    switch (lex->state)")?;
    writeln!(f, "    {{")?;
    for (state, transitions) in all_transitions {
        writeln!(f, "        case {}:", state)?;
        writeln!(f, "        {{")?;

        // TODO: sort below in a binary splitted tree (tree of if statements)
        let mut first = true;
        for (character_class, next_state) in transitions {
            // writeln!(f, "            // --> {:?}", character_class)?;
            for c_range in &character_class.ranges {
                if first {
                    write!(f, "            if (")?;
                    first = false;
                } else {
                    write!(f, "            else if (")?;
                }

                if c_range.begin == c_range.end {
                    write!(f, "c == '{}'", c_range.begin)?;
                } else {
                    write!(f, "('{}' <= c) && (c <= '{}')", c_range.begin, c_range.end)?;
                }
                writeln!(f, ")  // {:?}", c_range)?;
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
    writeln!(f, "}}")?;
    writeln!(f)?;

    writeln!(f, "enum add_result zowski_lexer_add(zowski_lexer_t* lex, int c) {{")?;

    writeln!(f, "    zowski_lexer_transition(lex, c);")?;
    writeln!(f)?;
    writeln!(f, "    enum add_result res = ADD_NEED_MORE;")?;
    writeln!(f, "    // Check for accept or error states")?;
    writeln!(f, "    switch (lex->state)")?;
    writeln!(f, "    {{")?;

    // Accepting states:
    for (accept_state, tokens) in accepting_states {
        let tok_typ = tokens.first().unwrap();
        writeln!(f, "        case {}:  // {:?}", accept_state, tokens)?;
        writeln!(
            f,
            "            zowski_lexer_mark_accepting(lex, TOKEN_TYP_{});",
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
    writeln!(f, "             res = ADD_DONE;")?;
    
    writeln!(
        f,
        "             lex->start_index = lex->index = lex->tok_end;"
    )?;
    writeln!(f, "             lex->state = 0;")?;
    writeln!(f, "         }}")?;
    writeln!(f, "         break;")?;
    writeln!(f)?;

    writeln!(f, "    }}")?;
    writeln!(f, "    return res;")?;
    writeln!(f, "}}")?;
    writeln!(f)?;

    writeln!(
        f,
        "const char* zowski_lexer_token_type_name(enum token_type token_type) {{"
    )?;
    writeln!(f, "    switch (token_type) {{")?;

    for token_type in token_types {
        writeln!(
            f,
            r###"        case TOKEN_TYP_{0}: return "{0}"; "###,
            token_type
        )?;
    }
    writeln!(f, "    }}")?;
    writeln!(f, r###"    return "?"; "###)?;
    writeln!(f, "}}")?;

    writeln!(f, "// END OF IMPLEMENTATION")?;
    writeln!(f)?;

    Ok(())
}
