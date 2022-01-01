
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "pascal_scanner.h"

void main()
{
    const char* source_filename = "macho.pas";
    printf("Scanning: %s\n", source_filename);

    char *source_buffer = 0;
    int source_size = 0;
    FILE *f = fopen(source_filename, "r");
    if (f)
    {
        fseek(f, 0, SEEK_END);
        source_size = ftell(f);
        source_buffer = malloc(source_size + 1);
        fseek(f, 0, SEEK_SET);
        fread(source_buffer, 1, source_size, f);
        source_buffer[source_size] = 0;
        fclose(f);
    }
    else
    {
        printf("File not found: %s\n", source_filename);
        return;
    }

    zowski_lexer_t* lex = zowski_lexer_new();
    zowski_lexer_feed(lex, source_buffer, source_size);

    #define TOKEN_TEXT_BUFSIZE 30
    char buf[TOKEN_TEXT_BUFSIZE];
    zowski_result_t res;

    do {
        res = zowski_lexer_next_token(lex);
        switch (res) {
            case ZOWSKI_RESULT_OK:
                {
                    zowski_token_t token_typ = zowski_lexer_get_token_type(lex);
                    switch (token_typ)
                    {
                        case ZOWSKI_TOKEN_TYP_NEWLINE:
                        case ZOWSKI_TOKEN_TYP_WHITESPACE:
                        case ZOWSKI_TOKEN_TYP_LINECOMMENT:
                        case ZOWSKI_TOKEN_TYP_COMMENT:
                            // Ignore some token types.
                            break;
                        default:
                            // display token:
                            zowski_lexer_token_copy_text(lex, buf, TOKEN_TEXT_BUFSIZE);
                            int tok_start = zowski_lexer_get_token_start(lex);
                            int tok_end = zowski_lexer_get_token_end(lex);
                            const char* tok_name = zowski_lexer_token_type_name(lex);

                            printf("Token span=%d-%d typ=%s: [%s]\n", tok_start, tok_end, tok_name, buf);
                    }
                }
                break;
            case ZOWSKI_RESULT_ERROR:
                printf("Errorzz!\n");
                break;
            case ZOWSKI_RESULT_FINISHED:
                break;
            default:
                printf("Now what?\n");
                break;
        }
    } while (res == ZOWSKI_RESULT_OK);

    zowski_lexer_delete(lex);
}
