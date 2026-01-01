#include <stdexcept>
#include <string>
#include <vector>

using namespace std;

struct Token {
  string type;
  string value;
  int line;
  int column;

  Token(string t, string v, int l, int c)
      : type(t), value(v), line(l), column(c) {}

  string toString() const {
    return "Token(" + type + ", " + value + ", " + to_string(line) + ", " +
           to_string(column) + ")";
  }
};

class Lexer {
 private:
  string text;
  size_t pos;
  int line;
  int column;

  int matchDFA0(size_t startPos);
  int matchDFA1(size_t startPos);
  int matchDFA2(size_t startPos);
  int matchDFA3(size_t startPos);
  int matchDFA4(size_t startPos);
  int matchDFA5(size_t startPos);
  int matchDFA6(size_t startPos);
  int matchDFA7(size_t startPos);
  int matchDFA8(size_t startPos);
  int matchDFA9(size_t startPos);

 public:
  Lexer(const string& input) : text(input), pos(0), line(1), column(1) {}

  Token* nextToken();
  vector<Token*> tokenize();
};

Token* Lexer::nextToken() {
  // Skip whitespace characters
  while (pos < text.length() && (text[pos] == ' ' || text[pos] == '\t' ||
                                 text[pos] == '\n' || text[pos] == '\r')) {
    if (text[pos] == '\n') {
      line++;
      column = 1;
    } else {
      column++;
    }
    pos++;
  }

  if (pos >= text.length()) {
    return nullptr;
  }

  // Try matching each rule (in file order)
  int longestLength = 0;
  string matchedType;

  // Rule: PLUS
  int match0 = matchDFA0(pos);
  if (match0 > longestLength) {
    longestLength = match0;
    matchedType = "PLUS";
  }

  // Rule: MINUS
  int match1 = matchDFA1(pos);
  if (match1 > longestLength) {
    longestLength = match1;
    matchedType = "MINUS";
  }

  // Rule: MUL
  int match2 = matchDFA2(pos);
  if (match2 > longestLength) {
    longestLength = match2;
    matchedType = "MUL";
  }

  // Rule: DIV
  int match3 = matchDFA3(pos);
  if (match3 > longestLength) {
    longestLength = match3;
    matchedType = "DIV";
  }

  // Rule: (
  int match4 = matchDFA4(pos);
  if (match4 > longestLength) {
    longestLength = match4;
    matchedType = "(";
  }

  // Rule: )
  int match5 = matchDFA5(pos);
  if (match5 > longestLength) {
    longestLength = match5;
    matchedType = ")";
  }

  // Rule: NUMBER
  int match6 = matchDFA6(pos);
  if (match6 > longestLength) {
    longestLength = match6;
    matchedType = "NUMBER";
  }

  // Rule: VARIABLE
  int match7 = matchDFA7(pos);
  if (match7 > longestLength) {
    longestLength = match7;
    matchedType = "VARIABLE";
  }

  // Rule: WHITESPACE (IGNORE)
  int match8 = matchDFA8(pos);
  if (match8 > 0) {
    pos += match8;
    column += match8;
    return nextToken();  // Continue matching the next token
  }

  // Rule: COMMENT (IGNORE)
  int match9 = matchDFA9(pos);
  if (match9 > 0) {
    pos += match9;
    column += match9;
    return nextToken();  // Continue matching the next token
  }

  if (longestLength > 0) {
    string value = text.substr(pos, longestLength);
    Token* token = new Token(matchedType, value, line, column);
    pos += longestLength;
    column += longestLength;
    return token;
  }

  // Error: unexpected character
  throw runtime_error("Unexpected character '" + string(1, text[pos]) +
                      "' at line " + to_string(line) + ", column " +
                      to_string(column));
}

vector<Token*> Lexer::tokenize() {
  vector<Token*> tokens;
  while (pos < text.length()) {
    Token* token = nextToken();
    if (token) {
      tokens.push_back(token);
    } else {
      break;
    }
  }
  return tokens;
}

int Lexer::matchDFA0(size_t startPos) {
  int state = 1;
  size_t currentPos = startPos;
  size_t lastAcceptingPos = string::npos;

  // Accepting states
  bool accepting[2] = {true, false};

  // State transition table: transitions[state][char] = nextState
  int transitions[2][256];
  // Initialize to -1 (no transition)
  for (int i = 0; i < 2; i++) {
    for (int j = 0; j < 256; j++) {
      transitions[i][j] = -1;
    }
  }

  transitions[1][43] = 0;

  // Run DFA matching
  while (currentPos < text.length()) {
    unsigned char ch = static_cast<unsigned char>(text[currentPos]);
    int nextState = transitions[state][ch];
    if (nextState == -1) {
      break;
    }
    state = nextState;
    currentPos++;
    if (accepting[state]) {
      lastAcceptingPos = currentPos;
    }
  }

  if (lastAcceptingPos != string::npos) {
    return static_cast<int>(lastAcceptingPos - startPos);
  }
  return 0;
}

int Lexer::matchDFA1(size_t startPos) {
  int state = 1;
  size_t currentPos = startPos;
  size_t lastAcceptingPos = string::npos;

  // Accepting states
  bool accepting[2] = {true, false};

  // State transition table: transitions[state][char] = nextState
  int transitions[2][256];
  // Initialize to -1 (no transition)
  for (int i = 0; i < 2; i++) {
    for (int j = 0; j < 256; j++) {
      transitions[i][j] = -1;
    }
  }

  transitions[1][45] = 0;

  // Run DFA matching
  while (currentPos < text.length()) {
    unsigned char ch = static_cast<unsigned char>(text[currentPos]);
    int nextState = transitions[state][ch];
    if (nextState == -1) {
      break;
    }
    state = nextState;
    currentPos++;
    if (accepting[state]) {
      lastAcceptingPos = currentPos;
    }
  }

  if (lastAcceptingPos != string::npos) {
    return static_cast<int>(lastAcceptingPos - startPos);
  }
  return 0;
}

int Lexer::matchDFA2(size_t startPos) {
  int state = 1;
  size_t currentPos = startPos;
  size_t lastAcceptingPos = string::npos;

  // Accepting states
  bool accepting[2] = {true, false};

  // State transition table: transitions[state][char] = nextState
  int transitions[2][256];
  // Initialize to -1 (no transition)
  for (int i = 0; i < 2; i++) {
    for (int j = 0; j < 256; j++) {
      transitions[i][j] = -1;
    }
  }

  transitions[1][42] = 0;

  // Run DFA matching
  while (currentPos < text.length()) {
    unsigned char ch = static_cast<unsigned char>(text[currentPos]);
    int nextState = transitions[state][ch];
    if (nextState == -1) {
      break;
    }
    state = nextState;
    currentPos++;
    if (accepting[state]) {
      lastAcceptingPos = currentPos;
    }
  }

  if (lastAcceptingPos != string::npos) {
    return static_cast<int>(lastAcceptingPos - startPos);
  }
  return 0;
}

int Lexer::matchDFA3(size_t startPos) {
  int state = 1;
  size_t currentPos = startPos;
  size_t lastAcceptingPos = string::npos;

  // Accepting states
  bool accepting[2] = {true, false};

  // State transition table: transitions[state][char] = nextState
  int transitions[2][256];
  // Initialize to -1 (no transition)
  for (int i = 0; i < 2; i++) {
    for (int j = 0; j < 256; j++) {
      transitions[i][j] = -1;
    }
  }

  transitions[1][47] = 0;

  // Run DFA matching
  while (currentPos < text.length()) {
    unsigned char ch = static_cast<unsigned char>(text[currentPos]);
    int nextState = transitions[state][ch];
    if (nextState == -1) {
      break;
    }
    state = nextState;
    currentPos++;
    if (accepting[state]) {
      lastAcceptingPos = currentPos;
    }
  }

  if (lastAcceptingPos != string::npos) {
    return static_cast<int>(lastAcceptingPos - startPos);
  }
  return 0;
}

int Lexer::matchDFA4(size_t startPos) {
  int state = 1;
  size_t currentPos = startPos;
  size_t lastAcceptingPos = string::npos;

  // Accepting states
  bool accepting[2] = {true, false};

  // State transition table: transitions[state][char] = nextState
  int transitions[2][256];
  // Initialize to -1 (no transition)
  for (int i = 0; i < 2; i++) {
    for (int j = 0; j < 256; j++) {
      transitions[i][j] = -1;
    }
  }

  transitions[1][40] = 0;

  // Run DFA matching
  while (currentPos < text.length()) {
    unsigned char ch = static_cast<unsigned char>(text[currentPos]);
    int nextState = transitions[state][ch];
    if (nextState == -1) {
      break;
    }
    state = nextState;
    currentPos++;
    if (accepting[state]) {
      lastAcceptingPos = currentPos;
    }
  }

  if (lastAcceptingPos != string::npos) {
    return static_cast<int>(lastAcceptingPos - startPos);
  }
  return 0;
}

int Lexer::matchDFA5(size_t startPos) {
  int state = 1;
  size_t currentPos = startPos;
  size_t lastAcceptingPos = string::npos;

  // Accepting states
  bool accepting[2] = {true, false};

  // State transition table: transitions[state][char] = nextState
  int transitions[2][256];
  // Initialize to -1 (no transition)
  for (int i = 0; i < 2; i++) {
    for (int j = 0; j < 256; j++) {
      transitions[i][j] = -1;
    }
  }

  transitions[1][41] = 0;

  // Run DFA matching
  while (currentPos < text.length()) {
    unsigned char ch = static_cast<unsigned char>(text[currentPos]);
    int nextState = transitions[state][ch];
    if (nextState == -1) {
      break;
    }
    state = nextState;
    currentPos++;
    if (accepting[state]) {
      lastAcceptingPos = currentPos;
    }
  }

  if (lastAcceptingPos != string::npos) {
    return static_cast<int>(lastAcceptingPos - startPos);
  }
  return 0;
}

int Lexer::matchDFA6(size_t startPos) {
  int state = 1;
  size_t currentPos = startPos;
  size_t lastAcceptingPos = string::npos;

  // Accepting states
  bool accepting[2] = {true, false};

  // State transition table: transitions[state][char] = nextState
  int transitions[2][256];
  // Initialize to -1 (no transition)
  for (int i = 0; i < 2; i++) {
    for (int j = 0; j < 256; j++) {
      transitions[i][j] = -1;
    }
  }

  transitions[0][48] = 0;
  transitions[0][49] = 0;
  transitions[0][50] = 0;
  transitions[0][51] = 0;
  transitions[0][52] = 0;
  transitions[0][53] = 0;
  transitions[0][54] = 0;
  transitions[0][55] = 0;
  transitions[0][56] = 0;
  transitions[0][57] = 0;
  transitions[1][48] = 0;
  transitions[1][49] = 0;
  transitions[1][50] = 0;
  transitions[1][51] = 0;
  transitions[1][52] = 0;
  transitions[1][53] = 0;
  transitions[1][54] = 0;
  transitions[1][55] = 0;
  transitions[1][56] = 0;
  transitions[1][57] = 0;

  // Run DFA matching
  while (currentPos < text.length()) {
    unsigned char ch = static_cast<unsigned char>(text[currentPos]);
    int nextState = transitions[state][ch];
    if (nextState == -1) {
      break;
    }
    state = nextState;
    currentPos++;
    if (accepting[state]) {
      lastAcceptingPos = currentPos;
    }
  }

  if (lastAcceptingPos != string::npos) {
    return static_cast<int>(lastAcceptingPos - startPos);
  }
  return 0;
}

int Lexer::matchDFA7(size_t startPos) {
  int state = 1;
  size_t currentPos = startPos;
  size_t lastAcceptingPos = string::npos;

  // Accepting states
  bool accepting[2] = {true, false};

  // State transition table: transitions[state][char] = nextState
  int transitions[2][256];
  // Initialize to -1 (no transition)
  for (int i = 0; i < 2; i++) {
    for (int j = 0; j < 256; j++) {
      transitions[i][j] = -1;
    }
  }

  transitions[1][65] = 0;
  transitions[1][66] = 0;
  transitions[1][67] = 0;
  transitions[1][68] = 0;
  transitions[1][69] = 0;
  transitions[1][70] = 0;
  transitions[1][71] = 0;
  transitions[1][72] = 0;
  transitions[1][73] = 0;
  transitions[1][74] = 0;
  transitions[1][75] = 0;
  transitions[1][76] = 0;
  transitions[1][77] = 0;
  transitions[1][78] = 0;
  transitions[1][79] = 0;
  transitions[1][80] = 0;
  transitions[1][81] = 0;
  transitions[1][82] = 0;
  transitions[1][83] = 0;
  transitions[1][84] = 0;
  transitions[1][85] = 0;
  transitions[1][86] = 0;
  transitions[1][87] = 0;
  transitions[1][88] = 0;
  transitions[1][89] = 0;
  transitions[1][90] = 0;
  transitions[1][97] = 0;
  transitions[1][98] = 0;
  transitions[1][99] = 0;
  transitions[1][100] = 0;
  transitions[1][101] = 0;
  transitions[1][102] = 0;
  transitions[1][103] = 0;
  transitions[1][104] = 0;
  transitions[1][105] = 0;
  transitions[1][106] = 0;
  transitions[1][107] = 0;
  transitions[1][108] = 0;
  transitions[1][109] = 0;
  transitions[1][110] = 0;
  transitions[1][111] = 0;
  transitions[1][112] = 0;
  transitions[1][113] = 0;
  transitions[1][114] = 0;
  transitions[1][115] = 0;
  transitions[1][116] = 0;
  transitions[1][117] = 0;
  transitions[1][118] = 0;
  transitions[1][119] = 0;
  transitions[1][120] = 0;
  transitions[1][121] = 0;
  transitions[1][122] = 0;

  // Run DFA matching
  while (currentPos < text.length()) {
    unsigned char ch = static_cast<unsigned char>(text[currentPos]);
    int nextState = transitions[state][ch];
    if (nextState == -1) {
      break;
    }
    state = nextState;
    currentPos++;
    if (accepting[state]) {
      lastAcceptingPos = currentPos;
    }
  }

  if (lastAcceptingPos != string::npos) {
    return static_cast<int>(lastAcceptingPos - startPos);
  }
  return 0;
}

int Lexer::matchDFA8(size_t startPos) {
  int state = 1;
  size_t currentPos = startPos;
  size_t lastAcceptingPos = string::npos;

  // Accepting states
  bool accepting[2] = {true, false};

  // State transition table: transitions[state][char] = nextState
  int transitions[2][256];
  // Initialize to -1 (no transition)
  for (int i = 0; i < 2; i++) {
    for (int j = 0; j < 256; j++) {
      transitions[i][j] = -1;
    }
  }

  transitions[0][32] = 0;
  transitions[0][110] = 0;
  transitions[0][114] = 0;
  transitions[0][116] = 0;
  transitions[1][32] = 0;
  transitions[1][110] = 0;
  transitions[1][114] = 0;
  transitions[1][116] = 0;

  // Run DFA matching
  while (currentPos < text.length()) {
    unsigned char ch = static_cast<unsigned char>(text[currentPos]);
    int nextState = transitions[state][ch];
    if (nextState == -1) {
      break;
    }
    state = nextState;
    currentPos++;
    if (accepting[state]) {
      lastAcceptingPos = currentPos;
    }
  }

  if (lastAcceptingPos != string::npos) {
    return static_cast<int>(lastAcceptingPos - startPos);
  }
  return 0;
}

int Lexer::matchDFA9(size_t startPos) {
  int state = 1;
  size_t currentPos = startPos;
  size_t lastAcceptingPos = string::npos;

  // Accepting states
  bool accepting[2] = {true, false};

  // State transition table: transitions[state][char] = nextState
  int transitions[2][256];
  // Initialize to -1 (no transition)
  for (int i = 0; i < 2; i++) {
    for (int j = 0; j < 256; j++) {
      transitions[i][j] = -1;
    }
  }

  transitions[0][128] = 0;
  transitions[0][129] = 0;
  transitions[0][130] = 0;
  transitions[0][131] = 0;
  transitions[0][132] = 0;
  transitions[0][133] = 0;
  transitions[0][134] = 0;
  transitions[0][135] = 0;
  transitions[0][136] = 0;
  transitions[0][137] = 0;
  transitions[0][138] = 0;
  transitions[0][139] = 0;
  transitions[0][140] = 0;
  transitions[0][141] = 0;
  transitions[0][142] = 0;
  transitions[0][143] = 0;
  transitions[0][144] = 0;
  transitions[0][145] = 0;
  transitions[0][146] = 0;
  transitions[0][147] = 0;
  transitions[0][148] = 0;
  transitions[0][149] = 0;
  transitions[0][150] = 0;
  transitions[0][151] = 0;
  transitions[0][152] = 0;
  transitions[0][153] = 0;
  transitions[0][154] = 0;
  transitions[0][155] = 0;
  transitions[0][156] = 0;
  transitions[0][157] = 0;
  transitions[0][158] = 0;
  transitions[0][159] = 0;
  transitions[0][160] = 0;
  transitions[0][161] = 0;
  transitions[0][162] = 0;
  transitions[0][163] = 0;
  transitions[0][164] = 0;
  transitions[0][165] = 0;
  transitions[0][166] = 0;
  transitions[0][167] = 0;
  transitions[0][168] = 0;
  transitions[0][169] = 0;
  transitions[0][170] = 0;
  transitions[0][171] = 0;
  transitions[0][172] = 0;
  transitions[0][173] = 0;
  transitions[0][174] = 0;
  transitions[0][175] = 0;
  transitions[0][176] = 0;
  transitions[0][177] = 0;
  transitions[0][178] = 0;
  transitions[0][179] = 0;
  transitions[0][180] = 0;
  transitions[0][181] = 0;
  transitions[0][182] = 0;
  transitions[0][183] = 0;
  transitions[0][184] = 0;
  transitions[0][185] = 0;
  transitions[0][186] = 0;
  transitions[0][187] = 0;
  transitions[0][188] = 0;
  transitions[0][189] = 0;
  transitions[0][190] = 0;
  transitions[0][191] = 0;
  transitions[0][192] = 0;
  transitions[0][193] = 0;
  transitions[0][194] = 0;
  transitions[0][195] = 0;
  transitions[0][196] = 0;
  transitions[0][197] = 0;
  transitions[0][198] = 0;
  transitions[0][199] = 0;
  transitions[0][200] = 0;
  transitions[0][201] = 0;
  transitions[0][202] = 0;
  transitions[0][203] = 0;
  transitions[0][204] = 0;
  transitions[0][205] = 0;
  transitions[0][206] = 0;
  transitions[0][207] = 0;
  transitions[0][208] = 0;
  transitions[0][209] = 0;
  transitions[0][210] = 0;
  transitions[0][211] = 0;
  transitions[0][212] = 0;
  transitions[0][213] = 0;
  transitions[0][214] = 0;
  transitions[0][215] = 0;
  transitions[0][216] = 0;
  transitions[0][217] = 0;
  transitions[0][218] = 0;
  transitions[0][219] = 0;
  transitions[0][220] = 0;
  transitions[0][221] = 0;
  transitions[0][222] = 0;
  transitions[0][223] = 0;
  transitions[0][224] = 0;
  transitions[0][225] = 0;
  transitions[0][226] = 0;
  transitions[0][227] = 0;
  transitions[0][228] = 0;
  transitions[0][229] = 0;
  transitions[0][230] = 0;
  transitions[0][231] = 0;
  transitions[0][232] = 0;
  transitions[0][233] = 0;
  transitions[0][234] = 0;
  transitions[0][235] = 0;
  transitions[0][236] = 0;
  transitions[0][237] = 0;
  transitions[0][238] = 0;
  transitions[0][239] = 0;
  transitions[0][240] = 0;
  transitions[0][241] = 0;
  transitions[0][242] = 0;
  transitions[0][243] = 0;
  transitions[0][244] = 0;
  transitions[0][245] = 0;
  transitions[0][246] = 0;
  transitions[0][247] = 0;
  transitions[0][248] = 0;
  transitions[0][249] = 0;
  transitions[0][250] = 0;
  transitions[0][251] = 0;
  transitions[0][252] = 0;
  transitions[0][253] = 0;
  transitions[0][254] = 0;
  transitions[0][255] = 0;
  transitions[0][1] = 0;
  transitions[0][2] = 0;
  transitions[0][3] = 0;
  transitions[0][4] = 0;
  transitions[0][5] = 0;
  transitions[0][6] = 0;
  transitions[0][7] = 0;
  transitions[0][8] = 0;
  transitions[0][9] = 0;
  transitions[0][11] = 0;
  transitions[0][12] = 0;
  transitions[0][13] = 0;
  transitions[0][14] = 0;
  transitions[0][15] = 0;
  transitions[0][16] = 0;
  transitions[0][17] = 0;
  transitions[0][18] = 0;
  transitions[0][19] = 0;
  transitions[0][20] = 0;
  transitions[0][21] = 0;
  transitions[0][22] = 0;
  transitions[0][23] = 0;
  transitions[0][24] = 0;
  transitions[0][25] = 0;
  transitions[0][26] = 0;
  transitions[0][27] = 0;
  transitions[0][28] = 0;
  transitions[0][29] = 0;
  transitions[0][30] = 0;
  transitions[0][31] = 0;
  transitions[0][32] = 0;
  transitions[0][33] = 0;
  transitions[0][34] = 0;
  transitions[0][35] = 0;
  transitions[0][36] = 0;
  transitions[0][37] = 0;
  transitions[0][38] = 0;
  transitions[0][39] = 0;
  transitions[0][40] = 0;
  transitions[0][41] = 0;
  transitions[0][42] = 0;
  transitions[0][43] = 0;
  transitions[0][44] = 0;
  transitions[0][45] = 0;
  transitions[0][46] = 0;
  transitions[0][47] = 0;
  transitions[0][48] = 0;
  transitions[0][49] = 0;
  transitions[0][50] = 0;
  transitions[0][51] = 0;
  transitions[0][52] = 0;
  transitions[0][53] = 0;
  transitions[0][54] = 0;
  transitions[0][55] = 0;
  transitions[0][56] = 0;
  transitions[0][57] = 0;
  transitions[0][58] = 0;
  transitions[0][59] = 0;
  transitions[0][60] = 0;
  transitions[0][61] = 0;
  transitions[0][62] = 0;
  transitions[0][63] = 0;
  transitions[0][64] = 0;
  transitions[0][65] = 0;
  transitions[0][66] = 0;
  transitions[0][67] = 0;
  transitions[0][68] = 0;
  transitions[0][69] = 0;
  transitions[0][70] = 0;
  transitions[0][71] = 0;
  transitions[0][72] = 0;
  transitions[0][73] = 0;
  transitions[0][74] = 0;
  transitions[0][75] = 0;
  transitions[0][76] = 0;
  transitions[0][77] = 0;
  transitions[0][78] = 0;
  transitions[0][79] = 0;
  transitions[0][80] = 0;
  transitions[0][81] = 0;
  transitions[0][82] = 0;
  transitions[0][83] = 0;
  transitions[0][84] = 0;
  transitions[0][85] = 0;
  transitions[0][86] = 0;
  transitions[0][87] = 0;
  transitions[0][88] = 0;
  transitions[0][89] = 0;
  transitions[0][90] = 0;
  transitions[0][91] = 0;
  transitions[0][92] = 0;
  transitions[0][93] = 0;
  transitions[0][94] = 0;
  transitions[0][95] = 0;
  transitions[0][96] = 0;
  transitions[0][97] = 0;
  transitions[0][98] = 0;
  transitions[0][99] = 0;
  transitions[0][100] = 0;
  transitions[0][101] = 0;
  transitions[0][102] = 0;
  transitions[0][103] = 0;
  transitions[0][104] = 0;
  transitions[0][105] = 0;
  transitions[0][106] = 0;
  transitions[0][107] = 0;
  transitions[0][108] = 0;
  transitions[0][109] = 0;
  transitions[0][110] = 0;
  transitions[0][111] = 0;
  transitions[0][112] = 0;
  transitions[0][113] = 0;
  transitions[0][114] = 0;
  transitions[0][115] = 0;
  transitions[0][116] = 0;
  transitions[0][117] = 0;
  transitions[0][118] = 0;
  transitions[0][119] = 0;
  transitions[0][120] = 0;
  transitions[0][121] = 0;
  transitions[0][122] = 0;
  transitions[0][123] = 0;
  transitions[0][124] = 0;
  transitions[0][125] = 0;
  transitions[0][126] = 0;
  transitions[0][127] = 0;
  transitions[1][35] = 0;

  // Run DFA matching
  while (currentPos < text.length()) {
    unsigned char ch = static_cast<unsigned char>(text[currentPos]);
    int nextState = transitions[state][ch];
    if (nextState == -1) {
      break;
    }
    state = nextState;
    currentPos++;
    if (accepting[state]) {
      lastAcceptingPos = currentPos;
    }
  }

  if (lastAcceptingPos != string::npos) {
    return static_cast<int>(lastAcceptingPos - startPos);
  }
  return 0;
}
