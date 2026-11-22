Feature: GGML spatial storage graphs
  @VT-18 @build
  Scenario: Prism defines F32 zero and circular padding, reflected padding and signed four-axis roll addressing and Verdant execution matches the imported GGML CPU oracle
    Given asymmetric padding, signed crops, circular wrapping, reflection and signed shifts
    When vTensor executes valid spatial graphs through Prism and Verdant
    Then complete encoded output and padding match the imported CPU reference
    And invalid shapes, missing operands and undefined circular empty-source graphs fail before writes

  @VT-57 @build
  Scenario: Prism plans GGML rotary position encodings and their backward graphs and Verdant computes each rotation against the imported GGML CPU
    Given floating-point source rows with positions, optional frequency factors, modes and offsets
    When vTensor executes the forward or backward rotation through Verdant
    Then the 510 ROPE and 424 ROPE_BACK factory cases match the upstream evaluator

  @VT-58 @build
  Scenario: Prism plans GGML extended flash attention views and Verdant computes the masked attention output against the imported GGML CPU evaluator
    Given the extended attention cases from the pinned upstream factory
    When the host schedules them on vTensor
    Then all 5775 outputs agree with the independent GGML evaluator
