#include <string>
#include <vector>

struct Token {
  std::string type;
  std::string value;
  int line;
  int column;
  Token(std::string t, std::string v, int l, int c)
      : type(std::move(t)), value(std::move(v)), line(l), column(c) {}
  std::string toString() const {
    return "Token(" + type + ", " + value + ", " + std::to_string(line) + ", " +
           std::to_string(column) + ")";
  }
};

// Lexer class placeholder.
class Lexer {
 public:
  Lexer(const std::string&) {}
  Token* nextToken() { return nullptr; }
  std::vector<Token*> tokenize() { return {}; }
};
