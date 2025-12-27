#pragma once
#include <memory>
#include <string>
#include <vector>

#include "rust/cxx.h"

struct BridgeToken {
  rust::String kind;
  rust::String value;
  int32_t line;
  int32_t column;

  rust::String get_kind() const { return kind; }
  rust::String get_value() const { return value; }
  int32_t get_line() const { return line; }
  int32_t get_column() const { return column; }
};

// Return a pointer to a heap-allocated vector
std::unique_ptr<std::vector<BridgeToken>> tokenize(rust::Str input);
