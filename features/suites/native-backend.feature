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
