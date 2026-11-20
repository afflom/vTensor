Feature: elementwise-graphs

  @VT-13 @build
  Scenario: F32 arithmetic and unary graphs execute through Verdant with Prism-defined scalar semantics and broadcasting and match the imported GGML CPU reference
    Given strided F32 tensors, broadcast operands and exceptional encodings
    When the imported GGML CPU backend and vTensor execute the same graphs
    Then addition, subtraction, multiplication, division, square, square root, negation and absolute value match
    And malformed shapes and missing required operands are refused
