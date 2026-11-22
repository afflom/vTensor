Feature: native-backend

  @VT-11 @build
  Scenario: GGML discovers the Prism-defined vTensor device and its owned buffers preserve upstream allocation and byte-transfer behavior
    Given the pinned upstream GGML backend ABI and native frontend
    When an application discovers vTensor and allocates and transfers tensor bytes
    Then ABI layouts, tensor placement, and observed bytes agree with upstream
    And invalid allocation and byte ranges have modeled refusals
    And native oracle executables are reused after host-only orchestration changes
    And changes to the compiler recipe or imported evaluator invalidate that reuse

  @VT-52 @build
  Scenario: Prism selects graph nodes by the imported GGML compute flag and vTensor preserves inactive results across scheduled graph reuse
    Given a scheduled graph with an intermediate result and a dependent consumer
    When the application reuses the graph with the intermediate compute bit set or cleared
    Then both node outputs match the imported CPU and each compute node is assigned to the selected device

  @VT-60 @build
  Scenario: GGML logarithm sine cosine and exponential graphs execute through the Verdant scalar primitives across the imported factory storage and view cases
    Given the pinned upstream GGML evaluation factory and CPU reference
    When LOG, SIN, COS and EXP run on the vTensor backend
    Then all 39 factory outputs match the reference within the imported tolerances

  @VT-61 @build
  Scenario: GGML sigmoid and SiLU execute through Verdant in unary, multiplied and added graph compositions across all 122 imported factory cases
    Given the pinned GGML CPU reference and graph factory
    When vTensor evaluates sigmoid, SiLU and their add and multiply compositions
    Then the 122 results match the upstream evaluator

  @VT-63 @build
  Scenario: GGML attention with BF16 key and value rows uses Verdant conversion and contraction and matches the imported CPU with optional masks and sinks
    Given the pinned GGML BF16 attention factory with 64 channels and 113 keys
    When vTensor evaluates all four combinations of mask and sink presence
    Then each output matches the imported GGML CPU evaluator
