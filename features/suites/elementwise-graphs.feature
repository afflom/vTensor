Feature: elementwise-graphs

  @VT-13 @build
  Scenario: F32 arithmetic and unary graphs execute through Verdant with Prism-defined scalar semantics and broadcasting and match the imported GGML CPU reference
    Given strided F32 tensors, broadcast operands and exceptional encodings
    When the imported GGML CPU backend and vTensor execute the same graphs
    Then addition, subtraction, multiplication, division, square, square root, negation and absolute value match
    And malformed shapes and missing required operands are refused

  @VT-56 @build
  Scenario: Prism plans combined swapped and split GGML SWIGLU rows and Verdant evaluates the gated values across F32 F16 and BF16 against the imported GGML CPU
    Given combined or split rows with the admitted GGML floating storage types
    When the scheduler submits a SWIGLU node with its source order and views
    Then all 36 upstream factory cases agree with the independent CPU backend
