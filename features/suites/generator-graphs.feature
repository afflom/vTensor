Feature: Zero-source numerical graph generators
  @VT-37 @build
  Scenario: Prism models zero-source GGML ARANGE planning and Verdant executes every required imported upstream ARANGE case with atomic invalid-parameter refusals
    Given the pinned unchanged upstream backend operation test factory and CPU reference backend
    When vTensor executes every upstream ARANGE case through its Prism plan and Verdant primitives
    Then all ten required cases pass the imported comparison without unsupported or omitted cases
    And invalid parameters and output counts are refused before any output write
    And stale sources wrong binaries and altered execution bindings are rejected
