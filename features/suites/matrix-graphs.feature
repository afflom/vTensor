Feature: matrix-graphs

  @VT-12 @build
  Scenario: GGML F32 matrix-product graphs execute through Verdant across rectangular shapes, padded storage and batch broadcasting and match the imported GGML CPU reference
    Given rectangular four-dimensional F32 tensor operands with independent source and destination padding
    When the same graph is executed by the imported GGML CPU reference and vTensor
    Then every output matches and only Verdant performs production tensor computation
    And invalid shapes are refused before execution
    And every batch load kernel call and output store follows the certified Prism execution schedule

  @VT-53 @build
  Scenario: Prism admits Q8_0 by F32 GGML matrix graphs and Verdant executes their block products against the imported GGML CPU
    Given the pinned factory for Q8_0 weights and F32 activations
    When the upstream evaluator executes each standalone matrix product
    Then every selected case is supported and its output passes the upstream CPU comparison

  @VT-54 @build
  Scenario: Prism admits Q4_0 by F32 GGML matrix graphs and Verdant executes their block products against the imported GGML CPU
    Given the pinned factory for Q4_0 weights and F32 activations
    When the upstream evaluator executes each standalone matrix product
    Then every selected case is supported and its output passes the upstream CPU comparison
