/*
Demo of C code generated by the scanner generator.

*/
#include <stdio.h>
#include <string.h>

#include "scanner.h"

void main() {
    char* test_text = "67432 2323.3  bla  ++ mo";

    zowski_lexer_t* lex = zowski_lexer_new();

    printf("Scanning: %s (len=%d)\n", test_text, strlen(test_text));

    int tok_len;
    #define TOKEN_TEXT_BUFSIZE 30
    char buf[TOKEN_TEXT_BUFSIZE];
    enum add_result res;

    while (test_text[lex->index] != 0) {
        int c = test_text[lex->index++];
        res = zowski_lexer_add(lex, c);
        if (res == ADD_DONE)
        {
            if (lex->tok_type == TOKEN_ERROR)
            {
                printf("ERROR!\n");
                break;
            }
            else
            {
                // display token:
                tok_len = lex->tok_end - lex->tok_start;
                if (tok_len > TOKEN_TEXT_BUFSIZE-1) tok_len = TOKEN_TEXT_BUFSIZE-1;
                strncpy(buf, &test_text[lex->tok_start], tok_len);
                buf[tok_len] = 0;
                printf("TOK: typ=%s %d-%d: [%s]\n", zowski_lexer_token_type_name(lex->tok_type), lex->tok_start, lex->tok_end, buf);

                // Update lexer:
                lex->index = lex->tok_end;
                lex->tok_type = TOKEN_ERROR;
            }
        }
    }

    zowski_lexer_delete(lex);
}
