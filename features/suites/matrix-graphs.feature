Feature: matrix-graphs

  @VT-12 @build
  Scenario: GGML F32 matrix-product graphs execute through Verdant across rectangular shapes, padded storage and batch broadcasting and match the imported GGML CPU reference
    Given rectangular four-dimensional F32 tensor operands with independent source and destination padding
    When the same graph is executed by the imported GGML CPU reference and vTensor
    Then every output matches and only Verdant performs production tensor computation
    And invalid shapes are refused before execution
    And every batch load kernel call and output store follows the certified Prism execution schedule
