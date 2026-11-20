Feature: GGML spatial storage graphs
  @VT-18 @build
  Scenario: Prism defines F32 zero and circular padding, reflected padding and signed four-axis roll addressing and Verdant execution matches the imported GGML CPU oracle
    Given asymmetric padding, signed crops, circular wrapping, reflection and signed shifts
    When vTensor executes valid spatial graphs through Prism and Verdant
    Then complete encoded output and padding match the imported CPU reference
    And invalid shapes, missing operands and undefined circular empty-source graphs fail before writes
