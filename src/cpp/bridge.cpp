#include "bridge.h"

#include "lexer.cpp"

std::unique_ptr<std::vector<BridgeToken>> tokenize_cpp(rust::Str input) {
  std::string str = std::string(input);
  Lexer lexer(str);
  std::vector<Token*> tokens = lexer.tokenize();

  auto result = std::make_unique<std::vector<BridgeToken>>();
  for (Token* token : tokens) {
    BridgeToken bridge_token;
    bridge_token.kind = token->type;
    bridge_token.value = token->value;
    bridge_token.line = token->line;
    bridge_token.column = token->column;
    result->push_back(bridge_token);
    delete token;
  }
  return result;
}
